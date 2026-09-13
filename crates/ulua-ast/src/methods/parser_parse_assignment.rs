//! Node: `cxx:Method:Luau.Ast:Ast/src/Parser.cpp:2004:parseAssignment`
//!
//! Faithful port of `Parser::parseAssignment` — `var {, var} = expr {, expr}`.
//! Each assignment target is validated as an l-value (a non-l-value is replaced
//! by an error node, gated on the export-value flag pair like the C++). The
//! target and value lists are gathered in scratch arenas (vars/values share the
//! comma-position arena, stack-disciplined) and copied into the node.

use crate::{
  functions::is_expr_l_value::is_expr_l_value,
  records::{
    ast_expr::AstExpr, ast_node::AstNode, ast_stat::AstStat, ast_stat_assign::AstStatAssign,
    cst_node::CstNode, cst_stat_assign::CstStatAssign, lexeme::Type, location::Location,
    parser::Parser, position::Position, temp_vector::TempVector,
  },
};

impl Parser {
  pub(crate) fn parse_assignment(&mut self, mut initial: *mut AstExpr) -> *mut AstStat {
    if !is_expr_l_value(initial) {
      initial = self.report_l_value_error(initial) as *mut AstExpr;
    }

    let mut vars = TempVector::new(&mut self.scratch_expr);
    let mut vars_comma_positions = TempVector::new(&mut self.scratch_position);
    vars.push_back(initial);

    while self.lexer.current().r#type == Type(b',' as i32) {
      if self.options.store_cst_data {
        vars_comma_positions.push_back(self.lexer.current().location.begin);
      }
      self.next_lexeme();

      let mut expr = self.parse_primary_expr(true);

      if !is_expr_l_value(expr) {
        expr = self.report_l_value_error(expr) as *mut AstExpr;
      }

      vars.push_back(expr);
    }

    let equals_found = self.expect_and_consume_char('=', "assignment");
    let equals_position = if equals_found {
      self.lexer.previous_location().begin
    } else {
      Position::missing()
    };

    let mut values = TempVector::new(&mut self.scratch_expr_aux);
    let mut values_comma_positions = TempVector::new(&mut self.scratch_position);
    self.parse_expr_list(
      &mut values,
      if self.options.store_cst_data {
        Some(&mut values_comma_positions)
      } else {
        None
      },
    );

    let vars_array = self.copy_temp_vector_t(&vars);
    let values_array = self.copy_temp_vector_t(&values);

    let node = unsafe {
      (*self.allocator).alloc(AstStatAssign::new(
        Location::new(
          (*initial).base.location.begin,
          (**values.back()).base.location.end,
        ),
        vars_array,
        values_array,
      ))
    };

    if self.options.store_cst_data {
      let vars_comma = self.copy_temp_vector_t(&vars_comma_positions);
      let values_comma = self.copy_temp_vector_t(&values_comma_positions);
      let cst_node = unsafe {
        (*self.allocator).alloc(CstStatAssign::new(
          vars_comma,
          equals_position,
          values_comma,
        ))
      };
      self
        .cst_node_map
        .try_insert(node as *mut AstNode, cst_node as *mut CstNode);
    }

    node as *mut AstStat
  }
}
