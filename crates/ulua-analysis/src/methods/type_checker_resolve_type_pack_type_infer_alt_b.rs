use core::ffi::CStr;

use ulua_ast::{
  records::{
    ast_node::AstNode, ast_type_pack::AstTypePack, ast_type_pack_explicit::AstTypePackExplicit,
    ast_type_pack_generic::AstTypePackGeneric, ast_type_pack_variadic::AstTypePackVariadic,
  },
  rtti::ast_node_as,
};

use crate::{
  records::{
    swapped_generic_type_parameter::SwappedGenericTypeParameter,
    type_checker::TypeChecker,
    type_pack_var::TypePackVar,
    unknown_symbol::{Context, UnknownSymbol},
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_pack_id::TypePackId},
};
impl TypeChecker {
  pub fn resolve_type_pack_scope_ptr_ast_type_pack(
    &mut self,
    scope: ScopePtr,
    annotation: &AstTypePack,
  ) -> TypePackId {
    let node = annotation as *const AstTypePack as *mut AstNode;

    let explicit = unsafe { ast_node_as::<AstTypePackExplicit>(node) };
    if !explicit.is_null() {
      return self
        .resolve_type_pack_scope_ptr_ast_type_list(scope, unsafe { &(*explicit).type_list });
    }

    let variadic = unsafe { ast_node_as::<AstTypePackVariadic>(node) };
    if !variadic.is_null() {
      let ty = if unsafe { (*variadic).variadic_type }.is_null() {
        self.error_recovery_type_scope_ptr(&scope)
      } else {
        self.resolve_type(scope.clone(), unsafe { &*(*variadic).variadic_type })
      };

      return self
        .add_type_pack_type_pack_var(TypePackVar::from(VariadicTypePack { ty, hidden: false }));
    }

    let generic = unsafe { ast_node_as::<AstTypePackGeneric>(node) };
    if !generic.is_null() {
      let name = unsafe { CStr::from_ptr((*generic).generic_name.value) }
        .to_string_lossy()
        .into_owned();
      if let Some(generic_pack) = scope.lookup_pack(&name) {
        return generic_pack;
      }

      let location = unsafe { (*generic).base.base.location };
      if scope.lookup_type(&name).is_some() {
        self.report_error_location_type_error_data(
          &location,
          SwappedGenericTypeParameter {
            name,
            kind: SwappedGenericTypeParameter::PACK,
          }
          .into(),
        );
      } else {
        self.report_error_location_type_error_data(
          &location,
          UnknownSymbol::new(name, Context::Type).into(),
        );
      }

      return self.error_recovery_type_pack_scope_ptr(scope);
    }

    self.error_recovery_type_pack_scope_ptr(scope)
  }
}
