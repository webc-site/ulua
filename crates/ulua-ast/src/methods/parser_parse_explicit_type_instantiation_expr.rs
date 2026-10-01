use core::ptr::{NonNull, from_mut};

use crate::{
  functions::optional_node::opt_node,
  records::{
    ast_expr::AstExpr, ast_expr_instantiate::AstExprInstantiate, ast_node::AstNode,
    cst_expr_explicit_type_instantiation::CstExprExplicitTypeInstantiation, cst_node::CstNode,
    cst_type_instantiation::CstTypeInstantiation, location::Location, parser::Parser,
    position::Position,
  },
};

impl Parser {
  pub fn parse_explicit_type_instantiation_expr(
    &mut self,
    start: Position,
    based_on_expr: &mut AstExpr,
  ) -> *mut AstExpr {
    // CST 出参槽用 Option 表达「CST 记录是否开启」，cpp 置 null 关闭记录的形态在
    // Rust 侧消失；分配走 alloc_mut 单一构造门面（会话级独占写契约见
    // records::parser），调用点全程 safe。
    let mut cst_node: Option<&mut CstExprExplicitTypeInstantiation> = if self.options.store_cst_data
    {
      Some(self.alloc_mut(CstExprExplicitTypeInstantiation::new(
        CstTypeInstantiation::default(),
      )))
    } else {
      None
    };

    let mut end_location = Location::default();
    let types_or_packs = self.parse_type_instantiation_expr(
      cst_node.as_mut().map(|cst| &mut cst.instantiation),
      Some(&mut end_location),
    );

    let expr = self.alloc_expr(AstExprInstantiate::new(
      Location::new(start, end_location.end),
      from_mut(based_on_expr),
      types_or_packs,
    ));

    if self.options.store_cst_data {
      self.cst_node_map.try_insert(
        expr.cast::<AstNode>(),
        // 槽位值落 `Option<NonNull>`：从独占引用取地址（NonNull::from，safe）。
        opt_node(
          cst_node
            .as_mut()
            .map(|cst| NonNull::from(&mut **cst).cast::<CstNode>()),
        ),
      );
    }

    expr
  }
}
