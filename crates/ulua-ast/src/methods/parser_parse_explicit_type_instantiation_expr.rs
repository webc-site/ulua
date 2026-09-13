use core::ptr::null_mut;

use crate::records::{
  ast_expr::AstExpr, ast_expr_instantiate::AstExprInstantiate, ast_node::AstNode,
  cst_expr_explicit_type_instantiation::CstExprExplicitTypeInstantiation, cst_node::CstNode,
  cst_type_instantiation::CstTypeInstantiation, location::Location, parser::Parser,
  position::Position,
};

impl Parser {
  pub fn parse_explicit_type_instantiation_expr(
    &mut self,
    start: Position,
    based_on_expr: &mut AstExpr,
  ) -> *mut AstExpr {
    let mut cst_node: *mut CstExprExplicitTypeInstantiation = null_mut();
    if self.options.store_cst_data {
      cst_node = unsafe {
        (*self.allocator).alloc(CstExprExplicitTypeInstantiation::new(
          CstTypeInstantiation::default(),
        ))
      };
    }

    let mut end_location = Location::default();
    let types_or_packs = self.parse_type_instantiation_expr(
      if !cst_node.is_null() {
        unsafe { &mut (*cst_node).instantiation }
      } else {
        null_mut()
      },
      Some(&mut end_location),
    );

    let expr = unsafe {
      (*self.allocator).alloc(AstExprInstantiate::new(
        Location::new(start, end_location.end),
        based_on_expr as *mut AstExpr,
        types_or_packs,
      ))
    };

    if self.options.store_cst_data {
      self
        .cst_node_map
        .try_insert(expr as *mut AstNode, cst_node as *mut CstNode);
    }

    expr as *mut AstExpr
  }
}
