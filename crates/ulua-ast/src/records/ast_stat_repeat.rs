use crate::records::{
  ast_expr::AstExpr, ast_stat::AstStat, ast_stat_block::AstStatBlock, node_handle::Node,
};

#[repr(C)]
#[derive(Debug)]
pub struct AstStatRepeat {
  pub base: AstStat,
  /// cpp `AstExpr* condition`（Ast.h:824）：ctor 必传（Ast.h:820/Ast.cpp:697），
  /// 唯一构造点 Parser.cpp:749 交出 parseExpr 的 `cond`（Parser.cpp:745，错误
  /// 回收路径亦 alloc 占位节点，恒非空）→ 恒非空 Node；visit 端无守卫下钻
  /// （Ast.cpp:708）。
  pub condition: Node<AstExpr>,
  /// cpp `AstStatBlock* body`（Ast.h:825）：ctor 必传，构造点交出
  /// parseBlockNoScope 的 `body`（Parser.cpp:735，arena alloc 恒非空——
  /// Parser.cpp:742 还先解引用写 hasEnd），visit 端无守卫下钻
  /// 无守卫下钻（Ast.cpp:707）→ 恒非空 Node。
  pub body: Node<AstStatBlock>,
  pub deprecated_has_until: bool,
}
