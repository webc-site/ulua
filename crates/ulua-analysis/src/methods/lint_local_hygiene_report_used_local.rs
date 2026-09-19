use ulua_ast::records::ast_local::AstLocal;
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning,
  records::{lint_local_hygiene::LintLocalHygiene, local_linter::Local},
};
impl LintLocalHygiene {
  /// # Safety
  /// 调用方须保证 `local` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn report_used_local(&mut self, local: *mut AstLocal, info: &Local) {
    let shadow = unsafe { (*local).shadow };
    if !shadow.is_null() {
      let shadow_local = self.locals.find(&shadow);
      let duplicate_function_enabled =
        unsafe { (*self.context).options.is_enabled(Code::DuplicateFunction) };
      let duplicate_local_enabled =
        unsafe { (*self.context).options.is_enabled(Code::DuplicateLocal) };

      if duplicate_function_enabled
        && info.function
        && shadow_local.is_some_and(|shadow_info| shadow_info.function)
      {
        return;
      }

      if duplicate_local_enabled
        && shadow_local.is_some_and(|shadow_info| shadow_info.defined == info.defined)
      {
        return;
      }

      if unsafe { (*shadow).function_depth == (*local).function_depth } {
        emit_warning(
          unsafe { &mut *self.context },
          Code::LocalShadow,
          unsafe { (*local).location },
          format_args!(
            "Variable '{}' shadows previous declaration at line {}",
            unsafe { (*local).name },
            unsafe { (*shadow).location.begin.line + 1 }
          ),
        );
      }

      return;
    }

    if let Some(global) = self.globals.find(unsafe { &(*local).name }) {
      if global.builtin {
        return;
      }

      if !global.first_ref.is_null() {
        emit_warning(
          unsafe { &mut *self.context },
          Code::LocalShadow,
          unsafe { (*local).location },
          format_args!(
            "Variable '{}' shadows a global variable used at line {}",
            unsafe { (*local).name },
            unsafe { (*global.first_ref).base.base.location.begin.line + 1 }
          ),
        );
      } else {
        emit_warning(
          unsafe { &mut *self.context },
          Code::LocalShadow,
          unsafe { (*local).location },
          format_args!("Variable '{}' shadows a global variable", unsafe {
            (*local).name
          }),
        );
      }
    }
  }
}
