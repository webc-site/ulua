//! Source: `Analysis/src/Unifier.cpp` (Unifier::tryUnifyUnionWithType, L672-715)
use alloc::{string::String, vec::Vec};

use crate::{
  functions::{has_unification_too_complex::has_unification_too_complex, is_prim::is_nil},
  records::{txn_log::TxnLog, type_error::TypeError, unifier::Unifier, union_type::UnionType},
  type_aliases::type_id::TypeId,
};

impl Unifier {
  /// `void Unifier::tryUnifyUnionWithType(TypeId sub_ty, const UnionType* subUnion, TypeId super_ty)`
  ///
  /// Rust 形态（§2）：C++ `const UnionType*` 入参 → `&'static UnionType`，
  /// 非空与存活由类型系统承载，本方法体内不再有裸指针解引用。调用方快照
  /// （`txn_log_get_mutable` 结果）经 `alias_opt`/`get_type_id` 折叠为引用后传入。
  pub fn unifier_try_unify_union_with_type(
    &mut self,
    sub_ty: TypeId,
    sub_union: &'static UnionType,
    super_ty: TypeId,
  ) {
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
    // Safety: self.types 是 Unifier 持有的 arena 句柄（目标存活且本方法内
    // 经 &mut self 独占驱动），满足 concat_as_union 的句柄别名契约。
    unsafe { self.log.concat_as_union(combined, self.types) };

    if let Some(e) = unification_too_complex {
      self.report_error_type_error(e);
    } else if failed {
      if let Some(ffo) = first_failed_option {
        self.unifier_report_type_mismatch_ext(
          super_ty,
          sub_ty,
          String::from("Not all union options are compatible."),
          Some(ffo),
        );
      } else if !errors_suppressed {
        self.unifier_report_type_mismatch(super_ty, sub_ty);
      }
      self.failure = true;
    }
  }
}
