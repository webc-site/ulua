use std::ptr::null_mut;

use crate::records::{
  ast_array::AstArray, ast_node::AstNode, ast_stat::AstStat,
  ast_stat_type_function::AstStatTypeFunction, cst_node::CstNode,
  cst_stat_type_function::CstStatTypeFunction, lexeme::Type, location::Location, name::Name,
  parser::Parser, position::Position,
};

impl Parser {
  pub fn parse_type_function(
    &mut self,
    start: &Location,
    exported: bool,
    type_keyword_position: Position,
  ) -> *mut AstStat {
    let match_fn = *self.lexer.current();
    self.next_lexeme();

    let errors_at_start = self.parse_errors.len();

    let fn_name = self.parse_name_opt("type function name");
    let fn_name =
      fn_name.unwrap_or_else(|| Name::new(self.name_error, self.lexer.current().location));

    self.match_recovery_stop_on_token[Type::RESERVED_END.0 as usize] += 1;

    let old_type_function_depth = self.type_function_depth;
    self.type_function_depth = self.function_stack.len();

    let body = self
      .parse_function_body(
        false,
        &match_fn,
        &fn_name.name,
        None,
        &AstArray {
          data: null_mut(),
          size: 0,
        },
        false,
      )
      .0;

    self.type_function_depth = old_type_function_depth;

    self.match_recovery_stop_on_token[Type::RESERVED_END.0 as usize] -= 1;

    let has_errors = self.parse_errors.len() > errors_at_start;

    let node = unsafe {
      (*self.allocator).alloc(AstStatTypeFunction::new(
        Location::new(start.begin, (*body).base.base.location.end),
        fn_name.name,
        fn_name.location,
        body,
        exported,
        has_errors,
      ))
    };

    if self.options.store_cst_data {
      self.cst_node_map.try_insert(node as *mut AstNode, unsafe {
        (*self.allocator).alloc(CstStatTypeFunction::new(
          type_keyword_position,
          match_fn.location.begin,
        ))
      } as *mut CstNode);
    }

    node as *mut AstStat
  }
}
