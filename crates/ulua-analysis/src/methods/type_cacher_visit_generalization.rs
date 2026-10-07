use alloc::vec::Vec;

use ulua_common::macros::luau_assert::{LUAU_ASSERT, LUAU_UNREACHABLE};

use crate::{
  enums::table_state::TableState,
  functions::{follow_type, follow_type_pack, get_type, get_type_pack},
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
    visit_key::VisitKey,
  },
  type_aliases::{
    bound_type_pack::BoundTypePack, error_type::ErrorType, error_type_pack::ErrorTypePack,
    type_id::TypeId, type_pack_id::TypePackId,
  },
};

impl TypeCacher {
  pub fn visit_type_id(&mut self, _ty: TypeId) -> bool {
    LUAU_ASSERT!(false);
    LUAU_UNREACHABLE!();
  }

  pub fn visit_type_pack_id_type_function_instance_type_pack(
    &mut self,
    tp: TypePackId,
    _tfitp: &TypeFunctionInstanceTypePack,
  ) -> bool {
    self.mark_uncacheable_type_pack_id(tp);
    false
  }

  /// C++ `bool TypeCacher::visit(TypePackId tp, const BoundTypePack& btp)`
  /// (Generalization.cpp:631-637).
  pub fn visit_type_pack_id_bound_type_pack(
    &mut self,
    tp: TypePackId,
    btp: &BoundTypePack,
  ) -> bool {
    cacher_traverse_type_pack_id(self, btp.bound_to);
    if self.is_uncacheable_type_pack_id(btp.bound_to) {
      self.mark_uncacheable_type_pack_id(tp);
    }
    false
  }

  pub fn visit_type_pack_id_type_pack(&mut self, tp: TypePackId, typ: &TypePack) -> bool {
    let mut uncacheable = false;
    for &ty in &typ.head {
      let followed = follow_type::follow(ty);
      cacher_traverse_type_id(self, followed);
      uncacheable |= self.is_uncacheable_type_id(followed);
    }
    if let Some(tail) = typ.tail {
      let followed = follow_type_pack::follow(tail);
      cacher_traverse_type_pack_id(self, followed);
      uncacheable |= self.is_uncacheable_type_pack_id(followed);
    }
    if uncacheable {
      self.mark_uncacheable_type_pack_id(tp);
    }
    false
  }

  /// C++ `bool TypeCacher::visit(TypeId ty, const FreeType& ft)`
  /// (Generalization.cpp:285-299).
  pub fn visit_type_id_free_type(&mut self, ty: TypeId, ft: &FreeType) -> bool {
    // Free types are never cacheable.
    LUAU_ASSERT!(!self.is_cached(ty));

    if !self.is_uncacheable_type_id(ty) {
      cacher_traverse_type_id(self, ft.lower_bound);
      cacher_traverse_type_id(self, ft.upper_bound);

      self.mark_uncacheable_type_id(ty);
    }

    false
  }

  pub fn visit_type_id_generic_type(&mut self, ty: TypeId, _gt: &GenericType) -> bool {
    self.cache(ty);
    false
  }

  pub fn visit_type_id_error_type(&mut self, ty: TypeId, _et: &ErrorType) -> bool {
    self.cache(ty);
    false
  }

  pub fn visit_type_id_primitive_type(&mut self, ty: TypeId, _pt: &PrimitiveType) -> bool {
    self.cache(ty);
    false
  }

  pub fn visit_type_id_singleton_type(&mut self, ty: TypeId, _st: &SingletonType) -> bool {
    self.cache(ty);
    false
  }

  pub fn visit_type_id_blocked_type(&mut self, ty: TypeId, _bt: &BlockedType) -> bool {
    self.mark_uncacheable_type_id(ty);
    false
  }

  pub fn visit_type_id_pending_expansion_type(
    &mut self,
    ty: TypeId,
    _pet: &PendingExpansionType,
  ) -> bool {
    self.mark_uncacheable_type_id(ty);
    false
  }

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
  let mut cur = follow_type_pack::follow(tp);
  while let Some(pack) = get_type_pack::get::<TypePack>(cur) {
    for &h in &pack.head {
      out.push(h);
    }
    match pack.tail {
      Some(tail) => cur = follow_type_pack::follow(tail),
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
  let ty = follow_type::follow(ty);
  let seen_key = VisitKey::from_ptr(ty);
  if this.base.base.seen.contains(&seen_key) {
    return;
  }
  this.base.base.seen.insert(seen_key);

  if let Some(v) = get_type::get::<FreeType>(ty) {
    this.visit_type_id_free_type(ty, v);
  } else if let Some(v) = get_type::get::<GenericType>(ty) {
    this.visit_type_id_generic_type(ty, v);
  } else if let Some(v) = get_type::get::<ErrorType>(ty) {
    this.visit_type_id_error_type(ty, v);
  } else if let Some(v) = get_type::get::<PrimitiveType>(ty) {
    this.visit_type_id_primitive_type(ty, v);
  } else if let Some(v) = get_type::get::<SingletonType>(ty) {
    this.visit_type_id_singleton_type(ty, v);
  } else if let Some(v) = get_type::get::<BlockedType>(ty) {
    this.visit_type_id_blocked_type(ty, v);
  } else if let Some(v) = get_type::get::<PendingExpansionType>(ty) {
    this.visit_type_id_pending_expansion_type(ty, v);
  } else if let Some(v) = get_type::get::<FunctionType>(ty) {
    this.visit_type_id_function_type(ty, v);
  } else if let Some(v) = get_type::get::<TableType>(ty) {
    this.visit_type_id_table_type(ty, v);
  } else if let Some(v) = get_type::get::<MetatableType>(ty) {
    this.visit_type_id_metatable_type(ty, v);
  } else if let Some(v) = get_type::get::<ExternType>(ty) {
    this.visit_type_id_extern_type(ty, v);
  } else if let Some(v) = get_type::get::<AnyType>(ty) {
    this.visit_type_id_any_type(ty, v);
  } else if let Some(v) = get_type::get::<NoRefineType>(ty) {
    this.visit_type_id_no_refine_type(ty, v);
  } else if let Some(v) = get_type::get::<UnionType>(ty) {
    this.visit_type_id_union_type(ty, v);
  } else if let Some(v) = get_type::get::<IntersectionType>(ty) {
    this.visit_type_id_intersection_type(ty, v);
  } else if let Some(v) = get_type::get::<UnknownType>(ty) {
    this.visit_type_id_unknown_type(ty, v);
  } else if let Some(v) = get_type::get::<NeverType>(ty) {
    this.visit_type_id_never_type(ty, v);
  } else if let Some(v) = get_type::get::<NegationType>(ty) {
    this.visit_type_id_negation_type(ty, v);
  } else if let Some(v) = get_type::get::<TypeFunctionInstanceType>(ty) {
    this.visit_type_id_type_function_instance_type(ty, v);
  }
  // Lazy / unhandled variants: the cacher has no override and never
  // legitimately reaches them here.
}

/// C++ `TypeOnceVisitor::traverse(TypePackId)` for the `TypeCacher`.
pub(crate) fn cacher_traverse_type_pack_id(this: &mut TypeCacher, tp: TypePackId) {
  let tp = follow_type_pack::follow(tp);
  let seen_key = VisitKey::from_ptr(tp);
  if this.base.base.seen.contains(&seen_key) {
    return;
  }
  this.base.base.seen.insert(seen_key);

  if let Some(v) = get_type_pack::get::<FreeTypePack>(tp) {
    this.visit_type_pack_id_free_type_pack(tp, v);
  } else if let Some(v) = get_type_pack::get::<GenericTypePack>(tp) {
    this.visit_type_pack_id_generic_type_pack(tp, v);
  } else if let Some(v) = get_type_pack::get::<ErrorTypePack>(tp) {
    this.visit_type_pack_id_error_type_pack(tp, v);
  } else if let Some(v) = get_type_pack::get::<VariadicTypePack>(tp) {
    this.visit_type_pack_id_variadic_type_pack(tp, v);
  } else if let Some(v) = get_type_pack::get::<BlockedTypePack>(tp) {
    this.visit_type_pack_id_blocked_type_pack(tp, v);
  } else if let Some(v) = get_type_pack::get::<TypeFunctionInstanceTypePack>(tp) {
    this.visit_type_pack_id_type_function_instance_type_pack(tp, v);
  } else if let Some(v) = get_type_pack::get::<BoundTypePack>(tp) {
    this.visit_type_pack_id_bound_type_pack(tp, v);
  } else if let Some(v) = get_type_pack::get::<TypePack>(tp) {
    this.visit_type_pack_id_type_pack(tp, v);
  }
}

impl TypeCacher {
  pub fn visit_type_id_table_type(&mut self, ty: TypeId, tt: &TableType) -> bool {
    if self.is_cached(ty) || self.is_uncacheable_type_id(ty) {
      return false;
    }

    if let Some(bound) = tt.bound_to {
      let followed = follow_type::follow(bound);
      cacher_traverse_type_id(self, followed);
      if self.is_uncacheable_type_id(followed) {
        self.mark_uncacheable_type_id(ty);
        return false;
      }
    }

    let mut uncacheable = matches!(tt.state, TableState::Free | TableState::Unsealed);

    for prop in tt.props.values() {
      if let Some(read) = prop.read_ty {
        let followed = follow_type::follow(read);
        cacher_traverse_type_id(self, followed);
        if self.is_uncacheable_type_id(followed) {
          uncacheable = true;
        }
      }
      if let Some(write) = prop.write_ty
        && Some(write) != prop.read_ty
      {
        let followed = follow_type::follow(write);
        cacher_traverse_type_id(self, followed);
        if self.is_uncacheable_type_id(followed) {
          uncacheable = true;
        }
      }
    }

    if let Some(indexer) = &tt.indexer {
      let idx = follow_type::follow(indexer.index_type);
      let res = follow_type::follow(indexer.index_result_type);
      cacher_traverse_type_id(self, idx);
      cacher_traverse_type_id(self, res);
      if self.is_uncacheable_type_id(idx) || self.is_uncacheable_type_id(res) {
        uncacheable = true;
      }
    }

    if uncacheable {
      self.mark_uncacheable_type_id(ty);
    } else {
      self.cache(ty);
    }

    false
  }

  pub fn visit_type_id_metatable_type(&mut self, ty: TypeId, mtv: &MetatableType) -> bool {
    let tbl = follow_type::follow(mtv.table());
    let mt = follow_type::follow(mtv.metatable());
    cacher_traverse_type_id(self, tbl);
    cacher_traverse_type_id(self, mt);
    if self.is_uncacheable_type_id(tbl) || self.is_uncacheable_type_id(mt) {
      self.mark_uncacheable_type_id(ty);
    } else {
      self.cache(ty);
    }
    false
  }

  pub fn visit_type_id_extern_type(&mut self, ty: TypeId, _et: &ExternType) -> bool {
    self.cache(ty);
    false
  }

  pub fn visit_type_id_any_type(&mut self, ty: TypeId, _at: &AnyType) -> bool {
    self.cache(ty);
    false
  }

  pub fn visit_type_id_no_refine_type(&mut self, ty: TypeId, _nrt: &NoRefineType) -> bool {
    self.cache(ty);
    false
  }

  pub fn visit_type_id_union_type(&mut self, ty: TypeId, ut: &UnionType) -> bool {
    if self.is_uncacheable_type_id(ty) || self.is_cached(ty) {
      return false;
    }

    let mut uncacheable = false;
    for &part in &ut.options {
      let followed = follow_type::follow(part);
      cacher_traverse_type_id(self, followed);
      uncacheable |= self.is_uncacheable_type_id(followed);
    }

    if uncacheable {
      self.mark_uncacheable_type_id(ty);
    } else {
      self.cache(ty);
    }

    false
  }

  pub fn visit_type_id_intersection_type(&mut self, ty: TypeId, it: &IntersectionType) -> bool {
    if self.is_uncacheable_type_id(ty) || self.is_cached(ty) {
      return false;
    }

    let mut uncacheable = false;
    for &part in &it.parts {
      let followed = follow_type::follow(part);
      cacher_traverse_type_id(self, followed);
      uncacheable |= self.is_uncacheable_type_id(followed);
    }

    if uncacheable {
      self.mark_uncacheable_type_id(ty);
    } else {
      self.cache(ty);
    }

    false
  }

  pub fn visit_type_id_unknown_type(&mut self, ty: TypeId, _ut: &UnknownType) -> bool {
    self.cache(ty);
    false
  }

  pub fn visit_type_id_never_type(&mut self, ty: TypeId, _nt: &NeverType) -> bool {
    self.cache(ty);
    false
  }

  pub fn visit_type_id_negation_type(&mut self, ty: TypeId, nt: &NegationType) -> bool {
    if !self.is_cached(ty) && !self.is_uncacheable_type_id(ty) {
      let followed = follow_type::follow(nt.ty);
      cacher_traverse_type_id(self, followed);

      if self.is_uncacheable_type_id(followed) {
        self.mark_uncacheable_type_id(ty);
      } else {
        self.cache(ty);
      }
    }
    false
  }

  pub fn visit_type_id_type_function_instance_type(
    &mut self,
    ty: TypeId,
    tfit: &TypeFunctionInstanceType,
  ) -> bool {
    if self.is_cached(ty) || self.is_uncacheable_type_id(ty) {
      return false;
    }

    let mut uncacheable = false;
    for &arg in &tfit.type_arguments {
      let followed = follow_type::follow(arg);
      cacher_traverse_type_id(self, followed);
      if self.is_uncacheable_type_id(followed) {
        uncacheable = true;
      }
    }

    for &pack in &tfit.pack_arguments {
      let followed = follow_type_pack::follow(pack);
      cacher_traverse_type_pack_id(self, followed);
      if self.is_uncacheable_type_pack_id(followed) {
        uncacheable = true;
      }
    }

    if uncacheable {
      self.mark_uncacheable_type_id(ty);
    } else {
      self.cache(ty);
    }

    false
  }

  pub fn visit_type_pack_id(&mut self, _tp: TypePackId) -> bool {
    LUAU_ASSERT!(false);
    LUAU_UNREACHABLE!();
  }

  pub fn visit_type_pack_id_free_type_pack(&mut self, tp: TypePackId, _ftp: &FreeTypePack) -> bool {
    self.mark_uncacheable_type_pack_id(tp);
    false
  }

  pub fn visit_type_pack_id_generic_type_pack(
    &mut self,
    _tp: TypePackId,
    _gtp: &GenericTypePack,
  ) -> bool {
    true
  }

  pub fn visit_type_pack_id_error_type_pack(
    &mut self,
    _tp: TypePackId,
    _etp: &ErrorTypePack,
  ) -> bool {
    true
  }

  pub fn visit_type_pack_id_variadic_type_pack(
    &mut self,
    tp: TypePackId,
    vtp: &VariadicTypePack,
  ) -> bool {
    if self.is_uncacheable_type_pack_id(tp) {
      return false;
    }

    let followed = follow_type::follow(vtp.ty);
    cacher_traverse_type_id(self, followed);

    if self.is_uncacheable_type_id(followed) {
      self.mark_uncacheable_type_pack_id(tp);
    }

    false
  }

  pub fn visit_type_pack_id_blocked_type_pack(
    &mut self,
    tp: TypePackId,
    _btp: &BlockedTypePack,
  ) -> bool {
    self.mark_uncacheable_type_pack_id(tp);
    false
  }
}
