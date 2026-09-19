use core::ptr::null_mut;

use ulua_ast::visit::ast_stat_visit;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{lint_context::LintContext, lint_duplicate_local::LintDuplicateLocal};
impl LintDuplicateLocal {
  pub fn process(context: &mut LintContext) {
    let mut pass = LintDuplicateLocal {
      context: context as *mut LintContext,
      locals: DenseHashMap::new(null_mut()),
    };

    unsafe {
      let root = (*pass.context).root;
      ast_stat_visit(root, &mut pass);
    }
  }
}
