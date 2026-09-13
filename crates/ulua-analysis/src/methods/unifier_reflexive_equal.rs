//! Reflexive structural-equality fast-path for the unifier.
//!
//! C++ Luau keeps the `if (super_ty == sub_ty) return;` pointer fast-path in
//! `tryUnify_` cheap because type-alias unions (and the function/pack types
//! built over them) share a single TypeId across every use, so the pointer
//! check fires pervasively. Our port does NOT pointer-share alias-derived
//! composite types, so structurally-identical `Color`/function/pack values sit
//! at distinct pointers and the pointer check misses — forcing the full
//! element-by-element walk on every curried use and blowing the iteration
//! limit on pathological inputs (`luau_subtyping_is_np_hard`).
//!
//! Unifying a type/pack with a structurally-identical type/pack always succeeds
//! by reflexivity, regardless of variance, so short-circuiting here is sound.
//! The walk is log-aware (uses `self.log.follow*`) and depth-bounded; on hitting
//! the bound it conservatively returns `false` and the normal unifier runs.
use crate::{
  records::{
    function_type::FunctionType, intersection_type::IntersectionType, type_pack::TypePack,
    unifier::Unifier, union_type::UnionType,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Unifier {
  pub(crate) fn reflexive_equal_type_id(&self, a: TypeId, b: TypeId, depth: u32) -> bool {
    if depth == 0 {
      return false;
    }
    let a = unsafe { self.log.follow_type_id(a) };
    let b = unsafe { self.log.follow_type_id(b) };
    if a == b {
      return true;
    }
    unsafe {
      let au = self.log.txn_log_get::<UnionType, TypeId>(a);
      let bu = self.log.txn_log_get::<UnionType, TypeId>(b);
      if !au.is_null() && !bu.is_null() {
        let ao = &(*au).options;
        let bo = &(*bu).options;
        return ao.len() == bo.len()
          && ao
            .iter()
            .zip(bo.iter())
            .all(|(x, y)| self.reflexive_equal_type_id(*x, *y, depth - 1));
      }

      let ai = self.log.txn_log_get::<IntersectionType, TypeId>(a);
      let bi = self.log.txn_log_get::<IntersectionType, TypeId>(b);
      if !ai.is_null() && !bi.is_null() {
        let ap = &(*ai).parts;
        let bp = &(*bi).parts;
        return ap.len() == bp.len()
          && ap
            .iter()
            .zip(bp.iter())
            .all(|(x, y)| self.reflexive_equal_type_id(*x, *y, depth - 1));
      }

      let af = self.log.txn_log_get::<FunctionType, TypeId>(a);
      let bf = self.log.txn_log_get::<FunctionType, TypeId>(b);
      if !af.is_null() && !bf.is_null() {
        return self.reflexive_equal_type_pack_id((*af).arg_types, (*bf).arg_types, depth - 1)
          && self.reflexive_equal_type_pack_id((*af).ret_types, (*bf).ret_types, depth - 1);
      }
    }
    false
  }

  pub(crate) fn reflexive_equal_type_pack_id(
    &self,
    a: TypePackId,
    b: TypePackId,
    depth: u32,
  ) -> bool {
    if depth == 0 {
      return false;
    }
    let a = unsafe { self.log.follow_type_pack_id(a) };
    let b = unsafe { self.log.follow_type_pack_id(b) };
    if a == b {
      return true;
    }
    unsafe {
      let ap = self.log.txn_log_get::<TypePack, TypePackId>(a);
      let bp = self.log.txn_log_get::<TypePack, TypePackId>(b);
      if !ap.is_null() && !bp.is_null() {
        let ah = &(*ap).head;
        let bh = &(*bp).head;
        if ah.len() != bh.len() {
          return false;
        }
        if !ah
          .iter()
          .zip(bh.iter())
          .all(|(x, y)| self.reflexive_equal_type_id(*x, *y, depth - 1))
        {
          return false;
        }
        return match (&(*ap).tail, &(*bp).tail) {
          (None, None) => true,
          (Some(ta), Some(tb)) => self.reflexive_equal_type_pack_id(*ta, *tb, depth - 1),
          _ => false,
        };
      }
    }
    false
  }
}
