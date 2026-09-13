//! Source: `Analysis/src/Unifier.cpp` (Unifier::tryUnify_(TypeId,...), L404-670)
use alloc::string::String;
use core::ptr::null;

use crate::{
  enums::{normalization_result::NormalizationResult, variance::Variance},
  functions::{
    get_type_alt_j::get_type_id, is_blocked_unifier::is_blocked_txn_log_type_id, is_prim::is_prim,
    promote_type_levels_unifier::promote_type_levels_txn_log_type_arena_type_level_type_id,
  },
  records::{
    any_type::AnyType,
    extern_type::ExternType,
    free_type::FreeType,
    function_type::FunctionType,
    generic_error::GenericError,
    generic_type::GenericType,
    intersection_type::IntersectionType,
    metatable_type::MetatableType,
    negation_type::NegationType,
    never_type::NeverType,
    primitive_type::{PrimitiveType, Type as PrimType},
    singleton_type::SingletonType,
    table_type::TableType,
    r#type::Type,
    type_function_instance_type::TypeFunctionInstanceType,
    type_mismatch::TypeMismatch,
    unification_too_complex::UnificationTooComplex,
    unifier::Unifier,
    union_type::UnionType,
    unknown_type::UnknownType,
    widen::Widen,
  },
  type_aliases::{
    error_type::ErrorType, literal_properties::LiteralProperties, type_error_data::TypeErrorData,
    type_id::TypeId, type_variant::TypeVariant,
  },
};
impl Unifier {
  /// `void Unifier::tryUnify_(TypeId sub_ty, TypeId super_ty, bool isFunctionCall, bool isIntersection, const LiteralProperties* literalProperties)`
  pub fn try_unify_type_id_type_id_bool_bool_literal_properties(
    &mut self,
    sub_ty: TypeId,
    super_ty: TypeId,
    is_function_call: bool,
    is_intersection: bool,
    literal_properties: Option<&LiteralProperties>,
  ) {
    self.try_unify_impl(
      sub_ty,
      super_ty,
      is_function_call,
      is_intersection,
      literal_properties,
    )
  }

  // 内部实现：裸指针解引用由 unsafe 块承担（私有可见性，不触发签名契约告警）。
  fn try_unify_impl(
    &mut self,
    mut sub_ty: TypeId,
    mut super_ty: TypeId,
    is_function_call: bool,
    is_intersection: bool,
    literal_properties: Option<&LiteralProperties>,
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

    super_ty = unsafe { self.log.follow_type_id(super_ty) };
    sub_ty = unsafe { self.log.follow_type_id(sub_ty) };

    if super_ty == sub_ty {
      return;
    }

    // Reflexive structural-equality fast-path. C++ relies on alias-shared
    // TypeIds making the `super_ty == sub_ty` pointer check above fire
    // pervasively; we don't pointer-share alias-derived composites, so
    // structurally-identical unions/functions/packs (e.g. `Color <: Color`,
    // Color = "red" | "blue") otherwise re-run the full walk on every
    // curried use and blow the iteration limit (np_hard). Reflexive — sound.
    if self.reflexive_equal_type_id(super_ty, sub_ty, 32) {
      return;
    }

    let sub_blocked = is_blocked_txn_log_type_id(&self.log, sub_ty);
    let super_blocked = is_blocked_txn_log_type_id(&self.log, super_ty);
    if sub_blocked && super_blocked {
      self.blocked_types.push(sub_ty);
      self.blocked_types.push(super_ty);
    } else if sub_blocked {
      self.blocked_types.push(sub_ty);
    } else if super_blocked {
      self.blocked_types.push(super_ty);
    }

    if !self
      .log
      .txn_log_get::<TypeFunctionInstanceType, TypeId>(super_ty)
      .is_null()
    {
      self.ice_string("Unexpected TypeFunctionInstanceType super_ty");
    }

    if !self
      .log
      .txn_log_get::<TypeFunctionInstanceType, TypeId>(sub_ty)
      .is_null()
    {
      self.ice_string("Unexpected TypeFunctionInstanceType sub_ty");
    }

    let super_free = self.log.txn_log_get_mutable::<FreeType, TypeId>(super_ty);
    let sub_free = self.log.txn_log_get_mutable::<FreeType, TypeId>(sub_ty);

    // C++ `subsumes(a, b)` for free/generic vars: `a->level.subsumes(b->level)`.
    // (Caller guarantees non-null pointers at each use site.)
    if !super_free.is_null()
      && !sub_free.is_null()
      && unsafe { (*super_free).level.subsumes(&(*sub_free).level) }
    {
      if !self.occurs_check_type_id_type_id_bool(sub_ty, super_ty, false) {
        self
          .log
          .replace_type_id_t(sub_ty, Type::new(TypeVariant::Bound(super_ty)));
      }
      return;
    } else if !super_free.is_null() && !sub_free.is_null() {
      if !self.occurs_check_type_id_type_id_bool(super_ty, sub_ty, true) {
        if unsafe { (*super_free).level.subsumes(&(*sub_free).level) } {
          self
            .log
            .change_level_type_id_type_level(sub_ty, unsafe { (*super_free).level });
        }
        self
          .log
          .replace_type_id_t(super_ty, Type::new(TypeVariant::Bound(sub_ty)));
      }
      return;
    } else if !super_free.is_null() {
      // Unification can't change the level of a generic.
      let sub_generic = self.log.txn_log_get_mutable::<GenericType, TypeId>(sub_ty);
      if !sub_generic.is_null() && !unsafe { (*sub_generic).level.subsumes(&(*super_free).level) } {
        self.report_error_location_type_error_data(
          self.location,
          TypeErrorData::GenericError(GenericError::new(String::from(
            "Generic subtype escaping scope",
          ))),
        );
        return;
      }

      if !self.occurs_check_type_id_type_id_bool(super_ty, sub_ty, true) {
        let super_level = unsafe { (*super_free).level };
        unsafe {
          promote_type_levels_txn_log_type_arena_type_level_type_id(
            &mut self.log,
            &*self.types,
            super_level,
            sub_ty,
          )
        };

        let mut widen = Widen::widen_widen(self.types, self.builtin_types);
        let widened = widen.operator_call_mut(sub_ty);
        self
          .log
          .replace_type_id_t(super_ty, Type::new(TypeVariant::Bound(widened)));
      }
      return;
    } else if !sub_free.is_null() {
      // Normally, if the subtype is free, it should not be bound to any, unknown, or error types.
      // But for bug compatibility, we'll only apply this rule to unknown.
      if !self
        .log
        .txn_log_get::<UnknownType, TypeId>(super_ty)
        .is_null()
      {
        return;
      }

      let super_generic = self
        .log
        .txn_log_get_mutable::<GenericType, TypeId>(super_ty);
      if !super_generic.is_null() && !unsafe { (*super_generic).level.subsumes(&(*sub_free).level) }
      {
        self.report_error_location_type_error_data(
          self.location,
          TypeErrorData::GenericError(GenericError::new(String::from(
            "Generic supertype escaping scope",
          ))),
        );
        return;
      }

      if !self.occurs_check_type_id_type_id_bool(sub_ty, super_ty, false) {
        let sub_level = unsafe { (*sub_free).level };
        unsafe {
          promote_type_levels_txn_log_type_arena_type_level_type_id(
            &mut self.log,
            &*self.types,
            sub_level,
            super_ty,
          )
        };
        self
          .log
          .replace_type_id_t(sub_ty, Type::new(TypeVariant::Bound(super_ty)));
      }
      return;
    }

    if !self.log.txn_log_get::<AnyType, TypeId>(super_ty).is_null() {
      return self
        .try_unify_with_any_type_id_type_id(sub_ty, unsafe { (*self.builtin_types).any_type });
    }

    if !self.log.txn_log_get::<AnyType, TypeId>(sub_ty).is_null() {
      if self.normalize {
        // TODO: there are probably cheaper ways to check if any <: T.
        let super_norm = unsafe { (*self.normalizer).normalize(super_ty) };
        if !get_type_id::<AnyType>(super_norm.tops).is_none() {
          // handled below
        } else {
          self.failure = true;
        }
      } else {
        self.failure = true;
      }
      return self
        .try_unify_with_any_type_id_type_id(super_ty, unsafe { (*self.builtin_types).any_type });
    }

    if !self.log.txn_log_get::<NeverType, TypeId>(sub_ty).is_null() {
      return self
        .try_unify_with_any_type_id_type_id(super_ty, unsafe { (*self.builtin_types).never_type });
    }

    // What if the types are immutable and we proved their relation before
    let cache_enabled =
      !is_function_call && !is_intersection && self.variance == Variance::Invariant;

    if cache_enabled {
      if unsafe {
        (*self.shared_state)
          .cached_unify
          .find(&(sub_ty, super_ty))
          .is_some()
      } {
        return;
      }

      // C++: `if (auto error = sharedState.cachedUnifyError.find({sub_ty, super_ty})) { reportError(*error); return; }`
      // Serve a previously-cached unification error rather than re-exploring the
      // (potentially exponential) failing subtree. This negative-cache fast-path is
      // what keeps pathological intersection subtyping (e.g. graph-coloring encodings)
      // under the iteration limit.
      if let Some(error) = unsafe {
        (*self.shared_state)
          .cached_unify_error
          .find(&(sub_ty, super_ty))
          .cloned()
      } {
        self.report_error_location_type_error_data(self.location, error);
        return;
      }
    }

    // If we have seen this pair before, we are recursing into cyclic types; assume they unify.
    if self.log.have_seen_type_id_type_id(super_ty, sub_ty) {
      return;
    }

    self.log.push_seen_type_id_type_id(super_ty, sub_ty);

    let error_count = self.errors.len();

    let sub_union = self.log.txn_log_get_mutable::<UnionType, TypeId>(sub_ty);
    let super_intersection = self
      .log
      .txn_log_get_mutable::<IntersectionType, TypeId>(super_ty);
    let super_union = self.log.txn_log_get_mutable::<UnionType, TypeId>(super_ty);
    let sub_intersection = self
      .log
      .txn_log_get_mutable::<IntersectionType, TypeId>(sub_ty);

    if !sub_union.is_null() {
      unsafe { self.unifier_try_unify_union_with_type(sub_ty, sub_union, super_ty) };
    } else if !super_intersection.is_null() {
      unsafe {
        self.unifier_try_unify_type_with_intersection(sub_ty, super_ty, super_intersection)
      };
    } else if !super_union.is_null() {
      unsafe {
        self.unifier_try_unify_type_with_union(
          sub_ty,
          super_ty,
          super_union,
          cache_enabled,
          is_function_call,
        )
      };
    } else if !sub_intersection.is_null() {
      unsafe {
        self.unifier_try_unify_intersection_with_type(
          sub_ty,
          sub_intersection,
          super_ty,
          cache_enabled,
          is_function_call,
        )
      };
    } else if !self.log.txn_log_get::<AnyType, TypeId>(sub_ty).is_null() {
      self.try_unify_with_any_type_id_type_id(super_ty, unsafe {
        (*self.builtin_types).unknown_type
      });
      self.failure = true;
    } else if !self.log.txn_log_get::<ErrorType, TypeId>(sub_ty).is_null()
      && !self
        .log
        .txn_log_get::<ErrorType, TypeId>(super_ty)
        .is_null()
    {
      // error <: error
    } else if !self
      .log
      .txn_log_get::<ErrorType, TypeId>(super_ty)
      .is_null()
    {
      self.try_unify_with_any_type_id_type_id(sub_ty, unsafe { (*self.builtin_types).error_type });
      self.failure = true;
    } else if !self.log.txn_log_get::<ErrorType, TypeId>(sub_ty).is_null() {
      self
        .try_unify_with_any_type_id_type_id(super_ty, unsafe { (*self.builtin_types).error_type });
      self.failure = true;
    } else if !self
      .log
      .txn_log_get::<UnknownType, TypeId>(super_ty)
      .is_null()
    {
      // At this point, all the supertypes of `error` have been handled.
      self
        .try_unify_with_any_type_id_type_id(sub_ty, unsafe { (*self.builtin_types).unknown_type });
    } else if !self
      .log
      .txn_log_get_mutable::<PrimitiveType, TypeId>(super_ty)
      .is_null()
      && !self
        .log
        .txn_log_get_mutable::<PrimitiveType, TypeId>(sub_ty)
        .is_null()
    {
      self.unifier_try_unify_primitives(sub_ty, super_ty);
    } else if (!self
      .log
      .txn_log_get_mutable::<PrimitiveType, TypeId>(super_ty)
      .is_null()
      || !self
        .log
        .txn_log_get_mutable::<SingletonType, TypeId>(super_ty)
        .is_null())
      && !self
        .log
        .txn_log_get_mutable::<SingletonType, TypeId>(sub_ty)
        .is_null()
    {
      self.unifier_try_unify_singletons(sub_ty, super_ty);
    } else if get_type_id::<PrimitiveType>(super_ty).is_some_and(|p| p.r#type == PrimType::Function)
      && !get_type_id::<FunctionType>(sub_ty).is_none()
    {
      // Ok. Do nothing. forall functions F, F <: function
    } else if is_prim(super_ty, PrimType::Table)
      && (!get_type_id::<TableType>(sub_ty).is_none()
        || !get_type_id::<MetatableType>(sub_ty).is_none())
    {
      // Ok, do nothing: forall tables T, T <: table
    } else if !self
      .log
      .txn_log_get_mutable::<FunctionType, TypeId>(super_ty)
      .is_null()
      && !self
        .log
        .txn_log_get_mutable::<FunctionType, TypeId>(sub_ty)
        .is_null()
    {
      self.unifier_try_unify_functions(sub_ty, super_ty, is_function_call);
    } else if let table = self.log.txn_log_get::<PrimitiveType, TypeId>(super_ty)
      && !table.is_null()
      && unsafe { (*table).r#type } == PrimType::Table
    {
      let empty_table = unsafe { (*self.builtin_types).empty_table_type };
      self.try_unify_type_id_type_id_bool_bool_literal_properties_entry(
        sub_ty,
        empty_table,
        is_function_call,
        is_intersection,
        None,
      );
    } else if let table = self.log.txn_log_get::<PrimitiveType, TypeId>(sub_ty)
      && !table.is_null()
      && unsafe { (*table).r#type } == PrimType::Table
    {
      let empty_table = unsafe { (*self.builtin_types).empty_table_type };
      self.try_unify_type_id_type_id_bool_bool_literal_properties_entry(
        empty_table,
        super_ty,
        is_function_call,
        is_intersection,
        None,
      );
    } else if !self
      .log
      .txn_log_get_mutable::<TableType, TypeId>(super_ty)
      .is_null()
      && !self
        .log
        .txn_log_get_mutable::<TableType, TypeId>(sub_ty)
        .is_null()
    {
      unsafe {
        self.unifier_try_unify_tables(
          sub_ty,
          super_ty,
          is_intersection,
          literal_properties.map_or(null(), |lp| lp as *const LiteralProperties),
        )
      };
    } else if !self
      .log
      .txn_log_get::<TableType, TypeId>(super_ty)
      .is_null()
      && (!self
        .log
        .txn_log_get::<PrimitiveType, TypeId>(sub_ty)
        .is_null()
        || !self
          .log
          .txn_log_get::<SingletonType, TypeId>(sub_ty)
          .is_null())
    {
      self.unifier_try_unify_scalar_shape(sub_ty, super_ty, false);
    } else if !self.log.txn_log_get::<TableType, TypeId>(sub_ty).is_null()
      && (!self
        .log
        .txn_log_get::<PrimitiveType, TypeId>(super_ty)
        .is_null()
        || !self
          .log
          .txn_log_get::<SingletonType, TypeId>(super_ty)
          .is_null())
    {
      self.unifier_try_unify_scalar_shape(sub_ty, super_ty, true);
    } else if !self
      .log
      .txn_log_get_mutable::<MetatableType, TypeId>(super_ty)
      .is_null()
    {
      self.unifier_try_unify_with_metatable(sub_ty, super_ty, false);
    } else if !self
      .log
      .txn_log_get_mutable::<MetatableType, TypeId>(sub_ty)
      .is_null()
    {
      self.unifier_try_unify_with_metatable(super_ty, sub_ty, true);
    } else if !self
      .log
      .txn_log_get_mutable::<ExternType, TypeId>(super_ty)
      .is_null()
    {
      self.unifier_try_unify_with_extern_type(sub_ty, super_ty, false);
    } else if !self
      .log
      .txn_log_get_mutable::<ExternType, TypeId>(sub_ty)
      .is_null()
    {
      self.unifier_try_unify_with_extern_type(sub_ty, super_ty, true);
    } else if !self
      .log
      .txn_log_get::<NegationType, TypeId>(super_ty)
      .is_null()
      || !self
        .log
        .txn_log_get::<NegationType, TypeId>(sub_ty)
        .is_null()
    {
      self.unifier_try_unify_negations(sub_ty, super_ty);
    } else if self.check_inhabited
      && unsafe { (*self.normalizer).is_inhabited_type_id(sub_ty) } == NormalizationResult::False
    {
      // uninhabited; nothing to do
    } else {
      let context = self.unifier_mismatch_context();
      self.report_error_location_type_error_data(
        self.location,
        TypeErrorData::TypeMismatch(TypeMismatch {
          wanted_type: super_ty,
          given_type: sub_ty,
          reason: String::new(),
          error: None,
          context,
        }),
      );
    }

    if cache_enabled {
      self.unifier_cache_result(sub_ty, super_ty, error_count);
    }

    self.log.pop_seen_type_id_type_id(super_ty, sub_ty);
  }
}
