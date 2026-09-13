use ulua_ast::{
  records::{ast_attr::AstAttrType, ast_expr_function::AstExprFunction, ast_stat::AstStat},
  visit::ast_stat_visit,
};
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning,
  records::lint_redundant_native_attribute::LintRedundantNativeAttribute,
};
impl LintRedundantNativeAttribute {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_function(&mut self, node: *mut AstExprFunction) -> bool {
    unsafe {
      let node = &*node;

      ast_stat_visit(node.body as *mut AstStat, self);

      for &attr in node.attributes.iter() {
        if !attr.is_null() && (*attr).r#type == AstAttrType::Native {
          emit_warning(
            &mut *self.context,
            Code::RedundantNativeAttribute,
            (*attr).base.location,
            format_args!(
              "native attribute on a function is redundant in a native module; consider removing it"
            ),
          );
        }
      }
    }

    false
  }
}
