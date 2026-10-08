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

/// cpp `Conformance.test.cpp:5532-5571` 的
/// `TEST_CASE("ClassInheritanceRepeatedCallMemberOffsetCorruption")`：继承不得
/// 改写常量表里的原始类模板。NEWCLASS 每次执行都经 `luaR_cloneclass` 克隆模板，
/// 其中 `memberstooffset` 走 `luaH_clone` 独立拷贝（cpp `VM/src/lclass.cpp:133`
/// ——该处 131-132 行注释 "the clone shares it" 已过时，**代码**是克隆），再对
/// 克隆跑 `luaR_inheritclass` 把成员偏移整体上移父类实例成员数
/// （cpp `lclass.cpp:227-241`）；若克隆退化为指针共享，上移即写穿共享模板，
/// 同一已加载主闭包第二轮调用起构造器读 `__init` 偏移即越界。cpp 断言：重复
/// 调用 1000 次每次 `LUA_OK`。
#[test]
fn conformance_class_inheritance_repeated_call_member_offset_corruption() {
  use ulua_common::fflag;

  use crate::common::{
    functions::{
      compile_and_load::compile_and_load, new_state::new_state,
      safe_api::{openlibs, pcall, pushvalue},
    },
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  // cpp:5536-5541 的五条 ScopedFastFlag。
  let _debug_luau_user_defined_classes =
    ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);
  let _debug_luau_user_defined_classes_runtime =
    ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClassesRuntime, true);
  let _luau_call_feedback = ScopedFastFlag::new(&fflag::LuauCallFeedback, true);
  let _luau_emit_call_feedback = ScopedFastFlag::new(&fflag::LuauEmitCallFeedback, true);
  let _luau_bytecode_cost_model = ScopedFastFlag::new(&fflag::LuauBytecodeCostModel, true);

  let source = r#"
        open class Parent
            public x: number
        end

        class Child extends Parent
            public y: number
        end
    "#;

  let global_state = new_state();
  let l = global_state.as_ptr();

  // cpp:5543-5550 只有 `luaL_openlibs`（本用例不沙箱），编译加载 "=ClassCorruption"。
  openlibs(l);
  compile_and_load(l, source, "=ClassCorruption", None);

  // cpp:5551-5558：重复调用同一个已加载主闭包；每轮先 pushvalue 复制栈顶，
  // 原闭包留在栈上供下一轮复制。
  for _ in 0..1000 {
    pushvalue(l, -1);
    let status = pcall(l, 0, 0, 0);
    assert_eq!(0, status, "repeated NEWCLASS execution must stay LUA_OK");
  }
}
