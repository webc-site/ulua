use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_global::AstExprGlobal, ast_node::AstNode,
  },
  rtti::ast_node_as,
};
use ulua_config::enums::code::Code;

use crate::{
  functions::{emit_warning::emit_warning, is_number::is_number},
  records::lint_deprecated_api::LintDeprecatedApi,
};
impl LintDeprecatedApi {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_call(&mut self, node: *mut AstExprCall) -> bool {
    unsafe {
      if (*node).self_ || (*node).args.size < 1 {
        return true;
      }

      let fenv = ast_node_as::<AstExprGlobal>((*node).func as *mut AstNode);

      if fenv.is_null() {
        return true;
      }
      let fenv_bytes = (*fenv).name.as_bytes();
      if fenv_bytes != b"getfenv" && fenv_bytes != b"setfenv" {
        return true;
      }

      let level = *(*node).args.data.add(0);
      let ty = (*self.context).get_type(level);
      let level_is_number =
        ty.is_some_and(is_number) || (*(level as *mut AstNode)).is::<AstExprConstantNumber>();

      if level_is_number {
        let suggestion = if fenv_bytes == b"getfenv" {
          "; consider using 'debug.info' instead"
        } else {
          ""
        };
        let function_name = (*fenv).name;

        emit_warning(
          &mut *self.context,
          Code::DeprecatedApi,
          (*node).base.base.location,
          format_args!("Function '{}' is deprecated{}", function_name, suggestion),
        );
      }
    }

    true
  }
}
