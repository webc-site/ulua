use core::ffi::{CStr, c_void};

use ulua_ast::records::{ast_expr_function::AstExprFunction, ast_stat::AstStat};
use ulua_config::enums::code::Code;

use crate::{
  functions::{emit_warning::emit_warning, get_fallthrough::get_fallthrough},
  records::lint_implicit_return::LintImplicitReturn,
};
impl LintImplicitReturn {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit(&mut self, node: *mut AstExprFunction) -> bool {
    let node = unsafe { &*node };
    let bodyf = get_fallthrough(node.body as *const AstStat);
    let vret = self.get_value_return(node.body as *mut c_void);

    if !bodyf.is_null() && !vret.is_null() {
      let location = self.get_end_location(bodyf as *const c_void);
      let return_line = unsafe { (*vret).base.base.location.begin.line + 1 };
      let context = unsafe { &mut *self.context };

      if !node.debugname.value.is_null() {
        let debugname = unsafe { CStr::from_ptr(node.debugname.value) }.to_string_lossy();
        emit_warning(
          context,
          Code::ImplicitReturn,
          location,
          format_args!(
            "Function '{}' can implicitly return no values even though there's an explicit return at line {}; add explicit return to silence",
            debugname, return_line
          ),
        );
      } else {
        emit_warning(
          context,
          Code::ImplicitReturn,
          location,
          format_args!(
            "Function can implicitly return no values even though there's an explicit return at line {}; add explicit return to silence",
            return_line
          ),
        );
      }
    }

    true
  }
}
