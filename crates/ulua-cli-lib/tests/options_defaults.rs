//! CLI 编译选项组装面与缺省级别对账。
//! oracle：`cpp/CLI/src/Bytecode.cpp` / `Compile.cpp` / `Repl.cpp` 中
//! `globalOptions` 默认值与 `copts()` 组装（`-O/-g/-t` 级别参数区间见
//! `option_parsing.rs` 的 union 表，此处不再重复）。
//!
//! 级别全局是进程级原子量，所有触碰它的读用例收敛在同一个 `#[test]` 内
//! 顺序执行，避免与并行用例互串。

use core::ptr::null;

use ulua_cli_lib::{
  functions::copts::{copts, copts_with},
  records::global_options::{
    DEFAULT_DEBUG_LEVEL, DEFAULT_OPTIMIZATION_LEVEL, get_debug_level, get_optimization_level,
    reset_to_defaults, set_debug_level, set_optimization_level,
  },
};

/// cpp `struct GlobalOptions` 的字段默认值（三处 CLI 同值）。
#[test]
fn global_option_defaults_match_cpp() {
  // Bytecode.cpp:19、Compile.cpp:47、Repl.cpp:87：optimizationLevel = 1
  assert_eq!(DEFAULT_OPTIMIZATION_LEVEL, 1);
  // Bytecode.cpp:20、Compile.cpp:48、Repl.cpp:88：debugLevel = 1
  assert_eq!(DEFAULT_DEBUG_LEVEL, 1);
  // Compile.cpp:49：typeInfoLevel = 0（仅 luau-compile 暴露该开关）
  // Bytecode.cpp:28 / Repl.cpp:98：copts() 里 typeInfoLevel 固定写 1
}

/// 级别全局 → `copts()` 的完整数据流；含 set/reset 闭环（单测内顺序执行）。
#[test]
fn levels_flow_into_copts() {
  // 未动过全局：默认 1/1，typeInfoLevel=1，vector 三指针为空
  // （cpp `CompileOptions result = {}` 零初始化 + Bytecode.cpp:26-28）
  let fresh = copts();
  assert_eq!(get_optimization_level(), 1);
  assert_eq!(get_debug_level(), 1);
  assert_eq!(fresh.optimization_level, 1);
  assert_eq!(fresh.debug_level, 1);
  assert_eq!(fresh.type_info_level, 1);
  assert_eq!(fresh.coverage_level, 0);
  assert!(fresh.vector_lib.is_null());
  assert!(fresh.vector_ctor.is_null());
  assert!(fresh.vector_type.is_null());

  // -O2 -g0：globalOptions 落位后 copts() 透出（Bytecode.cpp:63/73）
  set_optimization_level(2);
  set_debug_level(0);
  let changed = copts();
  assert_eq!(changed.optimization_level, 2);
  assert_eq!(changed.debug_level, 0);
  assert_eq!(changed.type_info_level, 1);

  // CLI 入口重置面（reset_to_defaults）恢复 cpp 缺省
  reset_to_defaults();
  let restored = copts();
  assert_eq!(restored.optimization_level, 1);
  assert_eq!(restored.debug_level, 1);
}

/// `copts_with`（全参形态）逐字段映射，含 vector_* 指针透传。
/// oracle：Compile.cpp:61-73（globalOptions 六字段直抄进 CompileOptions）。
#[test]
fn copts_with_maps_all_fields() {
  static VLIB: [u8; 4] = [b'V', b'e', b'2', 0];
  let opts = copts_with(2, 1, 0, VLIB.as_ptr().cast(), null(), null());
  assert_eq!(opts.optimization_level, 2);
  assert_eq!(opts.debug_level, 1);
  assert_eq!(opts.type_info_level, 0);
  assert_eq!(opts.vector_lib, VLIB.as_ptr().cast());
  assert!(opts.vector_ctor.is_null());
  assert!(opts.vector_type.is_null());
  // 其余字段走 CompileOptions::default()（cpp `= {}` 零初始化）
  assert_eq!(opts.coverage_level, 0);
}
