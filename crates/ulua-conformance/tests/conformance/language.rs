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

// `ExportEdgeCase`（cpp `Conformance.test.cpp:5083-5128`，`LuauExportValueSyntax`
// 旗标本端口已登记）在本端口不可运行，属**实现分歧**，按「发现分歧不改实现、不水
// 用例、不进提交」纪律挂账：1030 字段常量表把 `"table"`/`"freeze"` 两个字符串常量
// 的编号推过 1023 后，模块导出表的 `table.freeze` 取数在 cpp 侧有 10 位组件守卫
// ——`if (tableCid < 1024 && freezeCid < 1024)` 否则回退 `GETGLOBAL`+`GETTABLEKS`
// （cpp `Compiler/src/Compiler.cpp:447-467`，注释明言 "GETIMPORT encoding is
// limited to 10 bits per object id component otherwise we can fallback to
// getglobal"）；本端口 `ulua-compiler` 的 `records/compiler/module.rs:133`
// 无条件 `get_import_id2(table_cid, freeze_cid)`，组件越界即触发
// `pack_import_id` 的 `LUAU_ASSERT!(or_all <= K_IMPORT_COMPONENT_MASK)`
// （`ulua-bytecode` `records/bytecode_builder/constants.rs:181`）SIGTRAP。
// 实测（ulua-compile -O1 --fflags=LuauExportValueSyntax=true）：小表 + `export
// local` 通过；1030 字段表 + `export local` 崩；1030 字段表无 export 通过。
// 待 module.rs 补上 `<1024` 守卫与 GETGLOBAL 回退分支后，按 cpp 用例体
// （1030 字段表 + `export local x = 33` + `local function a(...) end a(x)`，
// validateBytecodeGraph → cold codegen → lua_resume == 0）补回本用例。
