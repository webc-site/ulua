#!/usr/bin/env -S bun

/**
 * 全引擎性能横向对比报表生成器 (CI / GitHub Actions Job Summary)
 *
 * 严格遵循 .agents/skills/js_review 代码规范：
 * 对标业界最快库 (LuaJIT 2.1)、官方 Luau (C++) 与 PUC Lua 5.4，
 * 生成详尽的 JIT / 解释器双赛道几何平均排行、24 项基准矩阵与编译吞吐报表。
 */

import { dirname, isAbsolute, join } from "node:path";

const HERE = import.meta.dirname,
  REPO_ROOT = dirname(dirname(HERE)),
  RESULTS_DIR_ENV = process.env.BENCH_RESULTS_DIR,
  DEFAULT_RESULTS_DIR = RESULTS_DIR_ENV
    ? (isAbsolute(RESULTS_DIR_ENV) ? RESULTS_DIR_ENV : join(REPO_ROOT, RESULTS_DIR_ENV))
    : join(REPO_ROOT, "bench-results"),
  msFmt = (v) => (v == null ? "-" : (typeof v === "number" ? v.toFixed(1) : v) + " ms"),
  ratioFmt = (target, base) => {
    if (!(target && base && target > 0 && base > 0)) return "-";
    const ratio = target / base;
    return ratio.toFixed(2) + "x";
  },
  speedupFmt = (self_time, target_time) => {
    if (!(self_time && target_time && self_time > 0 && target_time > 0)) return "-";
    const diff = Math.abs(self_time - target_time);
    if (diff < 0.05 || (diff / target_time) < 0.005) return "1.00x (持平)";
    if (self_time < target_time) {
      const pct = (((target_time - self_time) / target_time) * 100).toFixed(1);
      return (target_time / self_time).toFixed(2) + "x (快 " + pct + "%)";
    }
    const pct = (((self_time - target_time) / target_time) * 100).toFixed(1);
    return (target_time / self_time).toFixed(2) + "x (慢 " + pct + "%)";
  },
  geoMeanCalc = (num_li) => {
    let sum = 0, count = 0;
    for (const v of num_li) {
      if (typeof v === "number" && v > 0) {
        sum += Math.log(v);
        ++count;
      }
    }
    return count > 0 ? Math.exp(sum / count) : null;
  },
  jsonRead = async (path) => {
    try {
      const file = Bun.file(path);
      return (await file.exists()) ? await file.json() : null;
    } catch {
      return null;
    }
  },
  candidatePathLiGet = (file_name) => [
    process.env.RUNNER_RESULTS_DIR ? join(process.env.RUNNER_RESULTS_DIR, file_name) : null,
    join(DEFAULT_RESULTS_DIR, "bench-result-runner-ubuntu", file_name),
    join(DEFAULT_RESULTS_DIR, "bench-result-runner", file_name),
    join(REPO_ROOT, "benchmarks", file_name),
    join(HERE, file_name),
  ].filter(Boolean),
  firstJsonRead = async (file_name) => {
    const path_li = candidatePathLiGet(file_name);
    for (const p of path_li) {
      const data = await jsonRead(p);
      if (data) return data;
    }
    return null;
  },
  labelClean = (label) => (label ? label.replace(/\s*\((?:JIT|解释|Interp)\)/gi, "") : ""),
  rankBadgeGet = (idx) => (idx === 0 ? "🥇" : idx === 1 ? "🥈" : idx === 2 ? "🥉" : String(idx + 1)),
  runnerSummaryBuild = (results_data, compile_data, analysis_data) => {
    if (!results_data || !results_data.benchmarks || !results_data.engines) {
      return "";
    }

    const {
        benchmarks: benchmark_li = [],
        engines: engine_li = [],
        data: raw_data = {},
        reference: raw_ref = {},
        environment,
      } = results_data,
      env_str = environment?.summary ?? environment?.os + " (" + environment?.arch + ")",
      merged_data = Object.fromEntries(
        benchmark_li.map((b) => [
          b.id,
          {
            ...(raw_ref[b.id] ?? {}),
            ...(raw_data[b.id] ?? {}),
          },
        ]),
      ),
      engine_geomean_li = engine_li
        .map((eng) => {
          const geomean_val = geoMeanCalc(benchmark_li.map((b) => merged_data[b.id]?.[eng.key]));
          if (geomean_val == null) return null;
          return {
            ...eng,
            geomean_ms: Number(geomean_val.toFixed(1)),
          };
        })
        .filter(Boolean),
      jit_li = engine_geomean_li
        .filter((e) => e.mode === "jit")
        .sort((first, second) => first.geomean_ms - second.geomean_ms),
      interp_li = engine_geomean_li
        .filter((e) => e.mode === "interp")
        .sort((first, second) => first.geomean_ms - second.geomean_ms),
      fastest_jit = jit_li[0] ?? null,
      fastest_interp = interp_li[0] ?? null,
      ulua_jit = jit_li.find((e) => e.key === "ulua-jit") ?? null,
      ulua_interp = interp_li.find((e) => e.key === "ulua") ?? null,
      luau_jit = jit_li.find((e) => e.key === "mlua/luau-jit") ?? null,
      luau_interp = interp_li.find((e) => e.key === "mlua/luau") ?? null,
      has_luau_jit = Boolean(luau_jit),
      has_luau_interp = Boolean(luau_interp),
      has_luau = benchmark_li.some(
        (b) =>
          merged_data[b.id]?.["mlua/luau"] != null ||
          merged_data[b.id]?.["mlua/luau-jit"] != null,
      ),
      title_desc = has_luau
        ? "对标最快 LuaJIT 2.1、官方 Luau C++ 与 PUC Lua 5.4"
        : "对标业内最快 LuaJIT 2.1 与 PUC Lua 5.4";

    let md =
      "## 🚀 全引擎性能横向基准评测 (" +
      title_desc +
      ")\n\n" +
      "> **测试环境**：" +
      env_str +
      " · 纯 Rust 进程内微基准 (消除进程冷启动与 I/O 干扰) · 采样 3~5 轮取最小值\n\n" +
      "### 1. 几何平均综合排行榜 (越短越快 · 毫秒)\n\n" +
      "#### ⚡ 即时编译 (JIT) 赛道 · 极致吞吐对决\n\n" +
      "| 排名 | 引擎 | 实现 / 架构 | 几何平均耗时 | 相对最快 (" +
      (fastest_jit ? labelClean(fastest_jit.label) : "LuaJIT") +
      ") | 相对 ulua (JIT) |" +
      (has_luau_jit ? " 相对官方 Luau (JIT) 加速比 |" : "") +
      "\n| :---: | :--- | :--- | ---: | ---: | ---: |" +
      (has_luau_jit ? " ---: |" : "") +
      "\n";

    jit_li.forEach((eng, idx) => {
      const is_ulua = eng.is_ulua,
        label_text = is_ulua ? "**" + eng.label + " ★ 本项目**" : eng.label,
        vs_fastest = fastest_jit ? ratioFmt(eng.geomean_ms, fastest_jit.geomean_ms) : "-",
        vs_ulua = ulua_jit ? ratioFmt(eng.geomean_ms, ulua_jit.geomean_ms) : "-",
        vs_luau = luau_jit ? ratioFmt(luau_jit.geomean_ms, eng.geomean_ms) : "-";

      md +=
        "| " +
        rankBadgeGet(idx) +
        " | " +
        label_text +
        " | `" +
        eng.lang +
        "` | **" +
        msFmt(eng.geomean_ms) +
        "** | " +
        vs_fastest +
        " | " +
        vs_ulua +
        " |" +
        (has_luau_jit ? " " + vs_luau + " |" : "") +
        "\n";
    });

    md +=
      "\n#### 🛡️ 纯解释执行 (Interpreter) 赛道 · 零 JIT / iOS & Wasm 原生友好\n\n" +
      "| 排名 | 引擎 | 实现 / 架构 | 几何平均耗时 | 相对最快 (" +
      (fastest_interp ? labelClean(fastest_interp.label) : "LuaJIT") +
      ") | 相对 ulua (解释) |" +
      (has_luau_interp ? " 相对官方 Luau (解释) 加速比 |" : "") +
      "\n| :---: | :--- | :--- | ---: | ---: | ---: |" +
      (has_luau_interp ? " ---: |" : "") +
      "\n";

    interp_li.forEach((eng, idx) => {
      const is_ulua = eng.is_ulua,
        label_text = is_ulua ? "**" + eng.label + " ★ 本项目**" : eng.label,
        vs_fastest = fastest_interp ? ratioFmt(eng.geomean_ms, fastest_interp.geomean_ms) : "-",
        vs_ulua = ulua_interp ? ratioFmt(eng.geomean_ms, ulua_interp.geomean_ms) : "-",
        vs_luau = luau_interp ? ratioFmt(luau_interp.geomean_ms, eng.geomean_ms) : "-";

      md +=
        "| " +
        rankBadgeGet(idx) +
        " | " +
        label_text +
        " | `" +
        eng.lang +
        "` | **" +
        msFmt(eng.geomean_ms) +
        "** | " +
        vs_fastest +
        " | " +
        vs_ulua +
        " |" +
        (has_luau_interp ? " " + vs_luau + " |" : "") +
        "\n";
    });

    const conclusion_li = [];
    if (ulua_interp && luau_interp) {
      conclusion_li.push(
        "**纯解释器**：ulua 纯 Rust 解释器 (" +
          msFmt(ulua_interp.geomean_ms) +
          ") 对标官方 Luau C++ (" +
          msFmt(luau_interp.geomean_ms) +
          ") 达到 **" +
          speedupFmt(ulua_interp.geomean_ms, luau_interp.geomean_ms) +
          "**",
      );
    }
    if (ulua_jit && luau_jit) {
      conclusion_li.push(
        "**即时编译**：ulua JIT (" +
          msFmt(ulua_jit.geomean_ms) +
          ") 对标官方 Luau JIT (" +
          msFmt(luau_jit.geomean_ms) +
          ") 达到 **" +
          speedupFmt(ulua_jit.geomean_ms, luau_jit.geomean_ms) +
          "**",
      );
    }
    if (ulua_jit && fastest_jit && fastest_jit.key !== "ulua-jit") {
      conclusion_li.push(
        "**对标最快 (JIT)**：ulua (JIT) 持续紧逼业内最快标杆 " +
          labelClean(fastest_jit.label) +
          " (" +
          msFmt(fastest_jit.geomean_ms) +
          ")，差距收敛至 **" +
          ratioFmt(ulua_jit.geomean_ms, fastest_jit.geomean_ms) +
          "**",
      );
    }
    if (ulua_interp && fastest_interp && fastest_interp.key !== "ulua") {
      conclusion_li.push(
        "**对标最快 (解释器)**：ulua 纯 Rust 解释器 (" +
          msFmt(ulua_interp.geomean_ms) +
          ") 对标业内最快标杆 " +
          labelClean(fastest_interp.label) +
          " (" +
          msFmt(fastest_interp.geomean_ms) +
          ")，差距收敛至 **" +
          ratioFmt(ulua_interp.geomean_ms, fastest_interp.geomean_ms) +
          "**",
      );
    }

    if (conclusion_li.length > 0) {
      md +=
        "\n> **核心结论**：\n" +
        conclusion_li
          .map((item, idx) => "> - " + item + (idx === conclusion_li.length - 1 ? "。\n\n" : "；\n"))
          .join("");
    }

    md +=
      "### 2. 全量测试用例耗时矩阵 (" +
      benchmark_li.length +
      " 项基准测试 · 毫秒)\n\n" +
      "| 测试用例 | ulua (解释) | ulua (JIT) |" +
      (has_luau ? " mlua/luau (解释) | mlua/luau (JIT) |" : "") +
      " LuaJIT (解释) | LuaJIT (JIT) 🏆 | Lua 5.4 |" +
      (has_luau ? " ulua vs 官方 (解释) |" : "") +
      " ulua vs LuaJIT (JIT) |\n" +
      "| :--- | ---: | ---: |" +
      (has_luau ? " ---: | ---: |" : "") +
      " ---: | ---: | ---: |" +
      (has_luau ? " :--- |" : "") +
      " :--- |\n";

    benchmark_li.forEach((b) => {
      const row = merged_data[b.id] ?? {},
        u_i = row.ulua ?? null,
        u_j = row["ulua-jit"] ?? null,
        l_i = row["mlua/luau"] ?? null,
        l_j = row["mlua/luau-jit"] ?? null,
        lj_i = row["mlua/luajit-interp"] ?? null,
        lj_j = row["mlua/luajit"] ?? null,
        l54 = row["mlua/lua5.4"] ?? null,
        vs_luau = u_i && l_i ? speedupFmt(u_i, l_i) : "-",
        vs_lj = u_j && lj_j
          ? (u_j < lj_j ? ratioFmt(u_j, lj_j) + " ⚡快" : ratioFmt(u_j, lj_j))
          : u_i && lj_i
            ? (u_i < lj_i ? ratioFmt(u_i, lj_i) + " ⚡快" : ratioFmt(u_i, lj_i))
            : "-";

      md +=
        "| `" +
        b.id +
        "` | **" +
        msFmt(u_i) +
        "** | **" +
        msFmt(u_j) +
        "** |" +
        (has_luau ? " " + msFmt(l_i) + " | " + msFmt(l_j) + " |" : "") +
        " " +
        msFmt(lj_i) +
        " | " +
        msFmt(lj_j) +
        " | " +
        msFmt(l54) +
        " |" +
        (has_luau ? " " + vs_luau + " |" : "") +
        " " +
        vs_lj +
        " |\n";
    });

    // 几何平均汇总行
    const g_ui = ulua_interp?.geomean_ms ?? null,
      g_uj = ulua_jit?.geomean_ms ?? null,
      g_li = luau_interp?.geomean_ms ?? null,
      g_lj = luau_jit?.geomean_ms ?? null,
      g_lji = interp_li.find((e) => e.key === "mlua/luajit-interp")?.geomean_ms ?? null,
      g_ljj = jit_li.find((e) => e.key === "mlua/luajit")?.geomean_ms ?? null,
      g_l54 = interp_li.find((e) => e.key === "mlua/lua5.4")?.geomean_ms ?? null,
      g_vs_luau = g_ui && g_li ? speedupFmt(g_ui, g_li) : "-",
      g_vs_lj = g_uj && g_ljj ? ratioFmt(g_uj, g_ljj) : "-";

    md +=
      "| **几何平均** | **" +
      msFmt(g_ui) +
      "** | **" +
      msFmt(g_uj) +
      "** |" +
      (has_luau ? " **" + msFmt(g_li) + "** | **" + msFmt(g_lj) + "** |" : "") +
      " **" +
      msFmt(g_lji) +
      "** | **" +
      msFmt(g_ljj) +
      "** | **" +
      msFmt(g_l54) +
      "** |" +
      (has_luau ? " **" + g_vs_luau + "** |" : "") +
      " **" +
      g_vs_lj +
      "** |\n\n";

    // 编译吞吐量对比表
    if (compile_data && compile_data.data && compile_data.benchmarks) {
      const has_mlua_compile = compile_data.benchmarks.some(
        (cb) => compile_data.data[cb.id]?.["mlua-compile"] != null
      );
      md +=
        "### 3. 编译器前端与字节码编译吞吐量\n\n" +
        "| 源码测试用例 | ulua 纯语法解析 (parse) | ulua 解析+编译 (compile) |" +
        (has_mlua_compile
          ? " 官方 mlua/luau 编译 (C++) | ulua 相对官方编译吞吐 |"
          : "") +
        "\n| :--- | ---: | ---: |" +
        (has_mlua_compile ? " ---: | :--- |" : "") +
        "\n";

      compile_data.benchmarks.forEach((cb) => {
        const c_row = compile_data.data[cb.id] ?? {},
          parse_time = c_row["ulua-parse"] ?? null,
          ulua_cmp = c_row["ulua-compile"] ?? null,
          mlua_cmp = c_row["mlua-compile"] ?? null,
          vs_official = ulua_cmp && mlua_cmp ? speedupFmt(ulua_cmp, mlua_cmp) : "-";

        md +=
          "| `" +
          cb.id +
          "` | " +
          msFmt(parse_time) +
          " | **" +
          msFmt(ulua_cmp) +
          "** |" +
          (has_mlua_compile
            ? " " + msFmt(mlua_cmp) + " | " + vs_official + " |"
            : "") +
          "\n";
      });
      md += "\n";
    }

    // 静态类型检查吞吐量
    if (analysis_data && analysis_data.data && analysis_data.benchmarks) {
      md +=
        "### 4. 静态类型检查器吞吐量 (ulua-analysis, strict 模式)\n\n" +
        "| 类型检查基准项 | globals 导入耗时 | 端到端类型推导与检查总耗时 | 相对基线倍率 |\n" +
        "| :--- | ---: | ---: | ---: |\n";

      analysis_data.benchmarks.forEach((ab) => {
        const a_row = analysis_data.data[ab.id] ?? {},
          globals_time = a_row["ulua-analysis-globals"] ?? null,
          check_time = a_row["ulua-analysis-check"] ?? null,
          check_ratio = globals_time && check_time ? ratioFmt(check_time, globals_time) : "-";

        md +=
          "| `" +
          ab.id +
          "` | " +
          msFmt(globals_time) +
          " | **" +
          msFmt(check_time) +
          "** | " +
          check_ratio +
          " |\n";
      });
      md += "\n";
    }

    md += "---\n\n";
    return md;
  },
  main = async () => {
    const results_data = await firstJsonRead("results.json"),
      compile_data = await firstJsonRead("results-compile.json"),
      analysis_data = await firstJsonRead("results-analysis.json");

    if (!results_data) {
      console.warn("提示: 未检索到有效 results.json，跳过 runner 全量汇总");
      return;
    }

    const runner_md = runnerSummaryBuild(results_data, compile_data, analysis_data);
    if (!runner_md) return;

    const summary_path = process.env.GITHUB_STEP_SUMMARY;
    if (summary_path) {
      const existing = (await Bun.file(summary_path).exists())
        ? await Bun.file(summary_path).text()
        : "";
      await Bun.write(summary_path, existing + "\n" + runner_md);
      console.log("✓ 全引擎基准对比报表已写入 GITHUB_STEP_SUMMARY");
    } else {
      console.log(runner_md);
    }
  };

export { runnerSummaryBuild, firstJsonRead };

if (import.meta.main) {
  await main();
}
