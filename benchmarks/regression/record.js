#!/usr/bin/env -S bun

/**
 * ulua 性能基准单路采样器
 *
 * 通过 BENCH_MODE 环境变量决定执行模式：
 *   interp   — 解释器基准
 *   jit      — JIT 基准
 *   combined — 本地全量
 */

import { dirname, join } from "node:path";

const HERE = import.meta.dirname,
  REPO_ROOT = dirname(dirname(HERE)),
  HISTORY_PATH = join(HERE, "history.json"),
  MODE = (process.env.BENCH_MODE ?? "combined").toLowerCase(),
  SAMPLE_COUNT = Number.parseInt(process.env.BENCH_SAMPLES ?? "10", 10) || 10,
  RESULT_PATH = process.env.BENCH_RESULT_PATH ?? join(HERE, "result.json"),
  GROUP_DEF_LI = [
    { token: "vm_execution_jit", key: "vm_jit" },
    { token: "vm_execution", key: "vm" },
    { token: "high_level_lua_jit", key: "high_level_jit" },
    { token: "high_level_lua", key: "high_level" },
    { token: "compile_speed", key: "compile" },
  ],
  MODE_KEY_MAP = {
    interp: ["vm", "high_level", "compile"],
    jit: ["vm_jit", "high_level_jit"],
    combined: ["vm", "vm_jit", "high_level", "high_level_jit", "compile"],
  },
  gitInfoRead = () => {
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
  divanOutputParse = (text) => {
    const clean_text = text.replaceAll(/\x1b\[[0-9;]*[a-zA-Z]/g, ""),
      line_li = clean_text.split("\n"),
      group_map = Object.fromEntries(GROUP_DEF_LI.map((d) => [d.key, {}]));

    let current_group = "";

    for (const raw_line of line_li) {
      const line = raw_line.trim();

      for (const def of GROUP_DEF_LI) {
        if (line.includes(def.token)) {
          current_group = def.key;
          break;
        }
      }

      const match = line.match(/[├╰]─\s*([a-zA-Z0-9_]+)\s+.*?│.*?│\s*([0-9.]+)\s*([a-zA-Zµ]+)/);
      if (match && current_group) {
        const case_name = match[1],
          ms_val = Number(timeUnitToMs(match[2], match[3]).toFixed(2));
        group_map[current_group][case_name] = ms_val;
      }
    }

    return group_map;
  },
  geoMeanCalc = (val_li) => {
    const num_li = val_li.filter((v) => typeof v === "number" && !Number.isNaN(v) && v > 0);
    if (num_li.length === 0) return null;
    const sum = num_li.reduce((acc, v) => acc + Math.log(v), 0);
    return Math.exp(sum / num_li.length);
  },
  deltaFmt = (cur, prev) => {
    if (!(prev && prev > 0 && cur && cur > 0)) return "-";
    const pct = ((cur - prev) / prev) * 100;
    if (Math.abs(pct) < 1.0) return "持平 (±" + Math.abs(pct).toFixed(1) + "%)";
    if (pct < 0) return pct.toFixed(1) + "%";
    return "+" + pct.toFixed(1) + "%";
  },
  msFmt = (v) => (v == null ? "-".padStart(12) : (v.toFixed(1) + " ms").padStart(12)),
  pairPrint = (title, interp_key, jit_key, interp_map, jit_map, prev_entry) => {
    const case_li = Array.from(
      new Set([...Object.keys(interp_map ?? {}), ...Object.keys(jit_map ?? {})]),
    ).sort();
    console.log("\n## " + title + "：解释器 vs JIT\n");
    console.log(
      "测试用例".padEnd(16) +
        "解释器".padStart(12) +
        "上次".padStart(12) +
        "   " +
        "变动".padEnd(16) +
        "JIT".padStart(12) +
        "上次".padStart(12) +
        "   变动",
    );
    for (const c of case_li) {
      const ci = interp_map?.[c] ?? null,
        pi = prev_entry?.[interp_key]?.[c] ?? null,
        cj = jit_map?.[c] ?? null,
        pj = prev_entry?.[jit_key]?.[c] ?? null;
      console.log(
        c.padEnd(16) +
          msFmt(ci) +
          msFmt(pi) +
          "   " +
          (ci != null ? deltaFmt(ci, pi) : "-").padEnd(16) +
          msFmt(cj) +
          msFmt(pj) +
          "   " +
          (cj != null ? deltaFmt(cj, pj) : "-"),
      );
    }
    const g_ci = geoMeanCalc(case_li.map((c) => interp_map?.[c])),
      g_pi = geoMeanCalc(case_li.map((c) => prev_entry?.[interp_key]?.[c])),
      g_cj = geoMeanCalc(case_li.map((c) => jit_map?.[c])),
      g_pj = geoMeanCalc(case_li.map((c) => prev_entry?.[jit_key]?.[c]));
    console.log("-".repeat(84));
    console.log(
      "几何平均".padEnd(18) +
        msFmt(g_ci) +
        msFmt(g_pi) +
        "   " +
        (g_ci != null ? deltaFmt(g_ci, g_pi) : "-").padEnd(16) +
        msFmt(g_cj) +
        msFmt(g_pj) +
        "   " +
        (g_cj != null ? deltaFmt(g_cj, g_pj) : "-"),
    );
    if (g_ci != null && g_cj != null && g_cj > 0) {
      console.log("JIT 相对解释器几何加速比: " + (g_ci / g_cj).toFixed(2) + "x");
    }
  },
  singlePrint = (title, cur_map, prev_map) => {
    const case_li = Object.keys(cur_map ?? {}).sort();
    if (case_li.length === 0) return;
    console.log("\n## " + title + "\n");
    console.log(
      "测试用例".padEnd(16) + "本次耗时".padStart(12) + "上次耗时".padStart(12) + "   变动",
    );
    for (const c of case_li) {
      const cur = cur_map[c],
        prev = prev_map?.[c] ?? null;
      console.log(c.padEnd(16) + msFmt(cur) + msFmt(prev) + "   " + deltaFmt(cur, prev));
    }
    const g_cur = geoMeanCalc(case_li.map((c) => cur_map[c])),
      g_prev = geoMeanCalc(case_li.map((c) => prev_map?.[c]));
    console.log("-".repeat(56));
    console.log(
      "几何平均".padEnd(18) +
        msFmt(g_cur) +
        msFmt(g_prev) +
        "   " +
        deltaFmt(g_cur, g_prev),
    );
  },
  combinedHistoryWrite = async (group_map, git_info) => {
    const history_li = (await Bun.file(HISTORY_PATH).exists())
        ? await Bun.file(HISTORY_PATH).json()
        : [],
      last_entry = history_li.at(-1) ?? null,
      new_entry = { ...git_info, ...group_map };

    console.log("\n# ulua 性能回归测试报告 (" + git_info.commit + " - " + git_info.message + ")\n");
    pairPrint("VM 执行", "vm", "vm_jit", group_map.vm, group_map.vm_jit, last_entry);
    pairPrint(
      "端到端 Lua API",
      "high_level",
      "high_level_jit",
      group_map.high_level,
      group_map.high_level_jit,
      last_entry,
    );
    singlePrint("编译器吞吐量", group_map.compile, last_entry?.compile);

    await Bun.write(
      HISTORY_PATH,
      JSON.stringify([...history_li.slice(-99), new_entry], null, 2) + "\n",
    );
    console.log("性能回归数据已追加至 " + HISTORY_PATH + "\n");
  },
  main = async () => {
    if (!(MODE in MODE_KEY_MAP)) {
      console.error("未知 BENCH_MODE: " + MODE + "（可选 interp / jit / combined）");
      process.exit(2);
    }

    const use_jit = MODE !== "interp",
      bench_cmd = [
        "cargo",
        "bench",
        "--locked",
        "-p",
        "ulua",
        "--bench",
        "benchmarks",
        ...(use_jit ? ["--features", "jit"] : []),
        "--",
        "--sample-count=" + SAMPLE_COUNT,
      ];

    console.log("[record] mode=" + MODE + " 命令: " + bench_cmd.join(" "));

    const bench_proc = Bun.spawnSync(bench_cmd, { cwd: REPO_ROOT }),
      full_output = bench_proc.stdout.toString() + "\n" + bench_proc.stderr.toString();

    await Bun.write(
      join(HERE, "bench-stdout-" + MODE + ".txt"),
      "$ " + bench_cmd.join(" ") + "\n\n" + full_output,
    );

    if (bench_proc.exitCode !== 0) {
      console.error("[record] 基准测试运行失败 (exit " + bench_proc.exitCode + ")");
      if (process.env.GITHUB_STEP_SUMMARY) {
        const tail = full_output.split("\n").slice(-60).join("\n");
        await Bun.write(
          process.env.GITHUB_STEP_SUMMARY,
          ((await Bun.file(process.env.GITHUB_STEP_SUMMARY).exists())
            ? await Bun.file(process.env.GITHUB_STEP_SUMMARY).text()
            : "") +
            "### " +
            MODE +
            " 基准运行失败\n\n" +
            "```\n" +
            tail +
            "\n```\n\n",
        );
      }
      process.exit(bench_proc.exitCode);
    }

    const all_map = divanOutputParse(full_output),
      keep_li = MODE_KEY_MAP[MODE],
      group_map = Object.fromEntries(keep_li.map((k) => [k, all_map[k] ?? {}])),
      git_info = gitInfoRead(),
      result_payload = { mode: MODE, ...git_info, groups: group_map };

    await Bun.write(RESULT_PATH, JSON.stringify(result_payload, null, 2) + "\n");
    console.log("[" + MODE + "] 结果写入 " + RESULT_PATH);

    if (MODE === "combined") {
      await combinedHistoryWrite(group_map, git_info);
    }
  };

await main();
