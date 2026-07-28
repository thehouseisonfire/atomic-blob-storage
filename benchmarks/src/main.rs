#![expect(clippy::cast_precision_loss)]
#![expect(clippy::too_many_lines)]

use std::alloc::{GlobalAlloc, Layout, System};
use std::collections::BTreeMap;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use clap::{Args, Parser, Subcommand};
use serde::Serialize;
use serde_json::Value;

mod persistence;

struct CountingAllocator;

static ALLOCATION_COUNT: AtomicU64 = AtomicU64::new(0);
static ALLOCATED_BYTES: AtomicU64 = AtomicU64::new(0);

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATION_COUNT.fetch_add(1, Ordering::Relaxed);
        ALLOCATED_BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
        // SAFETY: This forwards the allocation unchanged to the system allocator.
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        // SAFETY: This forwards the matching deallocation to the system allocator.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        ALLOCATION_COUNT.fetch_add(1, Ordering::Relaxed);
        ALLOCATED_BYTES.fetch_add(new_size as u64, Ordering::Relaxed);
        // SAFETY: This forwards the matching reallocation to the system allocator.
        unsafe { System.realloc(pointer, layout, new_size) }
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: CountingAllocator = CountingAllocator;

#[derive(Parser, Debug)]
#[command(name = "atomic-blob-store-bench")]
#[command(about = "Maintained benchmark harness for atomic-blob-store")]
struct Cli {
    #[command(subcommand)]
    command: CommandGroup,
}

#[derive(Subcommand, Debug)]
enum CommandGroup {
    Persistence {
        #[command(subcommand)]
        command: persistence::PersistenceCommand,
    },
}

#[derive(Args, Debug, Clone)]
struct CommonArgs {
    #[arg(long)]
    run_id: Option<String>,
}

#[derive(Serialize)]
struct BenchOutput {
    schema_version: u32,
    run_id: String,
    scenario: String,
    started_at_unix: u64,
    finished_at_unix: u64,
    config: Value,
    metrics: BTreeMap<String, f64>,
    samples: BTreeMap<String, Vec<f64>>,
    environment: Environment,
}

#[derive(Serialize)]
struct Environment {
    git_commit: Option<String>,
    git_dirty: bool,
    build_profile: String,
    rustc: Option<String>,
    target: String,
    os: String,
    arch: String,
    cpu_count: usize,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        CommandGroup::Persistence { command } => persistence::run(command).await,
    }
}

fn run_id(input: Option<&str>, prefix: &str) -> String {
    input.map_or_else(
        || format!("{prefix}-{}-{}", unix_secs(), rand::random::<u32>()),
        ToOwned::to_owned,
    )
}

fn unix_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_secs())
}

fn environment() -> Environment {
    Environment {
        git_commit: command_stdout("git", &["rev-parse", "HEAD"]),
        git_dirty: command_stdout("git", &["status", "--porcelain"])
            .is_some_and(|status| !status.is_empty()),
        build_profile: if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        }
        .to_owned(),
        rustc: command_stdout("rustc", &["--version"]),
        target: format!("{}-{}", std::env::consts::ARCH, std::env::consts::OS),
        os: std::env::consts::OS.to_owned(),
        arch: std::env::consts::ARCH.to_owned(),
        cpu_count: std::thread::available_parallelism().map_or(1, usize::from),
    }
}

fn reset_allocation_counters() {
    ALLOCATION_COUNT.store(0, Ordering::SeqCst);
    ALLOCATED_BYTES.store(0, Ordering::SeqCst);
}

fn allocation_counters() -> (u64, u64) {
    (
        ALLOCATION_COUNT.load(Ordering::SeqCst),
        ALLOCATED_BYTES.load(Ordering::SeqCst),
    )
}

fn command_stdout(program: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(program).args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn print_output(output: &BenchOutput) -> anyhow::Result<()> {
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}
