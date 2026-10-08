// 语言基础特性 fixture 用例
// 移植自 `cpp/tests/Conformance.test.cpp`。

#[test]
fn conformance_assert() {
  use crate::common::functions::run_conformance::run_fixture;

  run_fixture("assert.luau");
}

#[test]
fn conformance_attrib() {
  use crate::common::functions::run_conformance::run_fixture;

  run_fixture("attrib.luau");
}

#[test]
fn conformance_basic() {
  use crate::common::functions::run_conformance::run_fixture;

  run_fixture("basic.luau");
}

#[test]
fn conformance_clear() {
  use crate::common::functions::run_conformance::run_fixture;

  run_fixture("clear.luau");
}

#[test]
fn conformance_closure() {
  use crate::common::functions::run_conformance::run_fixture;

  run_fixture("closure.luau");
}

#[test]
fn conformance_constructs() {
  use crate::common::functions::run_conformance::run_fixture;

  run_fixture("constructs.luau");
}

#[test]
fn conformance_events() {
  use crate::common::functions::run_conformance::run_fixture;

  run_fixture("events.luau");
}

#[test]
fn conformance_if_else_expression() {
  use crate::common::functions::run_conformance::run_fixture;

  run_fixture("ifelseexpr.luau");
}

#[test]
fn conformance_literals() {
  use crate::common::functions::run_conformance::run_fixture;

  run_fixture("literals.luau");
}

#[test]
fn conformance_locals() {
  use crate::common::functions::run_conformance::run_fixture;

  run_fixture("locals.luau");
}

/// cpp `Conformance.test.cpp:1383-1390` 设两条 ScopedFastFlag：
///
/// - `LuauTableMoveTimeoutFix`（DFFlag）打开 `table.move` 的 sparsemove 分支
///   （`cpp/VM/src/ltablib.cpp:153,292`），避免大区间移动退化成 O(n·m)。本端口
///   没有该 DFFlag：`ulua-vm/src/functions/tmove.rs:46-50` 无条件内联计算上游
///   `shouldsparsemove`，即恒等于上游 flag-on。
/// - `LuauTableArrayAdjustCheck`（`Conformance.test.cpp:1386`）给 `luaH_resizearray`
///   补 `nasize > MAXSIZE` 的 "table overflow" 报错（`cpp/VM/src/ltable.cpp:700-702`）。
///   本端口同样没有该 FFlag：`ulua-vm/src/functions/resize.rs:28` 在 `resize` 入口
///   无条件检查 `nasize > MAXSIZE || nhsize > MAXSIZE` 并抛同文案错误，同样恒等于
///   上游 flag-on。
///
/// 故两者都无需作用域 flag。
#[test]
fn conformance_move() {
  use crate::common::functions::run_conformance::run_fixture;

  run_fixture("move.luau");
}

#[test]
fn conformance_pack() {
  use crate::common::functions::run_conformance::run_fixture;

  run_fixture("tpack.luau");
}

// if-local 语法（cpp `Conformance.test.cpp:3335-3338` 的 `TEST_CASE("IfLocal")` +
// fixture 出处 `cpp/tests/conformance/iflocal.luau`）在本端口暂不可运行：该语料同时压
// 四种形态——`if local` 语句、`elseif local`、`if const` 语句，以及 `if local`/`if const`
// 表达式形式（cpp 同一 `DebugLuauIfLocalSyntax` 门控，表达式 parse 在
// `Parser.cpp:4417-4460`、lowering 在 `Compiler.cpp:2538`），上游语句形式
// 受 `DebugLuauIfLocalSyntax` 门控——parse 在 cpp `Ast/src/Parser.cpp:582-584`
// （parseIf 分派）与 `Parser.cpp:607`（`parseIfLocalCondition`），lowering 在 cpp
// `Compiler/src/Compiler.cpp:3806-3808`（compileStatIf 的 conditionLocal 分派，实体
// `compileStatIfLocal` 于 `Compiler.cpp:3724`），AST 字段 `AstStatIf::conditionLocal`
// 在 cpp `Ast/include/Luau/Ast.h:791`。本端口对应现状：`ulua-ast` 的 `AstStatIf`
// （records/ast_stat_if.rs）无 condition_local 字段，`parse_if`
// （methods/parser_parse_if.rs）parse 条件前不分派 `local`；`DebugLuauIfLocalSyntax`
// 在 `ulua-common/src/fflag.rs` 亦未定义。实测（临时探针）直接登记运行会在
// `iflocal.luau:5` 报 `Expected identifier when parsing expression, got 'local'`，
// 卡在 harness 的 parse 前置校验（run_conformance.rs 的 validate_bytecode_graph）。
// 原孤儿 fixture `conformance/iflocal.luau` 已删除（不留不可运行的 fixture 冒充覆盖）；
// 待 sync-cpp 引入上述 parse/lowering 与 flag（`TEST_CASE("IfLocal")` 还带
// `LuauCompileUndoEmitAdjust`，本端口同样未定义）后，连同该用例一并补回。

// `ExportEdgeCase`（cpp `Conformance.test.cpp:5083-5128`）转正见下方
// `conformance_export_edge_case`；`IfLocal` 缺口（上注）仍待 sync-cpp。

/// cpp `Conformance.test.cpp:5083-5128` 的 `TEST_CASE("ExportEdgeCase")`
/// （`LuauExportValueSyntax` 旗标本端口已登记）。
///
/// 1031 个字段把字符串常量编号推过 1023 后，模块导出表 `table.freeze` 的取数在
/// cpp 侧命中的是 10 位组件守卫失败分支——`if (tableCid < 1024 && freezeCid < 1024)`
/// 不成立即回退 `GETGLOBAL`+`GETTABLEKS`（cpp `Compiler/src/Compiler.cpp:443-465`，
/// 注释明言 "GETIMPORT encoding is limited to 10 bits per object id component
/// otherwise we can fallback to getglobal"）。无守卫的实现会把越界组件喂进
/// `pack_import_id` 的组件掩码断言（`ulua-bytecode`
/// `records/bytecode_builder/constants.rs:181`）。
#[test]
fn conformance_export_edge_case() {
  use alloc::string::String;
  use ulua_common::fflag;

  use crate::common::{
    functions::{
      cold_codegen_run::cold_codegen_run, default_compile_options::default_compile_options,
      new_state::new_state, run_conformance::validate_bytecode_graph,
    },
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _luau_export_value_syntax = ScopedFastFlag::new(&fflag::LuauExportValueSyntax, true);

  // cpp:5087-5094：1030 个字段 + 显式 k1030，字符串常量数必然越过 10 位组件上限。
  let mut source = String::from("local t = {\n");
  for idx in 0..1030 {
    source.push_str(&alloc::format!("k{idx} = \"v{idx}\",\n"));
  }
  source.push_str("k1030 = \"v1030\"}\n");
  source.push_str("export local x = 33\n");
  source.push_str("local function a(...) end a(x)");

  // cpp:5104 `validateBytecodeGraph(source, defaultOptions())`；本用例不经
  // `runConformance`，通用往返覆盖不到，须显式做一次（该验证只编译 `source`、
  // 不触碰状态，放在建状态之前无先后依赖）。
  validate_bytecode_graph(source.as_bytes(), &default_compile_options());

  // cpp:5096-5127：`codegen_create(可选) → openlibs/sandbox/sandboxthread →
  // luau_load "=ExportEdgeCase" → CodeGen_ColdFunctions(可选) → lua_resume == 0`，
  // 由 `cold_codegen_run` 门面逐字承载并断言 resume 成功。
  let global_state = new_state();
  let l = global_state.as_ptr();

  cold_codegen_run(l, &source, "=ExportEdgeCase");
}
