#!/usr/bin/env -S bun --loader .sh:js -e import(process.argv[1])

import fs from "node:fs";
import path from "node:path";
import yargs from "yargs";
import { hideBin } from "yargs/helpers";
import { $, cd } from "zx";

$.verbose = true;

const root_dir = import.meta.dirname,
  raw_argv_li = hideBin(process.argv);

cd(root_dir);

let target_base =
  process.env.ULUA_TARGET_BASE || process.env.CARGO_TARGET_DIR || "/tmp";
if (target_base.includes("/ulua")) {
  target_base = path.dirname(target_base);
}

const parsed = yargs(raw_argv_li)
  .parserConfiguration({
    "boolean-negation": false,
  })
  .option("package", {
    alias: "p",
    type: "string",
  })
  .option("workspace", {
    type: "boolean",
  })
  .help(false)
  .version(false)
  .parseSync();

const pkg_li = Array.isArray(parsed.package)
    ? parsed.package
    : parsed.package
      ? [parsed.package]
      : [],
  has_pkg = pkg_li.length > 0,
  has_workspace = Boolean(parsed.workspace);

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
};

const main = async () => {
  try {
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
  } catch (err) {
    process.exit(err.exitCode ?? 1);
  }
};

await main();
