#!/usr/bin/env -S bun

/**
 * ulua 性能回归汇总器 (CI 报告 job 使用)
 *
 * 上游 `benchmark-interpreter` 与 `benchmark-jit` 两个 job 各自把 `record.js` 生成
 * 的 `result.json` 上传成 artifact；本脚本从下载目录读取两者（JIT 允许缺失，
 * 因为它 `continue-on-error: true`），合并成一条 history 记录、更新
 * `benchmarks/regression/history.json`，并把双路对比 + JIT 崩溃摘要写入
 * Job Summary。
 *
 * 环境变量：
 *   BENCH_RESULTS_DIR   存放两个 result.json 的目录，默认 ./bench-results
 *   BENCH_INTERP_NAME   解释器 result 文件名，默认 interp.json
 *   BENCH_JIT_NAME      JIT result 文件名，默认 jit.json
 */

import { dirname, join } from "node:path";

const HERE = import.meta.dirname,
  REPO_ROOT = dirname(dirname(HERE)),
  HISTORY_PATH = join(REPO_ROOT, "benchmarks/regression/history.json"),
  RESULTS_DIR = process.env.BENCH_RESULTS_DIR ?? join(REPO_ROOT, "bench-results"),
  INTERP_PATH = join(RESULTS_DIR, process.env.BENCH_INTERP_NAME ?? "interp.json"),
  JIT_PATH = join(RESULTS_DIR, process.env.BENCH_JIT_NAME ?? "jit.json"),
  readJson = async (path) => {
    const f = Bun.file(path);
    return (await f.exists()) ? await f.json() : null;
  },
  fmtMs = (v) => (v == null ? "-" : v.toFixed(1) + " ms"),
  fmtDelta = (cur, prev) => {
    if (!(prev && prev > 0)) return "-";
    const pct = ((cur - prev) / prev) * 100;
    if (Math.abs(pct) < 1.0) return "基本持平 (±" + Math.abs(pct).toFixed(1) + "%)";
    if (pct < 0) return pct.toFixed(1) + "% (加速 ⚡)";
    return "+" + pct.toFixed(1) + "% (减速 ⚠️)";
  },
  pairTableMd = (title, cur_i, cur_j, prev_i, prev_j) => {
    const case_li = Array.from(
      new Set([...Object.keys(cur_i ?? {}), ...Object.keys(cur_j ?? {})]),
    ).sort();
    let md =
      "### " +
      title +
      "\n\n" +
      "| 测试用例 | 解释器 本次 | 上次 | Δ | JIT 本次 | 上次 | Δ |\n" +
      "| :--- | ---: | ---: | :--- | ---: | ---: | :--- |\n";
    for (const c of case_li) {
      const ci = cur_i?.[c] ?? null,
        pi = prev_i?.[c] ?? null,
        cj = cur_j?.[c] ?? null,
        pj = prev_j?.[c] ?? null;
      md +=
        "| `" +
        c +
        "` | " +
        fmtMs(ci) +
        " | " +
        fmtMs(pi) +
        " | " +
        (ci != null ? fmtDelta(ci, pi) : "-") +
        " | " +
        fmtMs(cj) +
        " | " +
        fmtMs(pj) +
        " | " +
        (cj != null ? fmtDelta(cj, pj) : "-") +
        " |\n";
    }
    return md;
  },
  singleTableMd = (title, cur, prev) => {
    const case_li = Object.keys(cur ?? {}).sort();
    if (case_li.length === 0) return "";
    let md =
      "### " +
      title +
      "\n\n" +
      "| 测试用例 | 本次耗时 | 上次耗时 | 性能波动 |\n" +
      "| :--- | ---: | ---: | :--- |\n";
    for (const c of case_li) {
      md +=
        "| `" +
        c +
        "` | " +
        fmtMs(cur[c]) +
        " | " +
        fmtMs(prev?.[c] ?? null) +
        " | " +
        fmtDelta(cur[c], prev?.[c] ?? null) +
        " |\n";
    }
    return md;
  },
  main = async () => {
    const interp = await readJson(INTERP_PATH),
      jit = await readJson(JIT_PATH);

    if (!interp) {
      console.error("✗ 缺少解释器结果 " + INTERP_PATH + "，无法生成报告");
      process.exit(1);
    }

    const history_li = (await Bun.file(HISTORY_PATH).exists())
        ? await Bun.file(HISTORY_PATH).json()
        : [],
      // 用最后一次「同时含解释器数据」的记录做基线；即使 JIT job 挂掉，
      // 解释器列的 Δ 仍能对上。
      last = history_li.at(-1) ?? null;

    // 合并两侧 group maps；缺哪侧就留空 map（history.json 保留上次同侧数据）。
    const merged_groups = {
      vm: interp.groups.vm ?? {},
      high_level: interp.groups.high_level ?? {},
      compile: interp.groups.compile ?? {},
      vm_jit: jit?.groups?.vm_jit ?? last?.vm_jit ?? {},
      high_level_jit: jit?.groups?.high_level_jit ?? last?.high_level_jit ?? {},
    };

    const new_entry = {
      commit: interp.commit,
      date: interp.date,
      message: interp.message,
      author: interp.author,
      jit_captured: !!jit,
      ...merged_groups,
    };
    await Bun.write(
      HISTORY_PATH,
      JSON.stringify([...history_li.slice(-99), new_entry], null, 2) + "\n",
    );
    console.log("✓ 合并后 history 已更新至 " + HISTORY_PATH);

    // Job Summary
    const summary_path = process.env.GITHUB_STEP_SUMMARY;
    if (!summary_path) return;

    let md = "### ⚡ ulua 性能回归测试报告 (" + interp.commit + ")\n\n";
    if (!jit) {
      md += "> ⚠️ 本次 JIT 分支未产出数据（job 崩溃或被跳过）；下表 JIT 列为上次记录或空。\n\n";
    }
    md +=
      pairTableMd(
        "VM 执行：解释器 vs JIT",
        merged_groups.vm,
        merged_groups.vm_jit,
        last?.vm,
        last?.vm_jit,
      ) + "\n";
    md +=
      pairTableMd(
        "端到端：解释器 vs JIT",
        merged_groups.high_level,
        merged_groups.high_level_jit,
        last?.high_level,
        last?.high_level_jit,
      ) + "\n";
    md += singleTableMd("编译器吞吐量", merged_groups.compile, last?.compile);

    const existing = (await Bun.file(summary_path).exists())
      ? await Bun.file(summary_path).text()
      : "";
    await Bun.write(summary_path, existing + "\n" + md);
  };

await main();
