use core::ffi::CStr;

use ulua_ast::records::ast_expr_global::AstExprGlobal;

use crate::{
  enums::value_context::ValueContext,
  records::{
    non_strict_context::NonStrictContext,
    non_strict_type_checker::NonStrictTypeChecker,
    symbol::Symbol,
    unknown_symbol::{UnknownSymbol, UnknownSymbol_Context},
  },
  type_aliases::type_error_data::TypeErrorData,
};
impl NonStrictTypeChecker {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_ast_expr_global_value_context(
    &mut self,
    global: *mut AstExprGlobal,
    context: ValueContext,
  ) -> NonStrictContext {
    // We don't file unknown symbols for LValues.
    if context == ValueContext::LValue {
      return NonStrictContext::new();
    }

    let Some(scope) = self.stack.last().copied() else {
      return NonStrictContext::new();
    };

    let sym = unsafe { Symbol::from_global((*global).name) };
    if unsafe { (*scope).lookup_symbol(sym).is_none() } {
      let name_str = unsafe { CStr::from_ptr((*global).name.value).to_string_lossy() };
      let error_data = TypeErrorData::UnknownSymbol(UnknownSymbol::new(
        name_str.to_string(),
        UnknownSymbol_Context::Binding,
      ));
      unsafe { self.report_error(error_data, &(*global).base.base.location) };
    }

    NonStrictContext::new()
  }
}
