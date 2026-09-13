use core::{cmp::min, mem::zeroed};

use ulua_common::DFInt;

use crate::{
  enums::{
    early_exit::EarlyExit, pack_field::PackField,
    subtyping_suppression_policy::SubtypingSuppressionPolicy, variant::Variant,
  },
  functions::{
    assert_reasoning_valid_subtyping::assert_reasoning_valid,
    flatten_type_pack::flatten_type_pack_id, follow_type_pack::follow_type_pack_id,
    get_type_pack::get_type_pack_id,
  },
  methods::{
    subtyping_is_covariant_with_super_tail::SuperTailCovariantArgs,
    subtyping_is_sub_tail_covariant_with::SubTailCovariantArgs,
  },
  records::{
    error_type_pack::ErrorTypePack, free_type_pack::FreeTypePack,
    generic_type_pack::GenericTypePack, index::Index,
    non_exceptional_recursion_limiter::NonExceptionalRecursionLimiter, nothing::Nothing,
    pack_subtype_constraint::PackSubtypeConstraint, scope::Scope, subtyping::Subtyping,
    subtyping_environment::SubtypingEnvironment, subtyping_result::SubtypingResult,
    type_error::TypeError, unexpected_type_pack_in_subtyping::UnexpectedTypePackInSubtyping,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{component::Component, constraint_v::ConstraintV, type_pack_id::TypePackId},
};
impl Subtyping {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn is_covariant_with_subtyping_environment_type_pack_id_type_pack_id_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    mut sub_tp: TypePackId,
    mut super_tp: TypePackId,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let recursion_count_ptr =
      unsafe { &mut (*(*self.normalizer).shared_state).counters.recursion_count as *mut i32 };
    let mut nerl = NonExceptionalRecursionLimiter {
      base: unsafe { zeroed() },
      native_stack_guard: unsafe { zeroed() },
    };
    nerl.non_exceptional_recursion_limiter_non_exceptional_recursion_limiter(recursion_count_ptr);
    if !nerl.is_ok(DFInt::LuauSubtypingRecursionLimit.get()) {
      return SubtypingResult {
        is_subtype: false,
        normalization_too_complex: true,
        ..Default::default()
      };
    }
    let _nerl = nerl;

    sub_tp = unsafe { follow_type_pack_id(sub_tp) };
    super_tp = unsafe { follow_type_pack_id(super_tp) };

    let type_pair = (sub_tp, super_tp);
    let fresh = {
      let entry = self.seen_packs.get_or_insert(type_pair);
      let fresh = !*entry;
      if fresh {
        *entry = true;
      }
      fresh
    };
    if !fresh {
      return SubtypingResult {
        is_subtype: true,
        is_cacheable: false,
        ..Default::default()
      };
    }

    let (sub_head, sub_tail) = flatten_type_pack_id(sub_tp);
    let (super_head, super_tail) = flatten_type_pack_id(super_tp);
    let head_size = min(sub_head.len(), super_head.len());

    let mut result = SubtypingResult {
      is_subtype: true,
      ..Default::default()
    };

    if sub_tp == super_tp {
      self.seen_pack_set_erase(type_pair);
      return result;
    }

    for i in 0..head_size {
      let mut part = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        sub_head[i],
        super_head[i],
        scope,
      );
      part.with_both_component(Component::Index(Index {
        index: i,
        variant: Variant::Pack,
      }));
      result.and_also(part, SubtypingSuppressionPolicy::Any);
    }

    if sub_head.len() < super_head.len() {
      if let Some(sub_tail) = sub_tail {
        let early_exit = unsafe {
          self.is_sub_tail_covariant_with(SubTailCovariantArgs {
            env,
            output_result: &mut result,
            sub_tp,
            sub_tail,
            super_tp,
            super_head_start_index: head_size,
            super_head: &super_head,
            super_tail,
            scope,
          })
        };
        if early_exit == EarlyExit::Yes {
          self.seen_pack_set_erase(type_pair);
          return result;
        }
      } else {
        result.and_also(
          SubtypingResult {
            is_subtype: false,
            ..Default::default()
          },
          SubtypingSuppressionPolicy::Any,
        );
        self.seen_pack_set_erase(type_pair);
        return result;
      }
    } else if sub_head.len() > super_head.len() {
      if let Some(super_tail) = super_tail {
        let early_exit = unsafe {
          self.is_covariant_with_super_tail(SuperTailCovariantArgs {
            env,
            output_result: &mut result,
            sub_tp,
            sub_head_start_index: head_size,
            sub_head: &sub_head,
            sub_tail,
            super_tp,
            super_tail,
            scope,
          })
        };
        if early_exit == EarlyExit::Yes {
          self.seen_pack_set_erase(type_pair);
          return result;
        }
      } else {
        self.seen_pack_set_erase(type_pair);
        return SubtypingResult {
          is_subtype: false,
          ..Default::default()
        };
      }
    }

    match (sub_tail, super_tail) {
      (Some(sub_tail), Some(super_tail)) => {
        if let (Some(sub), Some(super_)) = (
          get_type_pack_id::<VariadicTypePack>(sub_tail),
          get_type_pack_id::<VariadicTypePack>(super_tail),
        ) {
          let part = self
                        .is_tail_covariant_with_tail_subtyping_environment_not_null_scope_type_pack_id_variadic_type_pack_type_pack_id_variadic_type_pack(
                        env, scope, sub_tail, sub, super_tail, super_,
                    );
          result.and_also(part, SubtypingSuppressionPolicy::Any);
        } else if let (Some(sub), Some(super_)) = (
          get_type_pack_id::<GenericTypePack>(sub_tail),
          get_type_pack_id::<GenericTypePack>(super_tail),
        ) {
          let part = self
                        .is_tail_covariant_with_tail_subtyping_environment_not_null_scope_type_pack_id_generic_type_pack_type_pack_id_generic_type_pack(
                        env, scope, sub_tail, sub, super_tail, super_,
                    );
          result.and_also(part, SubtypingSuppressionPolicy::Any);
        } else if let (Some(sub), Some(super_)) = (
          get_type_pack_id::<VariadicTypePack>(sub_tail),
          get_type_pack_id::<GenericTypePack>(super_tail),
        ) {
          let part = self
                        .is_tail_covariant_with_tail_subtyping_environment_not_null_scope_type_pack_id_variadic_type_pack_type_pack_id_generic_type_pack(
                        env, scope, sub_tail, sub, super_tail, super_,
                    );
          result.and_also(part, SubtypingSuppressionPolicy::Any);
        } else if let (Some(sub), Some(super_)) = (
          get_type_pack_id::<GenericTypePack>(sub_tail),
          get_type_pack_id::<VariadicTypePack>(super_tail),
        ) {
          let part = self
                        .is_tail_covariant_with_tail_subtyping_environment_not_null_scope_type_pack_id_generic_type_pack_type_pack_id_variadic_type_pack(
                        env, scope, sub_tail, sub, super_tail, super_,
                    );
          result.and_also(part, SubtypingSuppressionPolicy::Any);
        } else if get_type_pack_id::<FreeTypePack>(sub_tail).is_some()
          || get_type_pack_id::<FreeTypePack>(super_tail).is_some()
        {
          let mut part = SubtypingResult {
            is_subtype: true,
            ..Default::default()
          };
          part.with_both_component(Component::PackField(PackField::Tail));
          part.with_assumed_constraint(ConstraintV::PackSubtype(PackSubtypeConstraint {
            sub_pack: sub_tail,
            super_pack: super_tail,
            returns: false,
          }));
          result.and_also(part, SubtypingSuppressionPolicy::Any);
        } else if get_type_pack_id::<ErrorTypePack>(sub_tail).is_some()
          || get_type_pack_id::<ErrorTypePack>(super_tail).is_some()
        {
          let mut part = SubtypingResult {
            is_subtype: true,
            ..Default::default()
          };
          part.with_both_component(Component::PackField(PackField::Tail));
          result.and_also(part, SubtypingSuppressionPolicy::Any);
        } else {
          let mut r = SubtypingResult {
            is_subtype: false,
            ..Default::default()
          };
          r.with_both_component(Component::PackField(PackField::Tail));
          r.with_error(TypeError::type_error_location_type_error_data(
            unsafe { (*scope).location },
            UnexpectedTypePackInSubtyping { tp: sub_tail }.into(),
          ));
          r.with_error(TypeError::type_error_location_type_error_data(
            unsafe { (*scope).location },
            UnexpectedTypePackInSubtyping { tp: super_tail }.into(),
          ));
          self.seen_pack_set_erase(type_pair);
          return r;
        }
      }
      (Some(sub_tail), None) => {
        if get_type_pack_id::<VariadicTypePack>(sub_tail).is_some() {
          let mut r = SubtypingResult {
            is_subtype: false,
            ..Default::default()
          };
          r.with_sub_component(Component::PackField(PackField::Tail));
          self.seen_pack_set_erase(type_pair);
          return r;
        } else if let Some(g) = get_type_pack_id::<GenericTypePack>(sub_tail) {
          let r = self
                        .is_tail_covariant_with_tail_subtyping_environment_not_null_scope_type_pack_id_generic_type_pack_nothing(
                        env,
                        scope,
                        sub_tail,
                        g,
                        Nothing::default(),
                    );
          self.seen_pack_set_erase(type_pair);
          return r;
        } else if get_type_pack_id::<FreeTypePack>(sub_tail).is_some() {
          let mut r = SubtypingResult {
            is_subtype: true,
            ..Default::default()
          };
          r.with_both_component(Component::PackField(PackField::Tail));
          r.with_assumed_constraint(ConstraintV::PackSubtype(PackSubtypeConstraint {
            sub_pack: sub_tail,
            super_pack: unsafe { (*self.builtin_types).empty_type_pack },
            returns: false,
          }));
          self.seen_pack_set_erase(type_pair);
          return r;
        } else {
          let mut r = SubtypingResult {
            is_subtype: false,
            ..Default::default()
          };
          r.with_sub_component(Component::PackField(PackField::Tail));
          r.with_error(TypeError::type_error_location_type_error_data(
            unsafe { (*scope).location },
            UnexpectedTypePackInSubtyping { tp: sub_tail }.into(),
          ));
          self.seen_pack_set_erase(type_pair);
          return r;
        }
      }
      (None, Some(super_tail)) => {
        if get_type_pack_id::<VariadicTypePack>(super_tail).is_some() {
          // A variadic super tail accepts the empty finite remainder.
        } else if let Some(g) = get_type_pack_id::<GenericTypePack>(super_tail) {
          let part = self
                        .is_tail_covariant_with_tail_subtyping_environment_not_null_scope_nothing_type_pack_id_generic_type_pack(
                        env,
                        scope,
                        Nothing::default(),
                        super_tail,
                        g,
                    );
          result.and_also(part, SubtypingSuppressionPolicy::Any);
        } else if get_type_pack_id::<FreeTypePack>(super_tail).is_some() {
          let mut part = SubtypingResult {
            is_subtype: true,
            ..Default::default()
          };
          part.with_both_component(Component::PackField(PackField::Tail));
          part.with_assumed_constraint(ConstraintV::PackSubtype(PackSubtypeConstraint {
            sub_pack: unsafe { (*self.builtin_types).empty_type_pack },
            super_pack: super_tail,
            returns: false,
          }));
          result.and_also(part, SubtypingSuppressionPolicy::Any);
        } else {
          let mut r = SubtypingResult {
            is_subtype: false,
            ..Default::default()
          };
          r.with_super_component(Component::PackField(PackField::Tail));
          r.with_error(TypeError::type_error_location_type_error_data(
            unsafe { (*scope).location },
            UnexpectedTypePackInSubtyping { tp: super_tail }.into(),
          ));
          self.seen_pack_set_erase(type_pair);
          return r;
        }
      }
      (None, None) => {}
    }

    assert_reasoning_valid(sub_tp, super_tp, &result, self.builtin_types, self.arena);

    self.seen_pack_set_erase(type_pair);
    result
  }

  #[inline]
  fn seen_pack_set_erase(&mut self, type_pair: (TypePackId, TypePackId)) {
    let entry = self.seen_packs.get_or_insert(type_pair);
    if *entry {
      *entry = false;
    }
  }
}
