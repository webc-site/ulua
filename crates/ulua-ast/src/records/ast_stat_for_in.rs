use crate::records::{
  ast_array::AstArray, ast_expr::AstExpr, ast_local::AstLocal, ast_stat::AstStat,
  ast_stat_block::AstStatBlock, location::Location, node_handle::Node,
};

#[repr(C)]
#[derive(Debug)]
pub struct AstStatForIn {
  pub base: AstStat,
  pub vars: AstArray<*mut AstLocal>,
  pub values: AstArray<*mut AstExpr>,
  /// cpp `AstStatBlock* body`（Ast.h:948）：ctor 必传（Ast.h:933-942），构造点
  /// Parser.cpp:915-916 交出 parseBlock 的 `body`（Parser.cpp:904，arena alloc
  /// 恒非空；Parser.cpp:913 还先解引用写 hasEnd），visit 端无守卫下钻
  /// （Ast.cpp:861）→ 恒非空 Node。vars/values 为 AstArray 数组字段（批次③）。
  pub body: Node<AstStatBlock>,
  pub has_in: bool,
  pub in_location: Location,
  pub has_do: bool,
  pub do_location: Location,
}
