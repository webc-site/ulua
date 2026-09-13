use core::ffi::CStr;

use ulua_ast::records::ast_stat_type_alias::AstStatTypeAlias;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::polarity::Polarity,
  functions::{
    as_mutable_type::as_mutable_type_id, as_mutable_type_pack::as_mutable_type_pack_id,
    emplace_type_pack::emplace_type_pack,
  },
  methods::unifiable_bound_type_id_emplace_type_bound_type::unifiable_bound_type_id_emplace_type_bound_type,
  records::{constraint_generator::ConstraintGenerator, scope::Scope, type_fun::TypeFun},
  type_aliases::type_pack_variant::TypePackVariant,
};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn resolve_generic_default_parameters(
    &mut self,
    defn_scope: *mut Scope,
    alias: *mut AstStatTypeAlias,
    fun: &TypeFun,
  ) {
    let alias_ref = unsafe { &*alias };
    LUAU_ASSERT!(alias_ref.generics.size == fun.type_params().len());

    for (i, &ast_ty) in alias_ref.generics.as_slice().iter().enumerate() {
      let param = &fun.type_params()[i];

      if !unsafe { (*ast_ty).default_value }.is_null()
        && let Some(to_unblock) = param.default_value
      {
        let resolves_to = unsafe { (*ast_ty).default_value };
        let resolved = self.resolve_type(
          defn_scope,
          resolves_to,
          /* in_type_arguments */ false,
          /* replace_error_with_fresh */ false,
          Polarity::Positive,
        );
        unsafe {
          let mut resolved = resolved;
          unifiable_bound_type_id_emplace_type_bound_type(
            &mut *as_mutable_type_id(to_unblock),
            &mut resolved,
          );
        }
      }

      unsafe {
        let name_key = CStr::from_ptr((*ast_ty).name.value)
          .to_string_lossy()
          .into_owned();
        (*defn_scope)
          .private_type_bindings
          .insert(name_key, TypeFun::type_fun_type_id(param.ty));
      }
    }

    LUAU_ASSERT!(alias_ref.generic_packs.size == fun.type_pack_params().len());

    for (i, &ast_pack) in alias_ref.generic_packs.as_slice().iter().enumerate() {
      let param = &fun.type_pack_params()[i];

      if !unsafe { (*ast_pack).default_value }.is_null()
        && let Some(to_unblock) = param.default_value
      {
        let resolves_to = unsafe { (*ast_pack).default_value };
        let resolved = self.resolve_type_pack_scope_ptr_ast_type_pack_bool_bool_polarity(
          defn_scope,
          resolves_to,
          /* in_type_arguments */ false,
          /* replace_error_with_fresh */ false,
          Polarity::Positive,
        );
        {
          unsafe {
            emplace_type_pack(
              as_mutable_type_pack_id(to_unblock),
              TypePackVariant::Bound(resolved),
            )
          };
        }
      }

      unsafe {
        let name_key = CStr::from_ptr((*ast_pack).name.value)
          .to_string_lossy()
          .into_owned();
        (*defn_scope)
          .private_type_pack_bindings
          .insert(name_key, param.tp);
      }
    }
  }
}
