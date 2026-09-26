use crate::records::{
  ast_expr::AstExpr,
  ast_local::AstLocal,
  node_handle::{Node, OptNode},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstExprIfElse {
  pub base: AstExpr,
  /// cpp `AstExpr* condition`（Ast.h:683）：构造必传（Ast.h:666/668），visit 端
  /// 无守卫解引用（Ast.cpp:551），parser 端 parse_expr 恒非空（缺失分支也落
  /// 错误/nil 节点）→ 恒非空 Node。
  pub condition: Node<AstExpr>,
  pub has_then: bool,
  /// cpp `AstExpr* trueExpr`（Ast.h:685），非空口径同 [`Self::condition`]。
  pub true_expr: Node<AstExpr>,
  pub has_else: bool,
  /// cpp `AstExpr* falseExpr`（Ast.h:687），非空口径同 [`Self::condition`]。
  pub false_expr: Node<AstExpr>,
  /// cpp `AstLocal* conditionLocal = nullptr`（Ast.h:690）：`if local`/`if const`
  /// 表达式专用,绑定到 `condition`、仅在 `trueExpr` 作用域内可见。上游语法仍受
  /// `DebugLuauIfLocalSyntax` FFlag 门控、本 parser 未接入,故构造端恒 None;
  /// 可空槽 → OptNode,nullptr 哨兵在类型层消失。
  pub condition_local: OptNode<AstLocal>,
}
