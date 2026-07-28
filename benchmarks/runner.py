#!/usr/bin/env python3
"""Run repeatable atomic-blob-store benchmark scenarios."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import subprocess
import tomllib
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
COMMANDS = {
    "envelope",
    "file-store",
    "coordination",
    "lifecycle",
    "maintenance",
    "backpressure",
}


def load_scenario(name: str) -> tuple[Path, dict[str, Any]]:
    path = Path(name)
    if not path.suffix:
        path = ROOT / "benchmarks" / "scenarios" / f"{name}.toml"
    elif not path.is_absolute():
        path = ROOT / path
    with path.open("rb") as source:
        scenario = tomllib.load(source)
    validate_scenario(path, scenario)
    return path, scenario


def validate_scenario(path: Path, scenario: dict[str, Any]) -> None:
    for field in ("name", "group", "command", "description", "primary_metric"):
        if not isinstance(scenario.get(field), str) or not scenario[field]:
            raise RuntimeError(f"{path}: missing string field '{field}'")
    if scenario["group"] != "persistence" or scenario["command"] not in COMMANDS:
        raise RuntimeError(f"{path}: unsupported atomic-blob-store scenario")
    if not isinstance(scenario.get("higher_is_better"), bool):
        raise RuntimeError(f"{path}: missing boolean field 'higher_is_better'")
    if scenario.get("requires_broker") is not False:
        raise RuntimeError(f"{path}: standalone scenarios cannot require a broker")
    if "args" in scenario and not isinstance(scenario["args"], dict):
        raise RuntimeError(f"{path}: args must be a table")


def scenario_command(scenario: dict[str, Any], run_id: str, release: bool) -> list[str]:
    command = ["cargo", "run", "--locked"]
    if release:
        command.append("--release")
    command.extend(
        [
            "-p",
            "atomic-blob-store-benchmarks",
            "--bin",
            "atomic-blob-store-bench",
            "--",
            "persistence",
            scenario["command"],
        ]
    )
    arguments = dict(scenario.get("args", {}))
    arguments["run-id"] = run_id
    for key in sorted(arguments):
        flag = f"--{key.replace('_', '-')}"
        value = arguments[key]
        if isinstance(value, bool):
            if value:
                command.append(flag)
        else:
            command.extend([flag, str(value)])
    return command


def read_result(output: str) -> dict[str, Any]:
    start = output.find("{")
    if start < 0:
        raise RuntimeError("benchmark did not emit JSON")
    result = json.loads(output[start:])
    if not isinstance(result, dict):
        raise RuntimeError("benchmark result is not an object")
    return result


def run(name: str, runs: int, release: bool, output_dir: Path | None) -> None:
    _, scenario = load_scenario(name)
    stamp = dt.datetime.now(dt.UTC).strftime("%Y%m%d-%H%M%SZ")
    destination = output_dir or ROOT / "benchmarks" / "results" / "runs" / stamp
    destination.mkdir(parents=True, exist_ok=True)
    for index in range(runs):
        run_id = f"{scenario['name']}-{stamp}-{index}"
        process = subprocess.run(
            scenario_command(scenario, run_id, release),
            cwd=ROOT,
            check=True,
            capture_output=True,
            text=True,
        )
        result = read_result(process.stdout)
        if scenario["primary_metric"] not in result.get("metrics", {}):
            raise RuntimeError(
                f"result omitted primary metric {scenario['primary_metric']!r}"
            )
        (destination / f"{index:03}.json").write_text(
            json.dumps(result, indent=2) + "\n", encoding="utf-8"
        )
    print(destination)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("scenario")
    parser.add_argument("--runs", type=int, default=1)
    parser.add_argument("--dev", action="store_true")
    parser.add_argument("--output-dir", type=Path)
    args = parser.parse_args()
    if args.runs < 1:
        parser.error("--runs must be positive")
    run(args.scenario, args.runs, not args.dev, args.output_dir)


if __name__ == "__main__":
    main()
