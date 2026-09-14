use ulua_ast::records::ast_local::AstLocal;
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning,
  records::{lint_local_hygiene::LintLocalHygiene, local_linter::Local},
};
impl LintLocalHygiene {
  /// # Safety
  /// 调用方须保证 `local` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn report_unused_local(&mut self, local: *mut AstLocal, info: &Local) {
    let name = unsafe { (*local).name };
    let bytes = name.as_bytes();
    if bytes.is_empty() || bytes[0] == b'_' {
      return;
    }

    let (code, prefix) = if info.function {
      (Code::FunctionUnused, "Function")
    } else if info.import {
      (Code::ImportUnused, "Import")
    } else {
      (Code::LocalUnused, "Variable")
    };

    emit_warning(
      unsafe { &mut *self.context },
      code,
      unsafe { (*local).location },
      format_args!(
        "{} '{}' is never used; prefix with '_' to silence",
        prefix, name
      ),
    );
  }
}
