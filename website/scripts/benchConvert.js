#!/usr/bin/env -S bun

/**
 * 将 benchmarks/results.json 转换为前端易导入的 ESM 数据模块 (benchData.js)
 * 严格遵循 .agents/skills/js_review 规范与动态加载机制
 */

import { readdirSync } from "node:fs";
import { basename, extname, join } from "node:path";
import { benchEnvGet } from "./benchEnv.js";

const HERE = import.meta.dirname,
  CASES_DIR = join(HERE, "../../benchmarks/cases"),
  DEFAULT_JSON_PATH = join(HERE, "../../benchmarks/results.json"),
  TARGET_JS_PATH = join(HERE, "../src/lib/benchData.js"),
  // 动态从 benchmarks/cases/ 扫描所有 .lua 文件
  benchCaseScan = () => {
    try {
      return readdirSync(CASES_DIR)
        .filter((file_name) => extname(file_name) === ".lua")
        .sort()
        .map((file_name) => {
          const id = basename(file_name, ".lua");
          return {
            id,
            name: id,
            desc_key: "bench.item." + id + "_desc",
            unit: "ms",
          };
        });
    } catch {
      return [
        {
          id: "binarytrees",
          name: "binarytrees",
          desc_key: "bench.item.binarytrees_desc",
          unit: "ms",
        },
        { id: "fib", name: "fib", desc_key: "bench.item.fib_desc", unit: "ms" },
        { id: "mandel", name: "mandel", desc_key: "bench.item.mandel_desc", unit: "ms" },
        { id: "matmul", name: "matmul", desc_key: "bench.item.matmul_desc", unit: "ms" },
        { id: "nbody", name: "nbody", desc_key: "bench.item.nbody_desc", unit: "ms" },
        {
          id: "spectralnorm",
          name: "spectralnorm",
          desc_key: "bench.item.spectralnorm_desc",
          unit: "ms",
        },
        { id: "strings", name: "strings", desc_key: "bench.item.strings_desc", unit: "ms" },
        { id: "tablesort", name: "tablesort", desc_key: "bench.item.tablesort_desc", unit: "ms" },
      ];
    }
  },
  SCANNED_BENCHMARK_LI = benchCaseScan(),
  DEFAULT_ENGINE_LI = [
    {
      id: "ulua",
      key: "ulua",
      label: "ulua",
      lang: "Luau (纯 Rust)",
      mode: "interp",
      color: "#0969da",
      is_ulua: true,
      is_reference: false,
    },
    {
      id: "ulua_jit",
      key: "ulua-jit",
      label: "ulua (JIT)",
      lang: "Luau (纯 Rust)",
      mode: "jit",
      color: "#0284c7",
      is_ulua: true,
      is_reference: false,
    },
    {
      id: "mlua_luau",
      key: "mlua/luau",
      label: "mlua/luau",
      lang: "Luau (C++)",
      mode: "interp",
      color: "#1f883d",
      is_ulua: false,
      is_reference: false,
    },
    {
      id: "mlua_luau_jit",
      key: "mlua/luau-jit",
      label: "mlua/luau (JIT)",
      lang: "Luau (C++)",
      mode: "jit",
      color: "#d97706",
      is_ulua: false,
      is_reference: false,
    },
    {
      id: "mlua_luajit",
      key: "mlua/luajit",
      label: "LuaJIT (JIT)",
      lang: "LuaJIT 2.1",
      mode: "jit",
      color: "#8250df",
      is_ulua: false,
      is_reference: true,
    },
    {
      id: "mlua_luajit_interp",
      key: "mlua/luajit-interp",
      label: "LuaJIT (解释)",
      lang: "LuaJIT 2.1",
      mode: "interp",
      color: "#6366f1",
      is_ulua: false,
      is_reference: true,
    },
    {
      id: "mlua_lua54",
      key: "mlua/lua5.4",
      label: "Lua 5.4",
      lang: "Lua 5.4",
      mode: "interp",
      color: "#64748b",
      is_ulua: false,
      is_reference: true,
    },
  ],
  geoMeanCalc = (num_li) => {
    if (num_li.length === 0) return 0;
    const log_sum = num_li.reduce((acc, val) => acc + Math.log(val), 0);
    return Math.exp(log_sum / num_li.length);
  },
  benchDataBuild = (source_data) => {
    const benchmark_li =
        source_data?.benchmarks?.map((bench) => ({
          id: bench.id,
          name: bench.name ?? bench.id,
          desc_key: "bench.item." + bench.id + "_desc",
          unit: bench.unit ?? "ms",
        })) ?? SCANNED_BENCHMARK_LI,
      raw_engine_li = source_data?.engines ?? DEFAULT_ENGINE_LI,
      raw_data = source_data?.data ?? {},
      raw_ref = source_data?.reference ?? {},
      engine_map = new Map(),
      engine_li = raw_engine_li
        .filter((eng) => {
          if (
            eng.id === "tsuki" ||
            eng.id === "lua-rs" ||
            eng.key === "tsuki" ||
            eng.key === "lua-rs"
          ) {
            return false;
          }
          if (engine_map.has(eng.key)) return false;
          engine_map.set(eng.key, true);
          return true;
        })
        .map((eng) => ({
          id: eng.id,
          key: eng.key,
          label: eng.label,
          lang: eng.lang,
          mode: eng.mode ?? "interp",
          color: eng.color ?? "#6e7781",
          is_ulua: Boolean(eng.is_ulua),
          is_reference: Boolean(eng.is_reference),
        })),
      result_map = Object.fromEntries(
        benchmark_li.map((bench) => {
          const bench_key = bench.id,
            bench_data = { ...(raw_ref[bench_key] ?? {}), ...(raw_data[bench_key] ?? {}) },
            ulua_time = bench_data.ulua ?? 0,
            engine_entry_li = engine_li.map((eng) => {
              const time_val = bench_data[eng.key] ?? null,
                ratio_vs_ulua =
                  time_val && ulua_time > 0 ? Number((time_val / ulua_time).toFixed(2)) : null;
              return [eng.key, { time_ms: time_val, ratio: ratio_vs_ulua }];
            });
          return [bench_key, Object.fromEntries(engine_entry_li)];
        }),
      ),
      geomean_li = engine_li
        .map((eng) => {
          const time_li = benchmark_li
            .map((bench) => result_map[bench.id]?.[eng.key]?.time_ms)
            .filter((t) => typeof t === "number" && t > 0);

          if (time_li.length === 0) return null;
          const geomean_ms = Number(geoMeanCalc(time_li).toFixed(1));

          return {
            id: eng.id,
            key: eng.key,
            label: eng.label,
            lang: eng.lang,
            mode: eng.mode,
            color: eng.color,
            is_ulua: eng.is_ulua,
            is_reference: eng.is_reference,
            geomean_ms,
          };
        })
        .filter(Boolean);

    geomean_li.sort((first, second) => first.geomean_ms - second.geomean_ms);

    return {
      metadata: {
        timestamp: source_data?.timestamp ?? new Date().toISOString(),
        platform: source_data?.platform ?? "Pure Rust In-Memory (No Process Overhead)",
        runs: source_data?.runs ?? 5,
        environment: benchEnvGet(source_data?.environment),
      },
      benchmark_li,
      engine_li,
      result_map,
      geomean_li,
    };
  },
  src_file = Bun.file(DEFAULT_JSON_PATH),
  raw_json = (await src_file.exists()) ? await src_file.json() : null,
  { metadata, benchmark_li, engine_li, result_map, geomean_li } = benchDataBuild(raw_json),
  js_content =
    "// 由 website/scripts/benchConvert.js 自动生成，请勿手动编辑\n" +
    "export const BENCHMARK_LI = " +
    JSON.stringify(benchmark_li, null, 2) +
    ",\n" +
    "  ENGINE_LI = " +
    JSON.stringify(engine_li, null, 2) +
    ",\n" +
    "  RESULT_MAP = " +
    JSON.stringify(result_map, null, 2) +
    ",\n" +
    "  GEOMEAN_LI = " +
    JSON.stringify(geomean_li, null, 2) +
    ",\n" +
    "  METADATA = " +
    JSON.stringify(metadata, null, 2) +
    ";\n";

await Bun.write(TARGET_JS_PATH, js_content);
console.log("✓ Converted " + DEFAULT_JSON_PATH + " -> " + TARGET_JS_PATH);
