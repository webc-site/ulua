/**
 * 基准测试 SVG 矢量图多语言国际化配置与命名规范
 * 一处统一定义，杜绝硬编码，由 I18N_TEXT 的 key 动态派生所有输出文件名
 */

export const BENCH_SVG_BASE = "benchmark";

/**
 * 根据语言代码动态生成该语言需输出的文件名列表
 * @param {string} lang 语言代码（如 'en', 'zh', 'ja'）
 * @returns {string[]} 文件名列表
 */
export const svgFileNamesGet = (lang) => [`${BENCH_SVG_BASE}-${lang}.svg`];

export const I18N_TEXT = {
  en: {
    title: "ulua Performance Benchmark (Geometric Mean · Lower is Faster)",
    subtitle: "8 Benchmark Cases · In-Memory Pure Rust Harness · JIT & Interpretation",
    star: "★ This Project",
    zone_jit: "Just-In-Time (JIT)",
    zone_interp: "Interpretation",
    env_summary: (env) => env?.summary_en || env?.summary || "In-Memory Pure Rust Benchmark",
    footer: (env_str) => `Environment: ${env_str} · 8 Cases Geomean (Lower is Better)`,
    engine_lang: {
      ulua: "Luau (Pure Rust)",
      ulua_jit: "Luau (Pure Rust)",
      mlua_luau: "Luau (C++)",
      mlua_luau_jit: "Luau (C++)",
      mlua_luajit: "LuaJIT 2.1",
      mlua_luajit_interp: "LuaJIT 2.1",
      mlua_lua54: "Lua 5.4",
    },
  },

  zh: {
    title: "ulua 性能基准测试 (几何平均耗时 · 越短越快)",
    subtitle: "8 项基准测试 · 纯 Rust 进程内微基准 · 即时编译与解释执行",
    star: "★ 本项目",
    zone_jit: "即时编译",
    zone_interp: "解释执行",
    env_summary: (env) => env?.summary_zh || env?.summary || "纯 Rust 进程内微基准",
    footer: (env_str) => `测试环境：${env_str} · 8 项几何平均耗时（越短越快）`,
    engine_lang: {
      ulua: "Luau (纯 Rust)",
      ulua_jit: "Luau (纯 Rust)",
      mlua_luau: "Luau (C++)",
      mlua_luau_jit: "Luau (C++)",
      mlua_luajit: "LuaJIT 2.1",
      mlua_luajit_interp: "LuaJIT 2.1",
      mlua_lua54: "Lua 5.4",
    },
  },

  "zh-TW": {
    title: "ulua 效能基準測試 (幾何平均耗時 · 越短越快)",
    subtitle: "8 項基準測試 · 純 Rust 行程內微基準 · 即時編譯與直譯執行",
    star: "★ 本專案",
    zone_jit: "即時編譯",
    zone_interp: "直譯執行",
    env_summary: (env) => env?.summary_zh || env?.summary || "純 Rust 行程內微基準",
    footer: (env_str) => `測試環境：${env_str} · 8 項幾何平均耗時（越短越快）`,
    engine_lang: {
      ulua: "Luau (純 Rust)",
      ulua_jit: "Luau (純 Rust)",
      mlua_luau: "Luau (C++)",
      mlua_luau_jit: "Luau (C++)",
      mlua_luajit: "LuaJIT 2.1",
      mlua_luajit_interp: "LuaJIT 2.1",
      mlua_lua54: "Lua 5.4",
    },
  },

  ja: {
    title: "ulua ベンチマークテスト (相乗平均時間 · 短いほど高速)",
    subtitle: "8つのテストケース · 純Rustインメモリベンチマーク · JIT＆インタプリタ",
    star: "★ 本プロジェクト",
    zone_jit: "JITコンパイル",
    zone_interp: "インタプリタ実行",
    env_summary: (env) => env?.summary_en || env?.summary || "純Rustインメモリベンチマーク",
    footer: (env_str) => `テスト環境：${env_str} · 8件の相乗平均（短いほど高速）`,
    engine_lang: {
      ulua: "Luau (純Rust)",
      ulua_jit: "Luau (純Rust)",
      mlua_luau: "Luau (C++)",
      mlua_luau_jit: "Luau (C++)",
      mlua_luajit: "LuaJIT 2.1",
      mlua_luajit_interp: "LuaJIT 2.1",
      mlua_lua54: "Lua 5.4",
    },
  },
};
