use std::ptr::null_mut;

use ulua_common::FFlag::LuauExportValueSyntax;

use crate::records::{
  ast_array::AstArray, ast_expr::AstExpr, ast_node::AstNode, ast_stat::AstStat,
  ast_stat_return::AstStatReturn, cst_node::CstNode, cst_stat_return::CstStatReturn, lexeme::Type,
  location::Location, parser::Parser, temp_vector::TempVector,
};

impl Parser {
  pub fn parse_return(&mut self) -> *mut AstStat {
    let start = self.lexer.current().location;
    self.next_lexeme();

    let mut list = TempVector::new(&mut self.scratch_expr);
    let mut comma_positions = TempVector::new(&mut self.scratch_position);

    if !self.block_follow(self.lexer.current()) && self.lexer.current().r#type != Type::SEMICOLON {
      self.parse_expr_list(
        &mut list,
        if self.options.store_cst_data {
          Some(&mut comma_positions)
        } else {
          None
        },
      );
    }

    let end = if list.empty() {
      start
    } else {
      unsafe { (**list.back()).base.location }
    };

    let list_array = self.copy_temp_vector_t(&list);
    let node = unsafe {
      (*self.allocator).alloc(AstStatReturn::new(
        Location::new(start.begin, end.end),
        list_array,
      ))
    };

    if self.options.store_cst_data {
      let comma_positions_array = self.copy_temp_vector_t(&comma_positions);
      let cst_node = unsafe { (*self.allocator).alloc(CstStatReturn::new(comma_positions_array)) };
      self
        .cst_node_map
        .try_insert(node as *mut AstNode, cst_node as *mut CstNode);
    }

    if LuauExportValueSyntax.get() && self.function_stack.len() == 1 {
      if !self.declared_export_bindings.is_empty() {
        let expressions = self.copy_initializer_list_t(&[node as *mut AstExpr]);
        return self.report_stat_error(
          unsafe { (*node).base.base.location },
          expressions,
          AstArray {
            data: null_mut(),
            size: 0,
          },
          format_args!(
            "Exporting values is not compatible with top-level return (export/return conflict)"
          ),
        ) as *mut AstStat;
      }

      self.has_module_return = true;
    }

    node as *mut AstStat
  }
}
