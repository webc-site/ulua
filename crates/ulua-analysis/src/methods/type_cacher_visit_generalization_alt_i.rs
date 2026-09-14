use alloc::vec::Vec;
use core::ffi::c_void;

use crate::{
  functions::{
    follow_type::follow_type_id, follow_type_pack::follow_type_pack_id,
    get_type_alt_j::get_type_id, get_type_pack::get_type_pack_id,
  },
  records::{
    any_type::AnyType, blocked_type::BlockedType, blocked_type_pack::BlockedTypePack,
    extern_type::ExternType, free_type::FreeType, free_type_pack::FreeTypePack,
    function_type::FunctionType, generic_type::GenericType, generic_type_pack::GenericTypePack,
    intersection_type::IntersectionType, metatable_type::MetatableType,
    negation_type::NegationType, never_type::NeverType, no_refine_type::NoRefineType,
    pending_expansion_type::PendingExpansionType, primitive_type::PrimitiveType,
    singleton_type::SingletonType, table_type::TableType, type_cacher::TypeCacher,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack, type_pack::TypePack,
    union_type::UnionType, unknown_type::UnknownType, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    bound_type_pack::BoundTypePack, error_type::ErrorType, error_type_pack::ErrorTypePack,
    type_id::TypeId, type_pack_id::TypePackId,
  },
};
impl TypeCacher {
  /// C++ `bool TypeCacher::visit(TypeId ty, const FunctionType& ft)`
  /// (Generalization.cpp:337-388).
  pub fn visit_type_id_function_type(&mut self, ty: TypeId, ft: &FunctionType) -> bool {
    if self.is_cached(ty) || self.is_uncacheable_type_id(ty) {
      return false;
    }

    cacher_traverse_type_pack_id(self, ft.arg_types);
    cacher_traverse_type_pack_id(self, ft.ret_types);
    for &r#gen in &ft.generics {
      cacher_traverse_type_id(self, r#gen);
    }

    let mut uncacheable = false;

    if self.is_uncacheable_type_pack_id(ft.arg_types)
      || self.is_uncacheable_type_pack_id(ft.ret_types)
    {
      uncacheable = true;
    }

    // for (TypeId argTy : ft.argTypes) — iterate the flattened arg pack.
    for arg_ty in flatten_type_pack(ft.arg_types) {
      if self.is_uncacheable_type_id(arg_ty) {
        uncacheable = true;
        break;
      }
    }

    for ret_ty in flatten_type_pack(ft.ret_types) {
      if self.is_uncacheable_type_id(ret_ty) {
        uncacheable = true;
        break;
      }
    }

    for &g in &ft.generics {
      if self.is_uncacheable_type_id(g) {
        uncacheable = true;
        break;
      }
    }

    if uncacheable {
      self.mark_uncacheable_type_id(ty);
    } else {
      self.cache(ty);
    }

    false
  }
}

/// C++ range-for over a `TypePackId` (`for (TypeId x : pack)`): walks the head
/// chain following `Bound`/`TypePack` tails, yielding each head element.
fn flatten_type_pack(tp: TypePackId) -> Vec<TypeId> {
  let mut out = Vec::new();
  let mut cur = unsafe { follow_type_pack_id(tp) };
  while let Some(pack) = get_type_pack_id::<TypePack>(cur) {
    for &h in &pack.head {
      out.push(h);
    }
    match pack.tail {
      Some(tail) => cur = unsafe { follow_type_pack_id(tail) },
      None => break,
    }
  }
  out
}

/// C++ `TypeOnceVisitor::traverse(TypeId)` for the `TypeCacher`. The cacher's
/// overrides live as inherent methods here; this routes a followed type to the
/// correct typed `visit`. (`follow` already skips `Bound` because the visitor
/// is constructed with `skipBoundTypes = true`.)
pub(crate) fn cacher_traverse_type_id(this: &mut TypeCacher, ty: TypeId) {
  let ty = follow_type_id(ty);
  let seen_key = ty as *mut c_void;
  if this.base.base.seen.contains(&seen_key) {
    return;
  }
  this.base.base.seen.insert(seen_key);

  if let Some(v) = get_type_id::<FreeType>(ty) {
    this.visit_type_id_free_type(ty, v);
  } else if let Some(v) = get_type_id::<GenericType>(ty) {
    this.visit_type_id_generic_type(ty, v);
  } else if let Some(v) = get_type_id::<ErrorType>(ty) {
    this.visit_type_id_error_type(ty, v);
  } else if let Some(v) = get_type_id::<PrimitiveType>(ty) {
    this.visit_type_id_primitive_type(ty, v);
  } else if let Some(v) = get_type_id::<SingletonType>(ty) {
    this.visit_type_id_singleton_type(ty, v);
  } else if let Some(v) = get_type_id::<BlockedType>(ty) {
    this.visit_type_id_blocked_type(ty, v);
  } else if let Some(v) = get_type_id::<PendingExpansionType>(ty) {
    this.visit_type_id_pending_expansion_type(ty, v);
  } else if let Some(v) = get_type_id::<FunctionType>(ty) {
    this.visit_type_id_function_type(ty, v);
  } else if let Some(v) = get_type_id::<TableType>(ty) {
    this.visit_type_id_table_type(ty, v);
  } else if let Some(v) = get_type_id::<MetatableType>(ty) {
    this.visit_type_id_metatable_type(ty, v);
  } else if let Some(v) = get_type_id::<ExternType>(ty) {
    this.visit_type_id_extern_type(ty, v);
  } else if let Some(v) = get_type_id::<AnyType>(ty) {
    this.visit_type_id_any_type(ty, v);
  } else if let Some(v) = get_type_id::<NoRefineType>(ty) {
    this.visit_type_id_no_refine_type(ty, v);
  } else if let Some(v) = get_type_id::<UnionType>(ty) {
    this.visit_type_id_union_type(ty, v);
  } else if let Some(v) = get_type_id::<IntersectionType>(ty) {
    this.visit_type_id_intersection_type(ty, v);
  } else if let Some(v) = get_type_id::<UnknownType>(ty) {
    this.visit_type_id_unknown_type(ty, v);
  } else if let Some(v) = get_type_id::<NeverType>(ty) {
    this.visit_type_id_never_type(ty, v);
  } else if let Some(v) = get_type_id::<NegationType>(ty) {
    this.visit_type_id_negation_type(ty, v);
  } else if let Some(v) = get_type_id::<TypeFunctionInstanceType>(ty) {
    this.visit_type_id_type_function_instance_type(ty, v);
  }
  // Lazy / unhandled variants: the cacher has no override and never
  // legitimately reaches them here.
}

/// C++ `TypeOnceVisitor::traverse(TypePackId)` for the `TypeCacher`.
pub(crate) fn cacher_traverse_type_pack_id(this: &mut TypeCacher, tp: TypePackId) {
  let tp = unsafe { follow_type_pack_id(tp) };
  let seen_key = tp as *mut c_void;
  if this.base.base.seen.contains(&seen_key) {
    return;
  }
  this.base.base.seen.insert(seen_key);

  if let Some(v) = get_type_pack_id::<FreeTypePack>(tp) {
    this.visit_type_pack_id_free_type_pack(tp, v);
  } else if let Some(v) = get_type_pack_id::<GenericTypePack>(tp) {
    this.visit_type_pack_id_generic_type_pack(tp, v);
  } else if let Some(v) = get_type_pack_id::<ErrorTypePack>(tp) {
    this.visit_type_pack_id_error_type_pack(tp, v);
  } else if let Some(v) = get_type_pack_id::<VariadicTypePack>(tp) {
    this.visit_type_pack_id_variadic_type_pack(tp, v);
  } else if let Some(v) = get_type_pack_id::<BlockedTypePack>(tp) {
    this.visit_type_pack_id_blocked_type_pack(tp, v);
  } else if let Some(v) = get_type_pack_id::<TypeFunctionInstanceTypePack>(tp) {
    this.visit_type_pack_id_type_function_instance_type_pack(tp, v);
  } else if let Some(v) = get_type_pack_id::<BoundTypePack>(tp) {
    this.visit_type_pack_id_bound_type_pack(tp, v);
  } else if let Some(v) = get_type_pack_id::<TypePack>(tp) {
    this.visit_type_pack_id_type_pack(tp, v);
  }
}
