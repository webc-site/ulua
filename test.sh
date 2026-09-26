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
    .example("$0", "运行 workspace 全部测试与 JIT 一致性测试")
    .example("$0 -p ulua-vm", "仅运行指定包测试")
    .example("$0 -p ulua-conformance", "运行 conformance 包（包含 JIT 一致性测试）")
    .example("$0 -- -E 'test(conformance)'", "透传 nextest 过滤器参数（或直接传 -E）")
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

cd(root_dir);

const runTest = async (sub_dir, extra_li, env = {}) => {
  const target_dir = path.join(target_base, sub_dir);
  fs.mkdirSync(target_dir, { recursive: true });
  await $({
    env: {
      ...process.env,
      CARGO_TARGET_DIR: target_dir,
      ...env,
    },
  })`cargo nextest run --target-dir ${target_dir} --status-level fail ${extra_li}`;
},
main = async () => {
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
      "ulua-jit",
      ["-p", "ulua-conformance", "--test", "conformance", ...jit_arg_li],
      { LUAU_CODEGEN: "1" },
    );
  }
};

await main();
