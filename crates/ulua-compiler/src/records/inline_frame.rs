use alloc::vec::Vec;

use ulua_ast::records::ast_expr_function::AstExprFunction;

use crate::records::node::Node;

/// cpp `InlineFrame`（Compiler.cpp:5530-5543）。
///
/// 未移植：cpp:5537 在 `FFlag::LuauCompileMoveElision`（上游默认 false，
/// Common.h:139 单参宏恒 false）下的 `resultLocal` 字段及其唯一赋值点 cpp:1249-1253。
/// 本移植全线按 flag-off 臂实现，等价上游默认态；全面移植需连带 ReturnKind/
/// regCaptured 记账与 compileExprAutoTemp 等 18 处分支（b23-moveelision-sync 调研裁定）。
#[derive(Debug, Clone)]
pub(crate) struct InlineFrame {
  pub(crate) func: Node<AstExprFunction>,
  pub(crate) local_offset: usize,
  pub(crate) target: u8,
  pub(crate) target_count: u8,
  pub(crate) return_jumps: Vec<usize>,
}
