#!/usr/bin/env python3
"""
Test Duration Analyzer for ulua

Runs tests using cargo nextest with JSON output, exports test execution times
to /tmp/ulua_test_durations.json, and prints a formatted analysis of slow tests.

Usage:
    python3 sh/test_analyze.py [options]
    ./sh/test_analyze.py --package ulua-conformance
    ./sh/test_analyze.py --top 40
    ./sh/test_analyze.py --cached  # analyze existing /tmp/ulua_test_durations.json
"""

import argparse
import json
import os
import subprocess
import sys
import time
from collections import defaultdict
from pathlib import Path

DEFAULT_OUTPUT_JSON = "/tmp/ulua_test_durations.json"
TARGET_DIR = os.environ.get("ULUA_TARGET_BASE", os.environ.get("CARGO_TARGET_DIR", "/tmp"))
if "/ulua" not in TARGET_DIR:
    TARGET_DIR = os.path.join(TARGET_DIR, "ulua")


def run_nextest(extra_args, env_vars=None):
    """Run cargo nextest with libtest-json format and collect raw json lines."""
    env = os.environ.copy()
    env["NEXTEST_EXPERIMENTAL_LIBTEST_JSON"] = "1"
    env["CARGO_TARGET_DIR"] = TARGET_DIR
    if env_vars:
        env.update(env_vars)

    cmd = [
        "cargo",
        "nextest",
        "run",
        "--target-dir",
        TARGET_DIR,
        "--message-format",
        "libtest-json",
    ] + extra_args

    print(f"==> Running: {' '.join(cmd)}")
    start = time.time()
    proc = subprocess.Popen(
        cmd,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        env=env,
    )
    stdout, stderr = proc.communicate()
    elapsed = time.time() - start

    if proc.returncode != 0:
        # Nextest might fail on some tests, but we still want to parse results if any
        print(f"Warning: nextest exited with code {proc.returncode}")
        if stderr and proc.returncode != 0:
            print(f"Stderr: {stderr[:500]}")

    results = []
    for line in stdout.splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            data = json.loads(line)
            if data.get("type") == "test" and data.get("event") in ("ok", "failed"):
                results.append(data)
        except Exception:
            pass

    print(f"==> Completed in {elapsed:.2f}s ({len(results)} tests collected)")
    return results


def parse_and_export(records, output_file):
    """Sort and export test durations to json."""
    tests = []
    for r in records:
        tests.append({
            "name": r.get("name", ""),
            "status": r.get("event", "ok"),
            "exec_time": r.get("exec_time", 0.0),
            "suite": r.get("suite_type", "default"),
        })

    tests.sort(key=lambda x: x["exec_time"], reverse=True)

    with open(output_file, "w") as f:
        json.dump(tests, f, indent=2)

    print(f"==> Exported {len(tests)} test records to {output_file}")
    return tests


def print_analysis(tests, top_n=30):
    if not tests:
        print("No test results found.")
        return

    print("\n" + "=" * 80)
    print(f" TOP {min(top_n, len(tests))} SLOWEST TESTS")
    print("=" * 80)
    for i, t in enumerate(tests[:top_n]):
        status_flag = " [FAIL]" if t["status"] != "ok" else ""
        suite = f"[{t['suite']}] " if t.get("suite") else ""
        print(f"  {i+1:2d}. {t['exec_time']:7.3f}s  {suite}{t['name']}{status_flag}")

    # Duration Distribution
    brackets = [
        (10.0, float("inf")),
        (5.0, 10.0),
        (2.0, 5.0),
        (1.0, 2.0),
        (0.5, 1.0),
        (0.1, 0.5),
        (0.0, 0.1),
    ]
    print("\n" + "-" * 80)
    print(" DURATION DISTRIBUTION")
    print("-" * 80)
    for low, high in brackets:
        matching = [t for t in tests if low <= t["exec_time"] < high]
        total_time = sum(t["exec_time"] for t in matching)
        high_str = f"{high:4.1f}s" if high != float("inf") else " inf"
        print(f"  {low:4.1f}s - {high_str}: {len(matching):5d} tests ({total_time:7.2f}s total)")

    # Package breakdown
    pkg_times = defaultdict(float)
    pkg_counts = defaultdict(int)
    for t in tests:
        name = t["name"]
        pkg = name.split("::")[0] if "::" in name else "other"
        pkg_times[pkg] += t["exec_time"]
        pkg_counts[pkg] += 1

    print("\n" + "-" * 80)
    print(" BY CRATE / PACKAGE")
    print("-" * 80)
    sorted_pkgs = sorted(pkg_times.items(), key=lambda x: x[1], reverse=True)
    for pkg, total_t in sorted_pkgs:
        cnt = pkg_counts[pkg]
        avg = total_t / cnt if cnt else 0.0
        print(f"  {pkg:30s}: {cnt:5d} tests, {total_t:7.2f}s total (avg: {avg*1000:5.1f}ms)")

    total_exec = sum(t["exec_time"] for t in tests)
    print("\n" + "=" * 80)
    print(f" TOTAL TESTS: {len(tests)} | CUMULATIVE TIME: {total_exec:.2f}s")
    print("=" * 80 + "\n")


def main():
    parser = argparse.ArgumentParser(description="Analyze test durations in ulua")
    parser.add_argument("--package", "-p", help="Filter by package (e.g., ulua-conformance, ulua-vm)")
    parser.add_argument("--filter", "-E", help="Filter expression for nextest")
    parser.add_argument("--top", type=int, default=30, help="Number of slowest tests to show")
    parser.add_argument("--output", "-o", default=DEFAULT_OUTPUT_JSON, help="Output JSON path")
    parser.add_argument("--cached", action="store_true", help="Analyze existing JSON file without running tests")
    parser.add_argument("--skip-jit", action="store_true", help="Skip running JIT conformance tests")
    args = parser.parse_args()

    if args.cached:
        if not os.path.exists(args.output):
            print(f"Error: {args.output} does not exist. Run without --cached first.")
            sys.exit(1)
        with open(args.output) as f:
            tests = json.load(f)
        print_analysis(tests, top_n=args.top)
        return

    extra_args = []
    if args.package:
        extra_args.extend(["-p", args.package])
    else:
        extra_args.append("--workspace")

    if args.filter:
        extra_args.extend(["-E", args.filter])

    all_records = []
    # Step 1: Run default/interpreter tests
    records_step1 = run_nextest(extra_args)
    for r in records_step1:
        r["suite_type"] = "vm"
    all_records.extend(records_step1)

    # Step 2: Run JIT conformance tests if applicable
    should_run_jit = not args.skip_jit and (not args.package or args.package == "ulua-conformance")
    if should_run_jit:
        jit_args = ["-p", "ulua-conformance", "--test", "conformance"]
        if args.filter:
            jit_args.extend(["-E", args.filter])
        records_step2 = run_nextest(jit_args, env_vars={"LUAU_CODEGEN": "1"})
        for r in records_step2:
            r["suite_type"] = "jit"
        all_records.extend(records_step2)

    tests = parse_and_export(all_records, args.output)
    print_analysis(tests, top_n=args.top)


if __name__ == "__main__":
    main()
