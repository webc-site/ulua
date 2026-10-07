#!/usr/bin/env -S bun --loader .sh:js -e import(process.argv[1])

import fs from "node:fs";
import path from "node:path";
import yargs from "yargs";
import { hideBin } from "yargs/helpers";
import { $, cd } from "zx";

$.verbose = true;

const root_dir = import.meta.dirname,
  raw_argv_li = hideBin(process.argv),
  env_target_base =
    process.env.ULUA_TARGET_BASE ?? process.env.CARGO_TARGET_DIR ?? "/tmp",
  target_base = env_target_base.includes("/ulua")
    ? path.dirname(env_target_base)
    : env_target_base,
  parsed = yargs(raw_argv_li)
    .scriptName("./test.sh")
    .usage("用法: $0 [选项] [nextest参数...]")
    .parserConfiguration({
      "boolean-negation": false,
    })
    .option("package", {
      alias: "p",
      type: "string",
      describe: "指定运行测试的包名（如 ulua-vm, ulua-conformance）",
    })
    .option("workspace", {
      type: "boolean",
      describe: "运行整个 workspace 所有包的测试（默认）",
    })
    .option("status-level", {
      type: "string",
      describe:
        "显示测试状态的详细级别（默认 slow：仅输出失败与慢测，慢测阈值 1s；可选 fail, all 等）",
    })
    .option("analyze", {
      type: "boolean",
      describe: "运行测试并输出耗时分析报告（导出耗时 JSON 到 /tmp/ulua_test_durations.json）",
    })
    .option("dump-json", {
      type: "string",
      describe: "导出测试耗时 JSON 到指定路径",
    })
    .example("$0", "运行 workspace 全部测试（默认仅输出失败与 1s 慢测）")
    .example("$0 --status-level pass", "输出每个测试及其耗时（覆盖默认 slow）")
    .example("$0 --analyze", "运行测试并分析瓶颈，导出 JSON 到 /tmp")
    .example("$0 -p ulua-vm", "仅运行指定包测试")
    .example("$0 -p ulua-conformance", "运行 conformance 包（包含 JIT 一致性测试）")
    .example("$0 -- -E 'test(conformance)'", "透传 nextest 过滤器参数（或直接传 -E）")
    .example("$0 --status-level fail", "仅在测试失败时输出状态")
    .help()
    .alias("h", "help")
    .version(false)
    .parseSync(),
  pkg_li = Array.isArray(parsed.package)
    ? parsed.package
    : parsed.package
      ? [parsed.package]
      : [],
  has_pkg = pkg_li.length > 0,
  has_workspace = Boolean(parsed.workspace);

// 跑批降噪（对齐 wedb/wedb/test.sh 口径）：红点与慢测即时输出，
// 进度条与绿流不入终端；slow 状态级增量含 retry/fail
// （fail 会挡掉 SLOW 行与终局慢测摘要）。慢测阈值 1s 见 .config/nextest.toml
const noise_li = [
  "--failure-output",
  "immediate",
  "--status-level",
  "slow",
  "--final-status-level",
  "slow",
  "--show-progress",
  "none",
];

cd(root_dir);

const runTest = async (sub_dir, extra_li, env = {}) => {
  const target_dir = path.join(target_base, sub_dir);
  fs.mkdirSync(target_dir, { recursive: true });
  // 外部已显式给出同名参数时不再追加该默认降噪参数
  const noise_extra_li = [];
  for (let i = 0; i < noise_li.length; i += 2) {
    const flag = noise_li[i];
    const value = noise_li[i + 1];
    const overridden = extra_li.some(
      (a) => a === flag || a.startsWith(`${flag}=`),
    );
    if (!overridden) noise_extra_li.push(flag, value);
  }
  await $({
    env: {
      ...process.env,
      CARGO_TARGET_DIR: target_dir,
      ...env,
    },
  })`cargo nextest run --target-dir ${target_dir} ${noise_extra_li} ${extra_li}`;
},
main = async () => {
  if (parsed.analyze || parsed["dump-json"]) {
    const analyze_args = ["scripts/test_analyze.py"];
    if (has_pkg) {
      for (const p of pkg_li) analyze_args.push("-p", p);
    }
    if (parsed["dump-json"] && typeof parsed["dump-json"] === "string") {
      analyze_args.push("-o", parsed["dump-json"]);
    }
    if (parsed.filter) {
      analyze_args.push("-E", parsed.filter);
    }
    await $`python3 ${analyze_args}`;
    return;
  }

  // 1. 运行测试集（解释执行模式）
  // 若外部显式指定了 -p/--package 或已包含 --workspace，则不再重复追加 --workspace
  const step1_arg_li =
    has_pkg || has_workspace ? raw_argv_li : ["--workspace", ...raw_argv_li];

  await runTest("ulua", step1_arg_li);

  // 2. JIT 机器码生成的一致性测试
  // 仅当未指定非 ulua-conformance 的包时才执行，避免参数冲突与找不到目标报错
  const should_run_jit = !has_pkg || pkg_li.includes("ulua-conformance");
  if (should_run_jit) {
    const jit_arg_li = [];
    for (let i = 0; i < raw_argv_li.length; ++i) {
      const arg = raw_argv_li[i];
      if (arg === "-p" || arg === "--package") {
        if (
          i + 1 < raw_argv_li.length &&
          raw_argv_li[i + 1] === "ulua-conformance"
        ) {
          ++i;
          continue;
        }
      } else if (
        arg === "--package=ulua-conformance" ||
        arg === "-p=ulua-conformance"
      ) {
        continue;
      } else if (arg === "--workspace") {
        continue;
      } else if (arg === "--test") {
        if (
          i + 1 < raw_argv_li.length &&
          raw_argv_li[i + 1] === "conformance"
        ) {
          ++i;
          continue;
        }
      } else if (arg === "--test=conformance") {
        continue;
      }
      jit_arg_li.push(arg);
    }

    await runTest(
      "ulua",
      ["-p", "ulua-conformance", "--test", "conformance", ...jit_arg_li],
      { LUAU_CODEGEN: "1" },
    );
  }
};

await main();
