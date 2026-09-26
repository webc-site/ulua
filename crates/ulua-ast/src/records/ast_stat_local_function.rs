use crate::records::{
  ast_expr_function::AstExprFunction, ast_local::AstLocal, ast_stat::AstStat, node_handle::Node,
};

#[repr(C)]
#[derive(Debug)]
pub struct AstStatLocalFunction {
  pub base: AstStat,
  /// cpp `AstLocal* name`（Ast.h:1006）：ctor 必传（Ast.h:1002），唯一构造点
  /// Parser.cpp:1350 交回 `parseFunctionBody` 的 `funLocal`，而该调用点 `localName`
  /// 恒为 `Some(&name)`（`local function <name>`），`pushLocal`（Parser.cpp:2345/5147
  /// `allocator.alloc`）恒非空 → 恒非空 Node。name 不是 visit 子节点
  /// （Ast.cpp:926 只下钻 func），故无守卫下钻证据来自构造端。
  pub name: Node<AstLocal>,
  /// cpp `AstExprFunction* func`（Ast.h:1007）：ctor 必传，visit 端无守卫下钻
  /// （Ast.cpp:929），parser 端 `parseFunctionBody` 返回的 `node` 恒非空
  /// → 恒非空 Node。
  pub func: Node<AstExprFunction>,
  pub is_const: bool,
}
