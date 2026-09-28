#!/usr/bin/env -S bun

/**
 * ulua 性能基准单路采样器 (Bun 原生运行环境)
 *
 * 通过 `BENCH_MODE` 环境变量决定这一趟跑什么：
 *   interp   — `cargo bench`（不加 --features jit），只采 `vm_execution` /
 *              `high_level_lua` / `compile_speed` 三组；写 `result.json`（mode=interp）。
 *   jit      — `cargo bench --features jit`，只采 `vm_execution_jit` /
 *              `high_level_lua_jit` 两组；写 `result.json`（mode=jit）。
 *   combined — 本地默认；`cargo bench --features jit` 一把梭 5 组，直接更新
 *              `history.json` 并打印双路对比。CI 不再走这条路径。
 *
 * 采样输出统一写入 `benchmarks/regression/result.json`（combined 模式额外写
 * history.json）。CI 的两个 bench job 各自把 result.json 上传成独立 artifact，
 * 由 `publish.js` 汇总进 history 与 Job Summary。
 *
 * 环境变量：
 *   BENCH_MODE     interp / jit / combined，默认 combined
 *   BENCH_SAMPLES  每个 bench 的采样数，默认 10
 *   BENCH_RESULT_PATH  result.json 落盘位置，默认同目录
 */

import { dirname, join } from "node:path";

const HERE = import.meta.dirname,
  REPO_ROOT = dirname(dirname(HERE)),
  HISTORY_PATH = join(HERE, "history.json"),
  MODE = (process.env.BENCH_MODE ?? "combined").toLowerCase(),
  SAMPLE_COUNT = Number.parseInt(process.env.BENCH_SAMPLES ?? "10", 10) || 10,
  RESULT_PATH = process.env.BENCH_RESULT_PATH ?? join(HERE, "result.json"),
  // 组检测按顺序匹配、首个命中即生效；jit 变体必须排在其基础组之前，
  // 否则 "vm_execution_jit" 会被 "vm_execution" 前缀误吞。
  GROUP_DEFS = [
    { token: "vm_execution_jit", key: "vm_jit" },
    { token: "vm_execution", key: "vm" },
    { token: "high_level_lua_jit", key: "high_level_jit" },
    { token: "high_level_lua", key: "high_level" },
    { token: "compile_speed", key: "compile" },
  ],
  // 每种 mode 只关心自己那几组；其余即使被解析到也丢弃，避免误合并。
  MODE_KEYS = {
    interp: ["vm", "high_level", "compile"],
    jit: ["vm_jit", "high_level_jit"],
    combined: ["vm", "vm_jit", "high_level", "high_level_jit", "compile"],
  },
  gitInfoGet = () => {
    const hash_proc = Bun.spawnSync(["git", "rev-parse", "--short", "HEAD"], { cwd: REPO_ROOT }),
      date_proc = Bun.spawnSync(["git", "log", "-1", "--format=%cI"], { cwd: REPO_ROOT }),
      msg_proc = Bun.spawnSync(["git", "log", "-1", "--format=%s"], { cwd: REPO_ROOT }),
      author_proc = Bun.spawnSync(["git", "log", "-1", "--format=%an"], { cwd: REPO_ROOT });

    return {
      commit: hash_proc.stdout.toString().trim() || "unknown",
      date: date_proc.stdout.toString().trim() || new Date().toISOString(),
      message: msg_proc.stdout.toString().trim() || "local bench",
      author: author_proc.stdout.toString().trim() || "unknown",
    };
  },
  timeUnitToMs = (val_str, unit_str) => {
    const val = Number.parseFloat(val_str);
    if (Number.isNaN(val)) return 0;
    if (unit_str === "s") return val * 1000;
    if (unit_str === "ms") return val;
    if (unit_str === "µs" || unit_str === "us") return val / 1000;
    if (unit_str === "ns") return val / 1000000;
    return val;
  },
  /**
   * 扫 divan 树输出，把每组的 case→ms 抽出来；调用方再按 mode 过滤。
   * 未采到的组（比如 interp mode 下 `--features jit` 没开）自然是空 map，
   * 这里不做取舍。
   */
  divanOutputParse = (text) => {
    const clean_text = text.replace(/\x1b\[[0-9;]*[a-zA-Z]/g, ""),
      line_li = clean_text.split("\n"),
      group_maps = Object.fromEntries(GROUP_DEFS.map((d) => [d.key, {}]));

    let current_group = "";

    for (const raw_line of line_li) {
      const line = raw_line.trim();

      for (const def of GROUP_DEFS) {
        if (line.includes(def.token)) {
          current_group = def.key;
          break;
        }
      }

      // 匹配形如：│  ├─ binarytrees 49.63 ms │ 51.43 ms │ 50.27 ms │ ...
      // 第二个 │ 之后的数值 = median。
      const match = line.match(/[├╰]─\s*([a-zA-Z0-9_]+)\s+.*?│.*?│\s*([0-9.]+)\s*([a-zA-Zµ]+)/);
      if (match && current_group) {
        const case_name = match[1],
          ms_val = Number(timeUnitToMs(match[2], match[3]).toFixed(2));
        group_maps[current_group][case_name] = ms_val;
      }
    }

    return group_maps;
  },
  main = async () => {
    if (!(MODE in MODE_KEYS)) {
      console.error("未知 BENCH_MODE: " + MODE + "（可选 interp / jit / combined）");
      process.exit(2);
    }

    const useJit = MODE !== "interp",
      bench_cmd = [
        "cargo",
        "bench",
        "--locked",
        "-p",
        "ulua",
        "--bench",
        "benchmarks",
        ...(useJit ? ["--features", "jit"] : []),
        "--",
        "--sample-count=" + SAMPLE_COUNT,
      ];

    console.log("[record] mode=" + MODE + " 命令: " + bench_cmd.join(" "));

    const bench_proc = Bun.spawnSync(bench_cmd, { cwd: REPO_ROOT }),
      full_output = bench_proc.stdout.toString() + "\n" + bench_proc.stderr.toString();

    // 无论成败都把 raw divan 输出落盘一次；JIT 崩溃时 publish job 靠它输出诊断。
    await Bun.write(
      join(HERE, "bench-stdout-" + MODE + ".txt"),
      "$ " + bench_cmd.join(" ") + "\n\n" + full_output,
    );

    if (bench_proc.exitCode !== 0) {
      console.error("[record] 基准测试运行失败 (exit " + bench_proc.exitCode + ")");
      // 失败时把尾 60 行贴到 Job Summary，方便一眼看到 SIGSEGV 现场。
      if (process.env.GITHUB_STEP_SUMMARY) {
        const tail = full_output.split("\n").slice(-60).join("\n");
        await Bun.write(
          process.env.GITHUB_STEP_SUMMARY,
          ((await Bun.file(process.env.GITHUB_STEP_SUMMARY).exists())
            ? await Bun.file(process.env.GITHUB_STEP_SUMMARY).text()
            : "") +
            "### ❌ mode=" +
            MODE +
            " 基准运行失败\n\n" +
            "```\n" +
            tail +
            "\n```\n\n",
        );
      }
      process.exit(bench_proc.exitCode);
    }

    const all_maps = divanOutputParse(full_output),
      keep = MODE_KEYS[MODE],
      group_maps = Object.fromEntries(keep.map((k) => [k, all_maps[k] ?? {}])),
      git_info = gitInfoGet();

    const result_payload = { mode: MODE, ...git_info, groups: group_maps };
    await Bun.write(RESULT_PATH, JSON.stringify(result_payload, null, 2) + "\n");
    console.log("✓ [" + MODE + "] 结果写入 " + RESULT_PATH);

    // combined 模式：本地一条命令看完，直接更新 history.json，不依赖 CI。
    if (MODE === "combined") {
      await writeCombinedHistory(group_maps, git_info);
    }
  },
  writeCombinedHistory = async (group_maps, git_info) => {
    const history_li = (await Bun.file(HISTORY_PATH).exists())
        ? await Bun.file(HISTORY_PATH).json()
        : [],
      last_entry = history_li.at(-1) ?? null,
      new_entry = { ...git_info, ...group_maps };

    console.log("\n# ulua 性能回归测试报告 (" + git_info.commit + " - " + git_info.message + ")\n");
    printPair("VM 执行", "vm", "vm_jit", group_maps.vm, group_maps.vm_jit, last_entry);
    printPair(
      "端到端 Lua API",
      "high_level",
      "high_level_jit",
      group_maps.high_level,
      group_maps.high_level_jit,
      last_entry,
    );
    printSingle("编译器吞吐量", group_maps.compile, last_entry?.compile);

    await Bun.write(
      HISTORY_PATH,
      JSON.stringify([...history_li.slice(-99), new_entry], null, 2) + "\n",
    );
    console.log("✓ 性能回归数据已追加至 " + HISTORY_PATH + "\n");
  },
  fmtDelta = (cur, prev) => {
    if (!(prev && prev > 0)) return "-";
    const pct = ((cur - prev) / prev) * 100;
    if (Math.abs(pct) < 1.0) return "基本持平 (±" + Math.abs(pct).toFixed(1) + "%)";
    if (pct < 0) return pct.toFixed(1) + "% (加速 ⚡)";
    return "+" + pct.toFixed(1) + "% (减速 ⚠️)";
  },
  fmt_ms = (v) => (v == null ? "-".padStart(12) : (v.toFixed(1) + " ms").padStart(12)),
  printPair = (title, interp_key, jit_key, interp_map, jit_map, prev_entry) => {
    const case_li = Array.from(
      new Set([...Object.keys(interp_map ?? {}), ...Object.keys(jit_map ?? {})]),
    ).sort();
    console.log("\n## " + title + "：解释器 vs JIT\n");
    console.log(
      "测试用例".padEnd(16) +
        "解释器".padStart(12) +
        "上次".padStart(12) +
        "   " +
        "Δ 解释器".padEnd(16) +
        "JIT".padStart(12) +
        "上次".padStart(12) +
        "   Δ JIT",
    );
    for (const c of case_li) {
      const ci = interp_map?.[c] ?? null,
        pi = prev_entry?.[interp_key]?.[c] ?? null,
        cj = jit_map?.[c] ?? null,
        pj = prev_entry?.[jit_key]?.[c] ?? null;
      console.log(
        c.padEnd(16) +
          fmt_ms(ci) +
          fmt_ms(pi) +
          "   " +
          (ci != null ? fmtDelta(ci, pi) : "-").padEnd(16) +
          fmt_ms(cj) +
          fmt_ms(pj) +
          "   " +
          (cj != null ? fmtDelta(cj, pj) : "-"),
      );
    }
  },
  printSingle = (title, cur_map, prev_map) => {
    const case_li = Object.keys(cur_map ?? {}).sort();
    if (case_li.length === 0) return;
    console.log("\n## " + title + "\n");
    console.log(
      "测试用例".padEnd(16) + "本次耗时".padStart(12) + "上次耗时".padStart(12) + "   性能变化",
    );
    for (const c of case_li) {
      const cur = cur_map[c],
        prev = prev_map?.[c] ?? null;
      console.log(c.padEnd(16) + fmt_ms(cur) + fmt_ms(prev) + "   " + fmtDelta(cur, prev));
    }
  };

await main();
