import importlib.util
import unittest
from pathlib import Path

MODULE_PATH = Path(__file__).with_name("runner.py")
SPEC = importlib.util.spec_from_file_location("atomic_blob_benchmark_runner", MODULE_PATH)
assert SPEC and SPEC.loader
runner = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(runner)


class RunnerTests(unittest.TestCase):
    def test_catalog_scenarios_validate(self):
        scenarios = sorted((runner.ROOT / "benchmarks" / "scenarios").glob("*.toml"))
        self.assertEqual(len(scenarios), 3)
        for path in scenarios:
            _, scenario = runner.load_scenario(str(path))
            self.assertIn(scenario["command"], runner.COMMANDS)

    def test_command_routes_to_standalone_benchmark(self):
        _, scenario = runner.load_scenario("persistence-envelope-1mib")
        command = runner.scenario_command(scenario, "run-1", False)
        self.assertIn("atomic-blob-store-benchmarks", command)
        self.assertIn("atomic-blob-store-bench", command)
        self.assertNotIn("--release", command)

    def test_result_requires_json(self):
        with self.assertRaisesRegex(RuntimeError, "did not emit JSON"):
            runner.read_result("progress only")


if __name__ == "__main__":
    unittest.main()
