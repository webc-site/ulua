use crate::records::{
  ast_expr::AstExpr,
  ast_local::AstLocal,
  ast_stat::AstStat,
  ast_stat_block::AstStatBlock,
  location::Location,
  node_handle::{Node, OptNode},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstStatFor {
  pub base: AstStat,
  /// cpp `AstLocal* var`（Ast.h:918）：ctor 必传（Ast.h:905-914），构造点
  /// Parser.cpp:844 交出 pushLocal 的返回值（Parser.cpp:2345/5147 双分支皆
  /// `allocator.alloc<AstLocal>`，恒非空）；visit 端直接解引用读 annotation
  /// （Ast.cpp:814）→ 恒非空 Node。
  pub var: Node<AstLocal>,
  /// cpp `AstExpr* from`（Ast.h:919）：ctor 必传，构造点交出 parseExpr 的
  /// `from`（Parser.cpp:819，错误回收亦 alloc 占位节点），visit 端无守卫下钻
  /// （Ast.cpp:817）→ 恒非空 Node。
  pub from: Node<AstExpr>,
  /// cpp `AstExpr* to`（Ast.h:920）：ctor 必传，构造点交出 parseExpr 的 `to`
  /// （Parser.cpp:824），visit 端无守卫下钻（Ast.cpp:818）→ 恒非空 Node。
  pub to: Node<AstExpr>,
  /// cpp `AstExpr* step`（Ast.h:921）：可空——Parser.cpp:827 初始化为
  /// `nullptr`，仅在出现第三逗号时 Parser.cpp:834 填 parseExpr 结果；visit 端
  /// `if (step)` 守卫下钻（Ast.cpp:820-821）→ OptNode。
  pub step: OptNode<AstExpr>,
  /// cpp `AstStatBlock* body`（Ast.h:922）：ctor 必传，构造点交出 parseBlock 的
  /// `body`（Parser.cpp:846，arena alloc 恒非空；Parser.cpp:855 还先解引用写
  /// hasEnd），visit 端无守卫下钻（Ast.cpp:823）→ 恒非空 Node。
  pub body: Node<AstStatBlock>,
  pub has_do: bool,
  pub do_location: Location,
}
