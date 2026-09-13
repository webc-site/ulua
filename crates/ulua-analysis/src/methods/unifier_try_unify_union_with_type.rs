//! Source: `Analysis/src/Unifier.cpp` (Unifier::tryUnifyUnionWithType, L672-715)
use alloc::{string::String, sync::Arc, vec::Vec};

use crate::{
  functions::{has_unification_too_complex::has_unification_too_complex, is_nil::is_nil},
  records::{
    txn_log::TxnLog, type_error::TypeError, type_mismatch::TypeMismatch, unifier::Unifier,
    union_type::UnionType,
  },
  type_aliases::{type_error_data::TypeErrorData, type_id::TypeId},
};

impl Unifier {
  /// `void Unifier::tryUnifyUnionWithType(TypeId sub_ty, const UnionType* subUnion, TypeId super_ty)`
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn unifier_try_unify_union_with_type(
    &mut self,
    sub_ty: TypeId,
    sub_union: *const UnionType,
    super_ty: TypeId,
  ) {
    let sub_union = unsafe { &*sub_union };
    // A | B <: T if and only if A <: T and B <: T
    let mut failed = false;
    let mut errors_suppressed = true;
    let mut unification_too_complex: Option<TypeError> = None;
    let mut first_failed_option: Option<TypeError> = None;

    let mut logs: Vec<TxnLog> = Vec::new();

    let options = sub_union.options.clone();
    for ty in options {
      let mut inner_state = self.unifier_make_child_unifier();
      inner_state
        .try_unify_type_id_type_id_bool_bool_literal_properties(ty, super_ty, false, false, None);

      if let Some(e) = has_unification_too_complex(&inner_state.errors) {
        unification_too_complex = Some(e);
      } else if inner_state.failure {
        // If errors were suppressed, we store the log up, so we can commit it if no other option succeeds.
        if inner_state.errors.is_empty() {
          logs.push(inner_state.log);
        }
        // 'nil' option is skipped from extended report because we present the type in a special way - 'T?'
        else if first_failed_option.is_none() && !is_nil(ty) {
          first_failed_option = Some(inner_state.errors[0].clone());
        }

        failed = true;
        errors_suppressed &= inner_state.errors.is_empty();
      }
    }

    let combined = self.unifier_combine_logs_into_union(logs, self.types);
    unsafe { self.log.concat_as_union(combined, self.types) };

    if let Some(e) = unification_too_complex {
      self.report_error_type_error(e);
    } else if failed {
      if let Some(ffo) = first_failed_option {
        let context = self.unifier_mismatch_context();
        self.report_error_location_type_error_data(
          self.location,
          TypeErrorData::TypeMismatch(TypeMismatch {
            wanted_type: super_ty,
            given_type: sub_ty,
            reason: String::from("Not all union options are compatible."),
            error: Some(Arc::new(ffo)),
            context,
          }),
        );
      } else if !errors_suppressed {
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
      self.failure = true;
    }
  }
}
