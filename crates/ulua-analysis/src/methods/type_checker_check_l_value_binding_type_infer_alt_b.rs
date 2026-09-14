use core::ffi::CStr;

use ulua_ast::records::ast_expr_local::AstExprLocal;

use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    never_type::NeverType,
    symbol::Symbol,
    type_checker::TypeChecker,
    type_error::TypeError,
    unknown_symbol::{Context, UnknownSymbol},
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_error_data::TypeErrorData, type_id::TypeId},
};
impl TypeChecker {
  pub fn check_l_value_binding_scope_ptr_ast_expr_local(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprLocal,
  ) -> TypeId {
    if let Some(ty) = scope.lookup_symbol(Symbol::from_local(expr.local)) {
      let ty = follow_type_id(ty);
      if get_type_id::<NeverType>(ty).is_some() {
        return self.unknown_type;
      }
      return ty;
    }

    // SAFETY: expr.local 指向 AST arena 节点，name.value 为 NUL 结尾 C 字符串。
    let name_str = unsafe { CStr::from_ptr((*expr.local).name.value).to_string_lossy() };
    let error_data =
      TypeErrorData::UnknownSymbol(UnknownSymbol::new(name_str.to_string(), Context::Binding));
    let error = TypeError::type_error_location_type_error_data(expr.base.base.location, error_data);
    self.report_error_type_error(&error);

    self.error_recovery_type_scope_ptr(scope)
  }
}
