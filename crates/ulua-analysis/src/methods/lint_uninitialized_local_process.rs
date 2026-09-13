use core::ptr::null_mut;

use ulua_ast::visit::ast_stat_visit;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  methods::lint_uninitialized_local_report::lint_uninitialized_local_report,
  records::{lint_context::LintContext, lint_uninitialized_local::LintUninitializedLocal},
};
impl LintUninitializedLocal {
  pub fn process(context: &mut LintContext) {
    let mut pass = LintUninitializedLocal {
      context: context as *mut LintContext,
      locals: DenseHashMap::new(null_mut()),
    };

    unsafe {
      let root = (*pass.context).root;
      ast_stat_visit(root, &mut pass);
    }

    lint_uninitialized_local_report(&mut pass);
  }
}
