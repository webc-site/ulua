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
