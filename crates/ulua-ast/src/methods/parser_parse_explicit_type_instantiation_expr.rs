use core::ptr::{NonNull, from_mut};

use crate::{
  functions::optional_node::{node_opt, opt_node},
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
    let mut cst_node: Option<NonNull<CstExprExplicitTypeInstantiation>> = None;
    if self.options.store_cst_data {
      cst_node = node_opt(self.alloc(CstExprExplicitTypeInstantiation::new(
        CstTypeInstantiation::default(),
      )));
    }

    let mut end_location = Location::default();
    let types_or_packs = self.parse_type_instantiation_expr(
      cst_node.map(|node| {
        // Safety: cst_node 经 node_opt 折叠为 Some 时必非空，其值是本函数开头
        // store_cst_data 门控下 self.alloc 刚 arena 分配的独占 CST 节点（alloc 恒非空，
        // 失败 handle_alloc_error 中止；bump 落位后地址永不移动）；此刻无其他借用，
        // 重建 `&mut` 借用其 instantiation 字段交给 parse_type_instantiation_expr 填写，无别名冲突。
        unsafe { &mut (*node.as_ptr()).instantiation }
      }),
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
        opt_node(cst_node.map(|node| node.cast::<CstNode>())),
      );
    }

    expr
  }
}
