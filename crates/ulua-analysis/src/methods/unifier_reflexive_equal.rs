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
    let a = self.log.follow_type_id(a);
    let b = self.log.follow_type_id(b);
    if a == b {
      return true;
    }
    if let (Some(au), Some(bu)) = (
      self.log.txn_log_get::<UnionType, TypeId>(a),
      self.log.txn_log_get::<UnionType, TypeId>(b),
    ) {
      return au.options.len() == bu.options.len()
        && au
          .options
          .iter()
          .zip(bu.options.iter())
          .all(|(x, y)| self.reflexive_equal_type_id(*x, *y, depth - 1));
    }

    if let (Some(ai), Some(bi)) = (
      self.log.txn_log_get::<IntersectionType, TypeId>(a),
      self.log.txn_log_get::<IntersectionType, TypeId>(b),
    ) {
      return ai.parts.len() == bi.parts.len()
        && ai
          .parts
          .iter()
          .zip(bi.parts.iter())
          .all(|(x, y)| self.reflexive_equal_type_id(*x, *y, depth - 1));
    }

    if let (Some(af), Some(bf)) = (
      self.log.txn_log_get::<FunctionType, TypeId>(a),
      self.log.txn_log_get::<FunctionType, TypeId>(b),
    ) {
      return self.reflexive_equal_type_pack_id(af.arg_types, bf.arg_types, depth - 1)
        && self.reflexive_equal_type_pack_id(af.ret_types, bf.ret_types, depth - 1);
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
    let a = self.log.follow_type_pack_id(a);
    let b = self.log.follow_type_pack_id(b);
    if a == b {
      return true;
    }
    if let (Some(ap), Some(bp)) = (
      self.log.txn_log_get::<TypePack, TypePackId>(a),
      self.log.txn_log_get::<TypePack, TypePackId>(b),
    ) {
      if ap.head.len() != bp.head.len() {
        return false;
      }
      if !ap
        .head
        .iter()
        .zip(bp.head.iter())
        .all(|(x, y)| self.reflexive_equal_type_id(*x, *y, depth - 1))
      {
        return false;
      }
      return match (&ap.tail, &bp.tail) {
        (None, None) => true,
        (Some(ta), Some(tb)) => self.reflexive_equal_type_pack_id(*ta, *tb, depth - 1),
        _ => false,
      };
    }
    false
  }
}
