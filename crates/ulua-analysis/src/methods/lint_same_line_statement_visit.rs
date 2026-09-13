use core::ffi::c_void;

use ulua_ast::{
  records::{ast_stat_block::AstStatBlock, ast_stat_local::AstStatLocal},
  rtti::ast_node_is,
};
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning, records::lint_same_line_statement::LintSameLineStatement,
};
impl LintSameLineStatement {
  pub fn visit_stat_block(&mut self, node: *mut c_void) -> bool {
    let node = unsafe { &*node.cast::<AstStatBlock>() };
    let body = &node.body;

    for pair in body.as_slice().windows(2) {
      let last_stmt = pair[0];
      let current_stmt = pair[1];

      let last = unsafe { (*last_stmt).base.location };
      let location = unsafe { (*current_stmt).base.location };

      if location.begin.line != last.end.line {
        continue;
      }

      if location.begin.line == self.last_line {
        continue;
      }

      let last_is_local = unsafe { ast_node_is::<AstStatLocal>(&(*last_stmt).base) };
      let current_is_block = unsafe { ast_node_is::<AstStatBlock>(&(*current_stmt).base) };

      if last_is_local && current_is_block {
        continue;
      }

      let last_has_semicolon = unsafe { (*last_stmt).has_semicolon };
      if last_has_semicolon {
        continue;
      }

      let context = unsafe { &mut *self.context };
      emit_warning(
        context,
        Code::SameLineStatement,
        location,
        format_args!(
          "A new statement is on the same line; add semi-colon on previous statement to silence"
        ),
      );

      self.last_line = location.begin.line;
    }

    true
  }
}
