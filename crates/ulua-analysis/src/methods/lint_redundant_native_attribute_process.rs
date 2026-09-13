use ulua_ast::visit::ast_stat_visit;

use crate::records::{
  lint_context::LintContext, lint_redundant_native_attribute::LintRedundantNativeAttribute,
};
pub fn lint_redundant_native_attribute_process(context: &mut LintContext) {
  let mut pass = LintRedundantNativeAttribute {
    context: context as *mut LintContext,
  };
  unsafe {
    ast_stat_visit(context.root, &mut pass);
  }
}
