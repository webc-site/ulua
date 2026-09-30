use core::ptr::from_mut;

use ulua_ast::{
  enums::ast_stat_ref::AstStatRef,
  records::{ast_stat_block::AstStatBlock, ast_visitor::AstVisitor},
  visit::ast_stat_visit,
};
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning,
  macros::lint_stat_process,
  records::{lint_context::LintContext, lint_context_handle::LintContextHandle},
};

#[derive(Debug, Clone)]
pub struct LintSameLineStatement<'ctx> {
  pub(crate) context: LintContextHandle<'ctx>,
  pub(crate) last_line: u32,
}

impl<'ctx> AstVisitor for LintSameLineStatement<'ctx> {
  fn visit_stat_block(&mut self, node: &mut AstStatBlock) -> bool {
    // methods/ 的 inherent 桥接仍以 *mut () 收口，此处仅做指针形态转换。
    self.visit_stat_block(from_mut(node).cast::<()>())
  }
}

// —— 原 methods/lint_same_line_statement_process.rs ——
impl<'ctx> LintSameLineStatement<'ctx> {
  lint_stat_process!(LintSameLineStatement {
    last_line: u32::MAX
  });
}

// —— 原 methods/lint_same_line_statement_visit.rs ——
impl<'ctx> LintSameLineStatement<'ctx> {
  pub fn visit_stat_block(&mut self, node: *mut ()) -> bool {
    // SAFETY: node 来自 visitor 分发，指向存活的 repr(C) AstStatBlock。
    let node = unsafe { &*node.cast::<AstStatBlock>() };
    let body = &node.body;
    for pair in body.as_slice().windows(2) {
      let last = pair[0].get();
      let current = pair[1].get();
      let location = current.base.location;
      if location.begin.line != last.base.location.end.line {
        continue;
      }
      if location.begin.line == self.last_line {
        continue;
      }
      let last_is_local = matches!(last.as_stat_ref(), AstStatRef::Local(_));
      let current_is_block = matches!(current.as_stat_ref(), AstStatRef::Block(_));
      if last_is_local && current_is_block {
        continue;
      }
      if last.has_semicolon {
        continue;
      }
      let mut handle = self.context;
      let context = handle.get();
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
