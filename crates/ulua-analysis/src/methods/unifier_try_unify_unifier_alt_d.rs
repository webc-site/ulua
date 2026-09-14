//! Source: `Analysis/src/Unifier.cpp` (Unifier::tryUnify_(TypePackId,...), L1405-1634)
use alloc::{string::String, vec::Vec};
use core::{mem::swap, ptr::null_mut};

use ulua_common::FInt;

use crate::{
  enums::polarity::Polarity,
  functions::{
    flatten_type_pack_alt_b::flatten, fresh_type::fresh_type,
    is_blocked_unifier_alt_c::is_blocked_txn_log_type_pack_id, is_optional::is_optional,
    size_type_pack::size,
  },
  records::{
    count_mismatch::{CountMismatch, CountMismatchContext},
    free_type_pack::FreeTypePack,
    scope::Scope,
    txn_log::TxnLog,
    type_level::TypeLevel,
    type_pack::TypePack,
    type_pack_mismatch::TypePackMismatch,
    type_pack_var::TypePackVar,
    unification_too_complex::UnificationTooComplex,
    unifier::Unifier,
    variadic_type_pack::VariadicTypePack,
    weird_iter::WeirdIter,
    widen::Widen,
  },
  type_aliases::{
    error_type_pack::ErrorTypePack, type_error_data::TypeErrorData, type_id::TypeId,
    type_pack_id::TypePackId, type_pack_variant::TypePackVariant,
  },
};
impl Unifier {
  /// `void Unifier::tryUnify_(TypePackId subTp, TypePackId superTp, bool isFunctionCall)`
  pub fn try_unify_type_pack_id_type_pack_id_bool(
    &mut self,
    sub_tp: TypePackId,
    super_tp: TypePackId,
    is_function_call: bool,
  ) {
    self.try_unify_pack_impl(sub_tp, super_tp, is_function_call)
  }

  // 内部实现：裸指针解引用由 unsafe 块承担（私有可见性，不触发签名契约告警）。
  fn try_unify_pack_impl(
    &mut self,
    mut sub_tp: TypePackId,
    mut super_tp: TypePackId,
    is_function_call: bool,
  ) {
    unsafe {
      (*self.shared_state).counters.iteration_count += 1;
      if (*self.shared_state).counters.iteration_limit > 0
        && (*self.shared_state).counters.iteration_limit
          < (*self.shared_state).counters.iteration_count
      {
        self.report_error_location_type_error_data(
          self.location,
          TypeErrorData::UnificationTooComplex(UnificationTooComplex::default()),
        );
        return;
      }
    }

    super_tp = unsafe { self.log.follow_type_pack_id(super_tp) };
    sub_tp = unsafe { self.log.follow_type_pack_id(sub_tp) };

    // Reflexive structural-equality fast-path (see unifier_reflexive_equal):
    // identical curried-function arg/return packs (e.g. `(Color)` vs
    // `(Color)`) recur element-by-element on every use without this, which
    // is what tips np_hard over the iteration limit.
    if self.reflexive_equal_type_pack_id(super_tp, sub_tp, 32) {
      return;
    }

    loop {
      let tp = self.log.txn_log_get_mutable::<TypePack, TypePackId>(sub_tp);
      if tp.is_null() {
        break;
      }
      if unsafe { (*tp).head.is_empty() && (*tp).tail.is_some() } {
        sub_tp = unsafe { self.log.follow_type_pack_id((*tp).tail.unwrap()) };
      } else {
        break;
      }
    }

    loop {
      let tp = self
        .log
        .txn_log_get_mutable::<TypePack, TypePackId>(super_tp);
      if tp.is_null() {
        break;
      }
      if unsafe { (*tp).head.is_empty() && (*tp).tail.is_some() } {
        super_tp = unsafe { self.log.follow_type_pack_id((*tp).tail.unwrap()) };
      } else {
        break;
      }
    }

    if super_tp == sub_tp {
      return;
    }

    if self
      .log
      .have_seen_type_pack_id_type_pack_id(super_tp, sub_tp)
    {
      return;
    }

    let sub_blocked = is_blocked_txn_log_type_pack_id(&self.log, sub_tp);
    let super_blocked = is_blocked_txn_log_type_pack_id(&self.log, super_tp);
    if sub_blocked && super_blocked {
      self.blocked_type_packs.push(sub_tp);
      self.blocked_type_packs.push(super_tp);
    } else if sub_blocked {
      self.blocked_type_packs.push(sub_tp);
    } else if super_blocked {
      self.blocked_type_packs.push(super_tp);
    }

    if !self
      .log
      .txn_log_get_mutable::<FreeTypePack, TypePackId>(super_tp)
      .is_null()
    {
      if !self.occurs_check_type_pack_id_type_pack_id_bool(super_tp, sub_tp, true) {
        let mut widen = Widen::widen_widen(self.types, self.builtin_types);
        let widened = widen.operator_call(sub_tp);
        let bound = TypePackVar {
          ty: TypePackVariant::Bound(widened),
          persistent: false,
          owning_arena: null_mut(),
        };
        self.log.replace_type_pack_id_type_pack_var(super_tp, bound);
      }
    } else if !self
      .log
      .txn_log_get_mutable::<FreeTypePack, TypePackId>(sub_tp)
      .is_null()
    {
      if !self.occurs_check_type_pack_id_type_pack_id_bool(sub_tp, super_tp, false) {
        let bound = TypePackVar {
          ty: TypePackVariant::Bound(super_tp),
          persistent: false,
          owning_arena: null_mut(),
        };
        self.log.replace_type_pack_id_type_pack_var(sub_tp, bound);
      }
    } else if !self
      .log
      .txn_log_get_mutable::<ErrorTypePack, TypePackId>(super_tp)
      .is_null()
    {
      self.try_unify_with_any_type_pack_id_type_pack_id(sub_tp, super_tp);
    } else if !self
      .log
      .txn_log_get_mutable::<ErrorTypePack, TypePackId>(sub_tp)
      .is_null()
    {
      self.try_unify_with_any_type_pack_id_type_pack_id(super_tp, sub_tp);
    } else if !self
      .log
      .txn_log_get_mutable::<VariadicTypePack, TypePackId>(super_tp)
      .is_null()
    {
      self.unifier_try_unify_variadics(sub_tp, super_tp, false, 0);
    } else if !self
      .log
      .txn_log_get_mutable::<VariadicTypePack, TypePackId>(sub_tp)
      .is_null()
    {
      self.unifier_try_unify_variadics(super_tp, sub_tp, true, 0);
    } else if !self
      .log
      .txn_log_get_mutable::<TypePack, TypePackId>(super_tp)
      .is_null()
      && !self
        .log
        .txn_log_get_mutable::<TypePack, TypePackId>(sub_tp)
        .is_null()
    {
      let super_tpv = self
        .log
        .txn_log_get_mutable::<TypePack, TypePackId>(super_tp);
      let sub_tpv = self.log.txn_log_get_mutable::<TypePack, TypePackId>(sub_tp);

      // If the size of two heads does not match, but both packs have free tail
      // we set the sentinel to avoid growing forever.
      let (super_types, super_tail) = flatten(super_tp, &self.log);
      let (sub_types, sub_tail) = flatten(sub_tp, &self.log);

      let no_infinite_growth = (super_types.len() != sub_types.len())
        && super_tail.is_some_and(|t| {
          !self
            .log
            .txn_log_get_mutable::<FreeTypePack, TypePackId>(t)
            .is_null()
        })
        && sub_tail.is_some_and(|t| {
          !self
            .log
            .txn_log_get_mutable::<FreeTypePack, TypePackId>(t)
            .is_null()
        });

      let mut super_iter = WeirdIter {
        pack_id: super_tp,
        log: &mut self.log as *mut _,
        pack: null_mut(),
        index: 0,
        growing: false,
        level: TypeLevel::default(),
        scope: null_mut(),
      };
      super_iter.weird_iter_type_pack_id_txn_log(super_tp, unsafe { &mut *(self.log_ptr()) });

      let mut sub_iter = WeirdIter {
        pack_id: sub_tp,
        log: &mut self.log as *mut _,
        pack: null_mut(),
        index: 0,
        growing: false,
        level: TypeLevel::default(),
        scope: null_mut(),
      };
      sub_iter.weird_iter_type_pack_id_txn_log(sub_tp, unsafe { &mut *(self.log_ptr()) });

      super_iter.scope = self.scope;
      sub_iter.scope = self.scope;

      let empty_tp = unsafe {
        (*self.types).add_type_pack_t(TypePack {
          head: Vec::new(),
          tail: None,
        })
      };

      let mut loop_count = 0;

      loop {
        if FInt::LuauTypeInferTypePackLoopLimit.get() > 0
          && loop_count >= FInt::LuauTypeInferTypePackLoopLimit.get()
        {
          self.ice_string("Detected possibly infinite TypePack growth");
        }

        loop_count += 1;

        if super_iter.weird_iter_good() && sub_iter.growing {
          let ft = self.mk_fresh_for_iter(sub_iter.scope);
          sub_iter.weird_iter_push_type(ft);
        }

        if sub_iter.weird_iter_good() && super_iter.growing {
          let ft = self.mk_fresh_for_iter(super_iter.scope);
          super_iter.weird_iter_push_type(ft);
        }

        if super_iter.weird_iter_good() && sub_iter.weird_iter_good() {
          let s = *sub_iter.weird_iter_operator_deref();
          let sup = *super_iter.weird_iter_operator_deref();
          self.try_unify_type_id_type_id_bool_bool_literal_properties(s, sup, false, false, None);

          if !self.errors.is_empty() && self.first_pack_error_pos.is_none() {
            self.first_pack_error_pos = Some(loop_count);
          }

          super_iter.weird_iter_advance();
          sub_iter.weird_iter_advance();
          continue;
        }

        // If both are at the end, we're done
        if !super_iter.weird_iter_good() && !sub_iter.weird_iter_good() {
          let l_free_tail = unsafe {
            (*super_tpv).tail.is_some_and(|t| {
              !self
                .log
                .txn_log_get_mutable::<FreeTypePack, TypePackId>(self.log.follow_type_pack_id(t))
                .is_null()
            })
          };
          let r_free_tail = unsafe {
            (*sub_tpv).tail.is_some_and(|t| {
              !self
                .log
                .txn_log_get_mutable::<FreeTypePack, TypePackId>(self.log.follow_type_pack_id(t))
                .is_null()
            })
          };
          if l_free_tail && r_free_tail {
            self.try_unify_type_pack_id_type_pack_id_bool(
              unsafe { (*sub_tpv).tail.unwrap() },
              unsafe { (*super_tpv).tail.unwrap() },
              false,
            );
          } else if l_free_tail {
            self.try_unify_type_pack_id_type_pack_id_bool(
              empty_tp,
              unsafe { (*super_tpv).tail.unwrap() },
              false,
            );
          } else if r_free_tail {
            self.try_unify_type_pack_id_type_pack_id_bool(
              empty_tp,
              unsafe { (*sub_tpv).tail.unwrap() },
              false,
            );
          } else if unsafe { (*sub_tpv).tail.is_some() && (*super_tpv).tail.is_some() } {
            if !self
              .log
              .txn_log_get_mutable::<VariadicTypePack, TypePackId>(super_iter.pack_id)
              .is_null()
            {
              self.unifier_try_unify_variadics(
                sub_iter.pack_id,
                super_iter.pack_id,
                false,
                sub_iter.index as i32,
              );
            } else if !self
              .log
              .txn_log_get_mutable::<VariadicTypePack, TypePackId>(sub_iter.pack_id)
              .is_null()
            {
              self.unifier_try_unify_variadics(
                super_iter.pack_id,
                sub_iter.pack_id,
                true,
                super_iter.index as i32,
              );
            } else {
              self.try_unify_type_pack_id_type_pack_id_bool(
                unsafe { (*sub_tpv).tail.unwrap() },
                unsafe { (*super_tpv).tail.unwrap() },
                false,
              );
            }
          }

          break;
        }

        // If both tails are free, bind one to the other and call it a day
        if super_iter.weird_iter_can_grow() && sub_iter.weird_iter_can_grow() {
          let s = unsafe { (*sub_iter.pack).tail.unwrap() };
          let sup = unsafe { (*super_iter.pack).tail.unwrap() };
          return self.try_unify_type_pack_id_type_pack_id_bool(s, sup, false);
        }

        // If just one side is free on its tail, grow it to fit the other side.
        if super_iter.weird_iter_can_grow() {
          let new_tail = unsafe {
            (*self.types).add_type_pack_type_pack_var(TypePackVar {
              ty: TypePackVariant::TypePack(TypePack {
                head: Vec::new(),
                tail: None,
              }),
              persistent: false,
              owning_arena: null_mut(),
            })
          };
          super_iter.weird_iter_grow(new_tail);
        } else if sub_iter.weird_iter_can_grow() {
          let new_tail = unsafe {
            (*self.types).add_type_pack_type_pack_var(TypePackVar {
              ty: TypePackVariant::TypePack(TypePack {
                head: Vec::new(),
                tail: None,
              }),
              persistent: false,
              owning_arena: null_mut(),
            })
          };
          sub_iter.weird_iter_grow(new_tail);
        } else {
          // A union type including nil marks an optional argument
          if super_iter.weird_iter_good() && is_optional(*super_iter.weird_iter_operator_deref()) {
            super_iter.weird_iter_advance();
            continue;
          } else if sub_iter.weird_iter_good() && is_optional(*sub_iter.weird_iter_operator_deref())
          {
            sub_iter.weird_iter_advance();
            continue;
          }

          if !self
            .log
            .txn_log_get_mutable::<VariadicTypePack, TypePackId>(super_iter.pack_id)
            .is_null()
          {
            self.unifier_try_unify_variadics(
              sub_iter.pack_id,
              super_iter.pack_id,
              false,
              sub_iter.index as i32,
            );
            return;
          }

          if !self
            .log
            .txn_log_get_mutable::<VariadicTypePack, TypePackId>(sub_iter.pack_id)
            .is_null()
          {
            self.unifier_try_unify_variadics(
              super_iter.pack_id,
              sub_iter.pack_id,
              true,
              super_iter.index as i32,
            );
            return;
          }

          if !is_function_call && sub_iter.weird_iter_good() {
            // Sometimes it is ok to pass too many arguments
            return;
          }

          // This is a bit weird because we don't actually know expected vs actual.
          let log_ptr = self.log_ptr();
          let mut expected_size = unsafe { size(super_tp, log_ptr) };
          let mut actual_size = unsafe { size(sub_tp, log_ptr) };
          if self.ctx == CountMismatchContext::FunctionResult
            || self.ctx == CountMismatchContext::ExprListResult
          {
            swap(&mut expected_size, &mut actual_size);
          }
          let ctx = self.ctx;
          self.report_error_location_type_error_data(
            self.location,
            TypeErrorData::CountMismatch(CountMismatch {
              expected: expected_size,
              maximum: None,
              actual: actual_size,
              context: ctx,
              is_variadic: false,
              function: String::new(),
            }),
          );

          let error_type = unsafe { (*self.builtin_types).error_type };
          while super_iter.weird_iter_good() {
            let cur = *super_iter.weird_iter_operator_deref();
            self.try_unify_type_id_type_id_bool_bool_literal_properties(
              cur, error_type, false, false, None,
            );
            super_iter.weird_iter_advance();
          }

          while sub_iter.weird_iter_good() {
            let cur = *sub_iter.weird_iter_operator_deref();
            self.try_unify_type_id_type_id_bool_bool_literal_properties(
              cur, error_type, false, false, None,
            );
            sub_iter.weird_iter_advance();
          }

          return;
        }

        if no_infinite_growth {
          break;
        }
      }
    } else {
      self.report_error_location_type_error_data(
        self.location,
        TypeErrorData::TypePackMismatch(TypePackMismatch {
          wanted_tp: super_tp,
          given_tp: sub_tp,
          reason: String::new(),
        }),
      );
    }
  }

  /// `mkFreshType` lambda in tryUnify_: `freshType(NotNull{types}, builtinTypes, scope)`.
  fn mk_fresh_for_iter(&mut self, scope: *mut Scope) -> TypeId {
    fresh_type(
      unsafe { &mut *self.types },
      unsafe { &*self.builtin_types },
      scope,
      Polarity::Positive,
    )
  }

  #[inline]
  fn log_ptr(&mut self) -> *mut TxnLog {
    &mut self.log as *mut _
  }
}
