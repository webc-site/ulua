use alloc::{string::String, vec::Vec};
use core::ffi::CStr;

use ulua_ast::records::ast_stat_type_alias::AstStatTypeAlias;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{control_flow::ControlFlow, polarity::Polarity},
  functions::{
    as_mutable_type::as_mutable_type_id, get_type_alt_j::get_type_id,
    occurs_check_type_utils::occurs_check_type_id_type_id,
  },
  records::{
    blocked_type::BlockedType, constraint_generator::ConstraintGenerator,
    name_constraint::NameConstraint, occurs_check_failed::OccursCheckFailed,
    reserved_identifier::ReservedIdentifier, scope::Scope,
  },
  type_aliases::{
    constraint_v::ConstraintV, scope_ptr_type::ScopePtr, type_error_data::TypeErrorData,
    type_id::TypeId, type_pack_id::TypePackId, type_variant::TypeVariant,
  },
};
impl ConstraintGenerator {
  // ConstraintGenerator::visit(const ScopePtr&, AstStatTypeAlias*)
  // (ConstraintGenerator.cpp).
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_scope_ptr_ast_stat_type_alias(
    &mut self,
    scope: &ScopePtr,
    alias: *mut AstStatTypeAlias,
  ) -> ControlFlow {
    let alias_ref = unsafe { &*alias };

    let alias_name = unsafe { CStr::from_ptr(alias_ref.name.value) };
    if alias_name.to_bytes() == b"%error-id%" {
      return ControlFlow::None;
    }

    if alias_name.to_bytes() == b"typeof" {
      self.report_error(
        alias_ref.base.base.location,
        TypeErrorData::ReservedIdentifier(ReservedIdentifier::new(String::from("typeof"))),
      );
      return ControlFlow::None;
    }

    let name_key: String = alias_name.to_string_lossy().into_owned();
    let scope_raw = scope.as_ref() as *const Scope as *mut Scope;
    unsafe {
      (*scope_raw)
        .type_alias_locations
        .insert(name_key.clone(), alias_ref.base.base.location);
      (*scope_raw)
        .type_alias_name_locations
        .insert(name_key.clone(), alias_ref.name_location);
    }

    let defn_scope_opt = self
      .ast_type_alias_defining_scopes
      .find(&(alias as *const AstStatTypeAlias))
      .cloned();

    // These will be undefined if the alias was a duplicate definition, in which
    // case we just skip over it.
    let binding_it = unsafe {
      if alias_ref.exported {
        (*scope_raw).exported_type_bindings.get(&name_key).cloned()
      } else {
        (*scope_raw).private_type_bindings.get(&name_key).cloned()
      }
    };

    let (fun, defn_scope) = match (binding_it, defn_scope_opt) {
      (Some(b), Some(Some(s))) => (b, s),
      _ => return ControlFlow::None,
    };
    let defn_scope_raw = defn_scope.as_ref() as *const Scope as *mut Scope;

    unsafe { self.resolve_generic_default_parameters(defn_scope_raw, alias, &fun) };

    let ty = self.resolve_type(
      defn_scope_raw,
      alias_ref.type_ptr,
      /* in_type_arguments */ false,
      /* replace_error_with_fresh */ false,
      Polarity::Positive,
    );

    let alias_ty = fun.r#type();
    // get_type_id 已 safe；对照 C++ `LUAU_ASSERT(is<BlockedType>(aliasTy))`
    LUAU_ASSERT!(get_type_id::<BlockedType>(alias_ty,).is_some());

    if occurs_check_type_id_type_id(alias_ty, ty) {
      unsafe {
        let mutable_alias = as_mutable_type_id(alias_ty);
        (*mutable_alias).ty = TypeVariant::Bound((*self.builtin_types).any_type);
      }
      self.report_error(
        alias_ref.name_location,
        TypeErrorData::OccursCheckFailed(OccursCheckFailed::default()),
      );
    } else {
      unsafe {
        let mutable_alias = as_mutable_type_id(alias_ty);
        (*mutable_alias).ty = TypeVariant::Bound(ty);
      }
    }

    let type_params: Vec<TypeId> = self
      .create_generics(
        &defn_scope,
        alias_ref.generics,
        /* use_cache */ true,
        /* add_types */ false,
      )
      .into_iter()
      .map(|(_, def)| def.ty)
      .collect();

    let type_pack_params: Vec<TypePackId> = self
      .create_generic_packs(
        &defn_scope,
        alias_ref.generic_packs,
        /* use_cache */ true,
        /* add_types */ false,
      )
      .into_iter()
      .map(|(_, def)| def.tp)
      .collect();

    self.add_constraint_scope_ptr_location_constraint_v(
      scope,
      unsafe { (*alias_ref.type_ptr).base.location },
      ConstraintV::Name(NameConstraint {
        named_type: ty,
        name: name_key,
        synthetic: false,
        type_parameters: type_params,
        type_pack_parameters: type_pack_params,
      }),
    );

    ControlFlow::None
  }
}
