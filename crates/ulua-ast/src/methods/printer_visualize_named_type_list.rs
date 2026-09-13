use core::ptr::null;

use crate::{
  records::{
    arg_name_inserter::ArgNameInserter, ast_array::AstArray, ast_node::AstNode,
    ast_type_group::AstTypeGroup, ast_type_list::AstTypeList,
    comma_separator_inserter::CommaSeparatorInserter, position::Position, printer::Printer,
  },
  rtti::ast_node_as,
  type_aliases::ast_argument_name::AstArgumentName,
};

impl<'a> Printer<'a> {
  pub fn visualize_named_type_list(
    &mut self,
    list: &AstTypeList,
    unconditionally_parenthesize: bool,
    open_parentheses_position: Position,
    close_parentheses_position: Position,
    comma_positions: &AstArray<Position>,
    arg_names: &AstArray<Option<AstArgumentName>>,
    arg_names_colon_positions: &AstArray<Position>,
  ) {
    let type_count = list.types.len() + if list.tail_type.is_null() { 0 } else { 1 };

    if type_count == 0 {
      self.maybe_advance_and_write(
        &open_parentheses_position,
        "(",
        unconditionally_parenthesize,
      );
      self.maybe_advance_and_write(
        &close_parentheses_position,
        ")",
        unconditionally_parenthesize,
      );
    } else if type_count == 1 {
      let should_parenthesize = unconditionally_parenthesize
        && (list.types.is_empty()
          || unsafe {
            let first_type = *list.types.begin();
            ast_node_as::<AstTypeGroup>(first_type as *mut AstNode).is_null()
          });

      self.maybe_advance_and_write(&open_parentheses_position, "(", should_parenthesize);

      {
        let mut arg_name_inserter =
          ArgNameInserter::new(self.writer, *arg_names, *arg_names_colon_positions);
        arg_name_inserter.operator_call();
      }

      if list.types.is_empty() {
        unsafe {
          self.visualize_type_pack_annotation(
            &mut *list.tail_type,
            /* for_var_arg */ false,
            /* unconditionally_parenthesize */ false,
            /* for_function_return */ false,
          );
        }
      } else {
        unsafe {
          self.visualize_type_annotation(&mut **list.types.begin());
        }
      }

      self.maybe_advance_and_write(&close_parentheses_position, ")", should_parenthesize);
    } else {
      self.maybe_advance_and_write(
        &open_parentheses_position,
        "(",
        unconditionally_parenthesize,
      );

      let comma_position_ptr = if !comma_positions.is_empty() {
        comma_positions.begin()
      } else {
        null()
      };

      // To avoid multiple mutable borrows of self.writer and self, we must not hold
      // the inserters across calls to visualize_type_annotation/visualize_type_pack_annotation.
      // We track the state (comma first-flag and arg-name index) manually or recreate them.
      let mut comma_first = true;
      let mut arg_name_idx = 0;

      for el in list.types.iter() {
        {
          let mut comma = CommaSeparatorInserter::new(self.writer, comma_position_ptr);
          comma.first = comma_first;
          comma.operator_call(self.writer);
          comma_first = comma.first;
        }
        {
          let mut arg_name =
            ArgNameInserter::new(self.writer, *arg_names, *arg_names_colon_positions);
          arg_name.idx = arg_name_idx;
          arg_name.operator_call();
          arg_name_idx = arg_name.idx;
        }
        unsafe {
          self.visualize_type_annotation(&mut **el);
        }
      }

      if !list.tail_type.is_null() {
        {
          let mut comma = CommaSeparatorInserter::new(self.writer, comma_position_ptr);
          comma.first = comma_first;
          comma.operator_call(self.writer);
        }
        unsafe {
          self.visualize_type_pack_annotation(
            &mut *list.tail_type,
            /* for_var_arg */ false,
            /* unconditionally_parenthesize */ false,
            /* for_function_return */ false,
          );
        }
      }

      self.maybe_advance_and_write(
        &close_parentheses_position,
        ")",
        unconditionally_parenthesize,
      );
    }
  }
}
