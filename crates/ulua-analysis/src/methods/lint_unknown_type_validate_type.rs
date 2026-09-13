use alloc::string::String;

use ulua_ast::records::ast_expr_constant_string::AstExprConstantString;
use ulua_config::enums::code::Code;

use crate::{
  enums::type_kind::TypeKind, functions::emit_warning::emit_warning,
  records::lint_unknown_type::LintUnknownType,
};

impl LintUnknownType {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn validate_type(
    &mut self,
    expr: *mut AstExprConstantString,
    expected: &[TypeKind],
    expected_string: &str,
  ) {
    let expr = unsafe { &*expr };
    let name_bytes = expr.value.as_bytes();
    let name = String::from_utf8_lossy(name_bytes);
    let kind = self.get_type_kind(&name);

    if kind == TypeKind::Unknown {
      let msg = format!("Unknown type '{}'", name);
      emit_warning(
        unsafe { &mut *self.context },
        Code::UnknownType,
        expr.base.base.location,
        format_args!("{}", msg),
      );
      return;
    }

    for &ek in expected {
      if kind == ek {
        return;
      }
    }

    let msg = format!("Unknown type '{}' (expected {})", name, expected_string);
    emit_warning(
      unsafe { &mut *self.context },
      Code::UnknownType,
      expr.base.base.location,
      format_args!("{}", msg),
    );
  }
}
