//! Source: `Analysis/src/Unifier2.cpp:669-807` — `Unifier2::unify_(TypePackId, TypePackId)`,
//! the type-pack core of new-solver unification.

use alloc::vec::Vec;
use core::{
  cmp::{max, min},
  ffi::c_void,
  mem::zeroed,
  ptr::{NonNull, null},
};

use ulua_common::{
  FFlag, FInt, macros::luau_assert::LUAU_ASSERT, records::dense_hash_set::DenseHashSet,
};

use crate::{
  enums::{occurs_check_result::OccursCheckResult, unify_result::UnifyResult},
  functions::{
    as_mutable_type_pack::as_mutable_type_pack_id, emplace_type_pack::emplace_type_pack,
    extend_type_pack::extend_type_pack, flatten_type_pack::flatten_type_pack_id,
    follow_type_pack::follow_type_pack_id, get_type_pack::get_type_pack_id,
    is_irresolvable_unifier_2_alt_b::is_irresolvable,
    occurs_check_type_utils_alt_b::occurs_check_type_pack_id_type_pack_id,
  },
  records::{
    free_type_pack::FreeTypePack,
    non_exceptional_recursion_limiter::NonExceptionalRecursionLimiter,
    pack_subtype_constraint::PackSubtypeConstraint, replacer::Replacer, type_pack::TypePack,
    unifier_2::Unifier2,
  },
  type_aliases::{
    constraint_v::ConstraintV, type_id::TypeId, type_pack_id::TypePackId,
    type_pack_variant::TypePackVariant,
  },
};
impl Unifier2 {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn unify_type_pack_id_type_pack_id(
    &mut self,
    mut sub_tp: TypePackId,
    mut super_tp: TypePackId,
  ) -> UnifyResult {
    if FInt::LuauTypeInferIterationLimit.get() > 0
      && self.iteration_count >= FInt::LuauTypeInferIterationLimit.get()
    {
      return UnifyResult::TooComplex;
    }

    self.iteration_count += 1;

    if FFlag::LuauLimitUnificationRecursion.get() {
      // ++(*count) — mirror the C++ NonExceptionalRecursionLimiter (RecursionCounter ctor).
      self.recursion_count += 1;
      let mut nerl = NonExceptionalRecursionLimiter {
        base: unsafe { zeroed() },
        native_stack_guard: unsafe { zeroed() },
      };
      nerl.non_exceptional_recursion_limiter_non_exceptional_recursion_limiter(
        &mut self.recursion_count,
      );
      if !nerl.is_ok(self.recursion_limit) {
        return UnifyResult::TooComplex;
      }
    }

    sub_tp = unsafe { follow_type_pack_id(sub_tp) };
    super_tp = unsafe { follow_type_pack_id(super_tp) };

    if self.seen_type_pack_pairings.contains(&(sub_tp, super_tp)) {
      return UnifyResult::Ok;
    }
    self.seen_type_pack_pairings.insert((sub_tp, super_tp));

    if sub_tp == super_tp {
      return UnifyResult::Ok;
    }

    // FIXME: CLI-188000: If we are _directly_ given a free type, we must
    // eagerly emplace it. Otherwise, later, we may generalize the underlying
    // free types incorrectly.
    if !get_type_pack_id::<FreeTypePack>(sub_tp).is_none() {
      return self.emplace_free_type_pack(sub_tp, super_tp);
    }

    if !get_type_pack_id::<FreeTypePack>(super_tp).is_none() {
      return self.emplace_free_type_pack(super_tp, sub_tp);
    }

    let sub_len = flatten_type_pack_id(sub_tp).0.len();
    let super_len = flatten_type_pack_id(super_tp).0.len();
    let max_length = max(sub_len, super_len);

    let arena = self.arena.as_ptr();
    let builtin_types_ptr = self.builtin_types.as_ptr();

    let sub_extended = unsafe {
      extend_type_pack(
        &mut *arena,
        builtin_types_ptr,
        sub_tp,
        max_length,
        Vec::new(),
      )
    };
    let super_extended = unsafe {
      extend_type_pack(
        &mut *arena,
        builtin_types_ptr,
        super_tp,
        max_length,
        Vec::new(),
      )
    };

    let sub_types = sub_extended.head;
    let sub_tail = sub_extended.tail;
    let super_types = super_extended.head;
    let super_tail = super_extended.tail;

    let limit = min(sub_types.len(), super_types.len());
    for (sub, sup) in sub_types.iter().zip(super_types.iter()).take(limit) {
      self.unify_type_id_type_id(*sub, *sup);
    }

    // At this point it should be the case that either:
    // - `subTypes` now has all of its types unified, and we are down to its tail
    // - `superTypes` now has all of its types unified, and we are down to its tail

    if sub_tail.is_none() && super_tail.is_none() {
      // If both types are missing a tail, we've done all we can.
      return UnifyResult::Ok;
    }

    // It should be the case that exclusively one of these packs can be reduced
    // to their tail for the rest of the function.
    if limit < sub_types.len() {
      LUAU_ASSERT!(limit == super_types.len());
      // If we have extra subtypes left over, construct a new type pack
      let new_sub_head: Vec<TypeId> = sub_types[super_types.len()..].to_vec();
      sub_tp = unsafe { &mut *arena }.add_type_pack_t(TypePack {
        head: new_sub_head,
        tail: sub_tail,
      });
      super_tp = self.maybe_replace_tail(super_tail);
    } else if limit < super_types.len() {
      LUAU_ASSERT!(limit == sub_types.len() && limit < super_types.len());
      // If we have extra subtypes left over, construct a new type pack
      let new_super_head: Vec<TypeId> = super_types[sub_types.len()..].to_vec();
      super_tp = unsafe { &mut *arena }.add_type_pack_t(TypePack {
        head: new_super_head,
        tail: super_tail,
      });
      sub_tp = self.maybe_replace_tail(sub_tail);
    } else {
      sub_tp = self.maybe_replace_tail(sub_tail);
      super_tp = self.maybe_replace_tail(super_tail);
    }

    if is_irresolvable(sub_tp) || is_irresolvable(super_tp) {
      if !self.uninhabited_type_functions.is_null()
        && unsafe {
          (*self.uninhabited_type_functions).contains(&(sub_tp as *const c_void))
            || (*self.uninhabited_type_functions).contains(&(super_tp as *const c_void))
        }
      {
        return UnifyResult::Ok;
      }

      self
        .incomplete_subtypes
        .push(ConstraintV::PackSubtype(PackSubtypeConstraint {
          sub_pack: sub_tp,
          super_pack: super_tp,
          returns: false,
        }));
      return UnifyResult::Ok;
    }

    // ... after doing all of our replacements, we may also need to check for
    // free types again.

    if !get_type_pack_id::<FreeTypePack>(sub_tp).is_none() {
      return self.emplace_free_type_pack(sub_tp, super_tp);
    }

    if !get_type_pack_id::<FreeTypePack>(super_tp).is_none() {
      return self.emplace_free_type_pack(super_tp, sub_tp);
    }

    UnifyResult::Ok
  }

  /// C++ `emplaceFreeTypePack` lambda from `unify_(TypePackId, TypePackId)`.
  fn emplace_free_type_pack(&mut self, target: TypePackId, bound_to: TypePackId) -> UnifyResult {
    LUAU_ASSERT!(!get_type_pack_id::<FreeTypePack>(target).is_none());

    let bound_to = self.instantiate_with_bound_types_pack(bound_to);

    let error_pack = unsafe { (*self.builtin_types.as_ptr()).error_type_pack };

    if FFlag::LuauOccursCheckForAllBindings.get() {
      if occurs_check_type_pack_id_type_pack_id(target, bound_to) == OccursCheckResult::Fail {
        unsafe {
          emplace_type_pack(
            as_mutable_type_pack_id(target),
            TypePackVariant::Bound(error_pack),
          )
        };
        return UnifyResult::OccursCheckFailed;
      }
    } else {
      let mut seen: DenseHashSet<TypePackId> = DenseHashSet::new(null());
      if OccursCheckResult::Fail == self.occurs_check_deprecated(&mut seen, target, bound_to) {
        unsafe {
          emplace_type_pack(
            as_mutable_type_pack_id(target),
            TypePackVariant::Bound(error_pack),
          )
        };
        return UnifyResult::OccursCheckFailed;
      }
    }

    unsafe {
      emplace_type_pack(
        as_mutable_type_pack_id(target),
        TypePackVariant::Bound(bound_to),
      )
    };
    UnifyResult::Ok
  }

  /// C++ `maybeReplaceTail` lambda from `unify_(TypePackId, TypePackId)`.
  fn maybe_replace_tail(&self, maybe_tp: Option<TypePackId>) -> TypePackId {
    let maybe_tp = match maybe_tp {
      None => return unsafe { (*self.builtin_types.as_ptr()).empty_type_pack },
      Some(tp) => tp,
    };

    let tp = unsafe { follow_type_pack_id(maybe_tp) };
    if let Some(replacement) = self.generic_pack_substitutions.find(&tp) {
      return unsafe { follow_type_pack_id(*replacement) };
    }
    tp
  }

  /// `instantiateWithBoundTypes` for type packs (`Unifier2.cpp:307-314`, TypePackId
  /// instantiation of the `template<typename TID>` overload).
  fn instantiate_with_bound_types_pack(&mut self, tp: TypePackId) -> TypePackId {
    let mut r = Replacer::new(
      self.arena.as_ptr(),
      NonNull::from(&mut self.generic_substitutions).as_ptr(),
      NonNull::from(&mut self.generic_pack_substitutions).as_ptr(),
    );
    if let Some(new_tp) = r.substitute_type_pack_id(tp) {
      return new_tp;
    }
    tp
  }
}
