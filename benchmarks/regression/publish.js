#!/usr/bin/env -S bun

/**
 * ulua 性能回归汇总器 (CI 报告使用)
 *
 * 读取各平台测试结果，分平台输出评测报表并计算几何平均值，
 * 更新 history.json，并在报表末尾输出全平台几何平均总览表。
 */

import { dirname, join } from "node:path";

const HERE = import.meta.dirname,
  REPO_ROOT = dirname(dirname(HERE)),
  HISTORY_PATH = join(REPO_ROOT, "benchmarks/regression/history.json"),
  RESULTS_DIR = process.env.BENCH_RESULTS_DIR ?? join(REPO_ROOT, "bench-results"),
  INTERP_PATH = join(RESULTS_DIR, process.env.BENCH_INTERP_NAME ?? "interp.json"),
  JIT_PATH = join(RESULTS_DIR, process.env.BENCH_JIT_NAME ?? "jit.json"),
  PLATFORM_LI = [
    { id: "ubuntu-latest", label: "Ubuntu Linux (x86_64)", arch: "x86_64" },
    { id: "ubuntu-24.04-arm", label: "Ubuntu Linux (ARM64)", arch: "aarch64" },
    { id: "macos-latest", label: "macOS (Apple Silicon ARM64)", arch: "aarch64" },
    { id: "windows-latest", label: "Windows (x86_64)", arch: "x86_64" },
  ],
  jsonRead = async (path) => {
    const f = Bun.file(path);
    return (await f.exists()) ? await f.json() : null;
  },
  msFmt = (v) => (v == null ? "-" : v.toFixed(1) + " ms"),
  deltaFmt = (cur, prev) => {
    if (!(prev && prev > 0 && cur && cur > 0)) return "-";
    const pct = ((cur - prev) / prev) * 100;
    if (Math.abs(pct) < 1.0) return "持平 (±" + Math.abs(pct).toFixed(1) + "%)";
    if (pct < 0) return pct.toFixed(1) + "%";
    return "+" + pct.toFixed(1) + "%";
  },
  geoMeanCalc = (val_li) => {
    const num_li = val_li.filter((v) => typeof v === "number" && !Number.isNaN(v) && v > 0);
    if (num_li.length === 0) return null;
    const sum = num_li.reduce((acc, v) => acc + Math.log(v), 0);
    return Math.exp(sum / num_li.length);
  },
  pairTableMd = (title, cur_i, cur_j, prev_i, prev_j) => {
    const case_li = Array.from(
      new Set([...Object.keys(cur_i ?? {}), ...Object.keys(cur_j ?? {})]),
    ).sort();
    let md =
      "### " +
      title +
      "\n\n" +
      "| 测试用例 | 解释器 本次 | 上次 | 变动 | JIT 本次 | 上次 | 变动 |\n" +
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
        msFmt(ci) +
        " | " +
        msFmt(pi) +
        " | " +
        (ci != null ? deltaFmt(ci, pi) : "-") +
        " | " +
        msFmt(cj) +
        " | " +
        msFmt(pj) +
        " | " +
        (cj != null ? deltaFmt(cj, pj) : "-") +
        " |\n";
    }

    const g_ci = geoMeanCalc(case_li.map((c) => cur_i?.[c])),
      g_pi = geoMeanCalc(case_li.map((c) => prev_i?.[c])),
      g_cj = geoMeanCalc(case_li.map((c) => cur_j?.[c])),
      g_pj = geoMeanCalc(case_li.map((c) => prev_j?.[c]));

    md +=
      "| **几何平均** | **" +
      msFmt(g_ci) +
      "** | **" +
      msFmt(g_pi) +
      "** | **" +
      deltaFmt(g_ci, g_pi) +
      "** | **" +
      msFmt(g_cj) +
      "** | **" +
      msFmt(g_pj) +
      "** | **" +
      deltaFmt(g_cj, g_pj) +
      "** |\n\n";

    if (g_ci != null && g_cj != null && g_cj > 0) {
      const speedup = (g_ci / g_cj).toFixed(2);
      md += "> **JIT 相对解释器几何加速比**: **`" + speedup + "x`**\n\n";
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
      "| 测试用例 | 本次耗时 | 上次耗时 | 变动 |\n" +
      "| :--- | ---: | ---: | :--- |\n";
    for (const c of case_li) {
      md +=
        "| `" +
        c +
        "` | " +
        msFmt(cur[c]) +
        " | " +
        msFmt(prev?.[c] ?? null) +
        " | " +
        deltaFmt(cur[c], prev?.[c] ?? null) +
        " |\n";
    }

    const g_cur = geoMeanCalc(case_li.map((c) => cur[c])),
      g_prev = geoMeanCalc(case_li.map((c) => prev?.[c]));

    md +=
      "| **几何平均** | **" +
      msFmt(g_cur) +
      "** | **" +
      msFmt(g_prev) +
      "** | **" +
      deltaFmt(g_cur, g_prev) +
      "** |\n\n";

    return md;
  },
  main = async () => {
    const history_li = (await Bun.file(HISTORY_PATH).exists())
        ? await Bun.file(HISTORY_PATH).json()
        : [],
      new_entry_li = [],
      platform_summary_li = [];
    let combined_md = "",
      commit_info = null;

    for (const p of PLATFORM_LI) {
      const p_interp_path = join(RESULTS_DIR, "bench-result-interp-" + p.id, "result.json"),
        p_jit_path = join(RESULTS_DIR, "bench-result-jit-" + p.id, "result.json");

      let interp = await jsonRead(p_interp_path),
        jit = await jsonRead(p_jit_path);

      if (!interp && p.id === "ubuntu-latest") {
        interp = (await jsonRead(INTERP_PATH)) ?? (await jsonRead(join(RESULTS_DIR, "bench-result-interp/result.json")));
        jit = (await jsonRead(JIT_PATH)) ?? (await jsonRead(join(RESULTS_DIR, "bench-result-jit/result.json")));
      }

      if (!interp) {
        continue;
      }

      if (!commit_info) {
        commit_info = {
          commit: interp.commit,
          date: interp.date,
          message: interp.message,
          author: interp.author,
        };
      }

      const prev_entry = history_li.slice().reverse().find(
          (e) => e.platform === p.id || (!e.platform && p.id === "ubuntu-latest"),
        ) ?? null,
        merged_group_map = {
          vm: interp.groups?.vm ?? {},
          high_level: interp.groups?.high_level ?? {},
          compile: interp.groups?.compile ?? {},
          vm_jit: jit?.groups?.vm_jit ?? {},
          high_level_jit: jit?.groups?.high_level_jit ?? {},
        },
        new_entry = {
          commit: interp.commit,
          date: interp.date,
          message: interp.message,
          author: interp.author,
          platform: p.id,
          jit_captured: !!jit,
          ...merged_group_map,
        };
      new_entry_li.push(new_entry);

      const vm_i_geo = geoMeanCalc(Object.values(merged_group_map.vm)),
        vm_j_geo = geoMeanCalc(Object.values(merged_group_map.vm_jit)),
        hl_i_geo = geoMeanCalc(Object.values(merged_group_map.high_level)),
        hl_j_geo = geoMeanCalc(Object.values(merged_group_map.high_level_jit)),
        cmp_geo = geoMeanCalc(Object.values(merged_group_map.compile));

      platform_summary_li.push({
        platform: p.label,
        id: p.id,
        arch: p.arch,
        vm_i: vm_i_geo,
        vm_j: vm_j_geo,
        vm_speedup: (vm_i_geo != null && vm_j_geo != null && vm_j_geo > 0)
          ? (vm_i_geo / vm_j_geo).toFixed(2) + "x"
          : "-",
        hl_i: hl_i_geo,
        hl_j: hl_j_geo,
        hl_speedup: (hl_i_geo != null && hl_j_geo != null && hl_j_geo > 0)
          ? (hl_i_geo / hl_j_geo).toFixed(2) + "x"
          : "-",
        compile: cmp_geo,
      });

      combined_md += "## 分平台评测：" + p.label + " (" + interp.commit + ")\n\n";
      if (!jit) {
        combined_md += "> 注：该平台本次 JIT 分支未产出数据（被跳过或构建未开启）；下表 JIT 列留空。\n\n";
      }
      combined_md +=
        pairTableMd(
          "VM 执行：解释器 vs JIT",
          merged_group_map.vm,
          merged_group_map.vm_jit,
          prev_entry?.vm,
          prev_entry?.vm_jit,
        ) + "\n";
      combined_md +=
        pairTableMd(
          "端到端：解释器 vs JIT",
          merged_group_map.high_level,
          merged_group_map.high_level_jit,
          prev_entry?.high_level,
          prev_entry?.high_level_jit,
        ) + "\n";
      combined_md += singleTableMd("编译器吞吐量", merged_group_map.compile, prev_entry?.compile) + "\n---\n\n";
    }

    if (new_entry_li.length === 0) {
      console.error("未在 " + RESULTS_DIR + " 下找到任何平台的有效解释器结果");
      process.exit(1);
    }

    await Bun.write(
      HISTORY_PATH,
      JSON.stringify([...history_li.slice(-100), ...new_entry_li], null, 2) + "\n",
    );
    console.log("成功更新 " + new_entry_li.length + " 个平台的基线历史至 " + HISTORY_PATH);

    let matrix_md = "## 各平台性能几何平均总览\n\n";
    matrix_md += "| 平台 | 架构 | VM 解释器 | VM JIT | VM 加速比 | 端到端 解释器 | 端到端 JIT | 端到端 加速比 | 编译器耗时 |\n";
    matrix_md += "| :--- | :--- | ---: | ---: | :--- | ---: | ---: | :--- | ---: |\n";

    for (const s of platform_summary_li) {
      matrix_md +=
        "| **" +
        s.platform +
        "** | `" +
        s.arch +
        "` | **" +
        msFmt(s.vm_i) +
        "** | **" +
        msFmt(s.vm_j) +
        "** | " +
        s.vm_speedup +
        " | **" +
        msFmt(s.hl_i) +
        "** | **" +
        msFmt(s.hl_j) +
        "** | " +
        s.hl_speedup +
        " | **" +
        msFmt(s.compile) +
        "** |\n";
    }
    matrix_md += "\n> 注：所有耗时均为微秒级测量经几何平均汇算，数值越小性能越优。\n\n";

    combined_md = combined_md + matrix_md;
    console.log("\n" + matrix_md);

    const summary_path = process.env.GITHUB_STEP_SUMMARY;
    if (summary_path && combined_md) {
      const existing = (await Bun.file(summary_path).exists())
        ? await Bun.file(summary_path).text()
        : "";
      await Bun.write(summary_path, existing + "\n" + combined_md);
      console.log("报表已写入 GITHUB_STEP_SUMMARY");
    }
  };

await main();
