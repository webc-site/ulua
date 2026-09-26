use crate::records::{
  ast_expr::AstExpr, ast_expr_function::AstExprFunction, ast_stat::AstStat, node_handle::Node,
};

#[repr(C)]
#[derive(Debug)]
pub struct AstStatFunction {
  pub base: AstStat,
  /// cpp `AstExpr* name`（Ast.h:993）：ctor 必传（Ast.h:989/Ast.cpp:901），唯一
  /// 构造点 Parser.cpp:1015 交出 parseFunctionName 的 `expr`——parseNameExpr
  /// 双分支恒返回 arena 节点（无名即 alloc\<AstExprError\>，Parser.cpp:3718/3723），
  /// 非左值时又被 reportLValueError/reportExprError 归一为错误节点（同为
  /// alloc 非空）→ 恒非空 Node；visit 端无守卫下钻（Ast.cpp:912）。
  pub name: Node<AstExpr>,
  /// cpp `AstExprFunction* func`（Ast.h:994）：ctor 必传，构造点交出
  /// parseFunctionBody 的 `node`（arena alloc 恒非空），visit 端无守卫下钻
  /// （Ast.cpp:913）→ 恒非空 Node。
  pub func: Node<AstExprFunction>,
}
