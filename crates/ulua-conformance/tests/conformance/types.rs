// 类型标注、RTTI 与 class 语法用例
// 移植自 `cpp/tests/Conformance.test.cpp`。

#[test]
fn conformance_classes() {
  use ulua_common::fflag;

  use crate::common::{
    functions::run_conformance::run_fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  // cpp `Conformance.test.cpp:4685-4692` 的六个 ScopedFastFlag：前两个打开 user-defined
  // class 的类型与运行时支持，后四个打开 call feedback / cost model / virtual BC builder
  // 的字节码路径 —— classes.luau 在上游就是带着这四条一起跑的，缺了会测到另一套配置。
  let _debug_luau_user_defined_classes =
    ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);
  let _debug_luau_user_defined_classes_runtime =
    ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClassesRuntime, true);
  let _call_feedback = ScopedFastFlag::new(&fflag::LuauCallFeedback, true);
  let _emit_call_feedback = ScopedFastFlag::new(&fflag::LuauEmitCallFeedback, true);
  let _bytecode_cost_model = ScopedFastFlag::new(&fflag::LuauBytecodeCostModel, true);
  let _virtual_bc_builder = ScopedFastFlag::new(&fflag::LuauVirtualBcBuilder, true);

  run_fixture("classes.luau");
}

#[test]
fn conformance_explicit_type_instantiations() {
  use crate::common::functions::run_conformance::run_fixture;

  run_fixture("explicit_type_instantiations.luau");
}

#[test]
fn conformance_types() {
  use ulua_common::fflag;

  use crate::common::{
    functions::{
      conformance_types_setup::conformance_types_setup, run_conformance::run_fixture_setup,
    },
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  // cpp `Conformance.test.cpp:2017`：`ScopedFastFlag integerType{FFlag::LuauIntegerType2, true}`。
  // setup 会对每个全局 binding 调 populate_rtti 后无条件 lua_setfield，而 populate_rtti 的
  // Integer 分支（cpp `Conformance.test.cpp:1955-1958`）只在该旗标为真时压栈；旗标不设成
  // true 就会少压一次栈，setfield 反过来消费栈顶的 RTTI 表本身。
  let _integer_type = ScopedFastFlag::new(&fflag::LuauIntegerType2, true);

  run_fixture_setup("types.luau", conformance_types_setup);
}

// `ClassInheritanceRepeatedCallMemberOffsetCorruption`
// （cpp `Conformance.test.cpp:5532-5571`，五旗标组合本端口均已登记）在本端口
// 不可运行，属**实现分歧**，按「发现分歧不改实现、不水用例、不进提交」纪律挂账：
// cpp `NEWCLASS` 每次执行都对常量表里的共享模板 `luaR_cloneclass`，且克隆对
// `memberstooffset` 做 `luaH_clone` 独立拷贝（cpp `VM/src/lclass.cpp:141`——
// 该处注释 "the clone shares it" 已过时，**代码**是克隆；cpp 该用例正是为防
// 写穿共享模板而立，重复调用同一主闭包 1000 次每次必须 `LUA_OK`）。
// 本端口 `ulua-vm` `functions/lua_r_cloneclass.rs` 照抄了过时注释：
// `(*newclass).memberstooffset = (*classobject).memberstooffset;` 按指针共享；
// 随后 `luaR_inheritclass` 对 child 的 memberstooffset 整体上移 parent 实例成员数
// （cpp lclass.cpp:227-241，克隆共享后即写穿模板）。实测（首跑正常、第二轮起崩）：
// `open class Parent / public x / class Child extends Parent` 的已加载主闭包
// 第二次 `lua_pcall` 起，`lua_r_setupconstructor` 读 `__init` 偏移已 +1，
// `lua_r_newclass.rs:307` 越界（len 2 index 2）。
// 待 cloneclass 改回 `luaH_clone` 语义（`lua_h_clone` 已在位）后，按 cpp 用例体
// （openlibs → compile_and_load "=ClassCorruption" → pushvalue+pcall ×1000 全零）
// 补回本用例。
