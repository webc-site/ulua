use ulua_compiler::records::compile_options::CompileOptions;

use crate::common::functions::run_conformance::optimization_level;

/// cpp `tests/Conformance.test.cpp:107-116` 的 `defaultOptions()`：
/// `CompileOptions{}` 之后写入 `optimizationLevel = optimizationLevel`、
/// `debugLevel = 1`、`typeInfoLevel = 1`（`vectorPrecision` 本端口未建模）。
///
/// `debugLevel = 1` 与 `coverageLevel = 0` 直接取
/// [`CompileOptions::default`]（即 `luacode.h` 头注释里的缺省值），本函数只覆盖
/// 与 harness 绑定/与头注释不同的两项，避免上游改默认值时这里静默漂移。需要偏离
/// 默认的用例一律以 `CompileOptions { 差异字段, ..default_compile_options() }`
/// 起步 —— 对应 cpp 侧 `lua_CompileOptions copts = defaultOptions(); copts.x = y;`。
pub fn default_compile_options() -> CompileOptions {
  CompileOptions {
    optimization_level: optimization_level(),
    type_info_level: 1,
    ..Default::default()
  }
}
