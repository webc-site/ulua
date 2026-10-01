use crate::records::{ast_type::AstType, node_handle::Node};

#[repr(C)]
#[derive(Debug)]
pub struct AstTypeGroup {
  pub base: AstType,
  /// cpp `AstType* type`（`Ast.h:1406`）：四处构造（`Parser.cpp:2655/2667/3031/4686`）
  /// 均以 `parseType` 链或既有单元素列表的 `result[0]`/`params[0]` 接线，文法必建，恒非空。
  pub type_: Node<AstType>,
}
