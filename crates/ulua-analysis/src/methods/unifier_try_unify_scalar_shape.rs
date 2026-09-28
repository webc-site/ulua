//! Source: `Analysis/src/Unifier.cpp` (Unifier::tryUnifyScalarShape, L2151-2224)
use alloc::string::String;
use core::mem::swap;

use crate::{
  enums::{normalization_result::NormalizationResult, table_state::TableState},
  functions::{
    get_metatable_type::get_metatable_type_id_not_null_builtin_types,
    has_unification_too_complex::has_unification_too_complex,
  },
  records::{table_type::TableType, r#type::Type, type_error::TypeError, unifier::Unifier},
  type_aliases::{type_id::TypeId, type_variant::TypeVariant},
};
impl Unifier {
  /// `void Unifier::tryUnifyScalarShape(TypeId sub_ty, TypeId super_ty, bool reversed)`
  pub(crate) fn unifier_try_unify_scalar_shape(
    &mut self,
    mut sub_ty: TypeId,
    mut super_ty: TypeId,
    reversed: bool,
  ) {
    let osub_ty = sub_ty;
    let osuper_ty = super_ty;

    // If the normalizer hits resource limits, we can't show it's uninhabited, so, we should continue.
    if self.check_inhabited
      && self.normalizer_mut().is_inhabited_type_id(sub_ty) == NormalizationResult::False
    {
      return;
    }

    if reversed {
      swap(&mut sub_ty, &mut super_ty);
    }

    let super_table = self.log.txn_log_get_mutable::<TableType, TypeId>(super_ty);

    if super_table.is_null() || unsafe { (*super_table).state } != TableState::Free {
      self.unifier_report_type_mismatch(osuper_ty, osub_ty);
      return;
    }

    // Given t1 where t1 = { lower: (t1) -> (a, b...) }
    // It should be the case that `string <: t1` iff `(subtype's metatable).__index <: t1`
    if let Some(metatable) =
      get_metatable_type_id_not_null_builtin_types(sub_ty, self.builtin_types_ref())
    {
      // C++ `if (!mttv) fail(std::nullopt);`——原实现（含上游）缺失 return，
      // 这里以 Option 折叠判空与解引用：None 时仅报错，跳过 `__index` 查找，
      // 活跃路径行为与 C++ 一致，死路径由崩溃改为报错。
      // 先抽取所需值，尽快释放对 self.log 的借用（后续需要 &mut self）。
      let index_ty = self
        .log
        .txn_log_get::<TableType, TypeId>(metatable)
        .and_then(|mttv| mttv.props.get("__index"))
        .map(|prop| prop.type_deprecated());
      // C++ `if (!mttv) fail(std::nullopt);`——原实现（含上游）缺失 return，
      // 活跃路径下 metatable 必为 TableType（Option 命中）；None 属死路径，
      // 报错兜底，行为与 C++ 报错分支一致。
      if index_ty.is_none()
        && self
          .log
          .txn_log_get::<TableType, TypeId>(metatable)
          .is_none()
      {
        self.scalar_shape_fail(osuper_ty, osub_ty, None);
      }

      if let Some(ty) = index_ty {
        let mut child = self.unifier_make_child_unifier();
        child
          .try_unify_type_id_type_id_bool_bool_literal_properties(ty, super_ty, false, false, None);

        // To perform subtype <: free table unification, we have tried to unify (subtype's metatable) <: free table
        // There is a chance that it was unified with the original subtype, but then, (subtype's metatable) <: subtype could've failed
        // Here we check if we have a new supertype instead of the original free table and try original subtype <: new supertype check
        let new_super_ty = child.log.follow_type_id(super_ty);

        if super_ty != new_super_ty
          && self
            .can_unify_type_id_type_id(sub_ty, new_super_ty)
            .is_empty()
        {
          self
            .log
            .replace_type_id_t(super_ty, Type::new(TypeVariant::Bound(sub_ty)));
          return;
        }

        if let Some(e) = has_unification_too_complex(&child.errors) {
          self.report_error_type_error(e);
        } else if !child.errors.is_empty() {
          let first = child.errors[0].clone();
          self.scalar_shape_fail(osuper_ty, osub_ty, Some(first));
        }

        let child_errors_empty = child.errors.is_empty();
        self.log.concat(child.log);

        // To perform subtype <: free table unification, we have tried to unify (subtype's metatable) <: free table
        // We return success because subtype <: free table which means that correct unification is to replace free table with the subtype
        if child_errors_empty {
          self
            .log
            .replace_type_id_t(super_ty, Type::new(TypeVariant::Bound(sub_ty)));
        }

        return;
      } else {
        self.scalar_shape_fail(osuper_ty, osub_ty, None);
        return;
      }
    }

    self.unifier_report_type_mismatch(osuper_ty, osub_ty);
  }

  /// The `fail` lambda from `tryUnifyScalarShape`.
  fn scalar_shape_fail(&mut self, osuper_ty: TypeId, osub_ty: TypeId, e: Option<TypeError>) {
    self.unifier_report_type_mismatch_ext(
      osuper_ty,
      osub_ty,
      String::from("The given type's metatable does not satisfy the requirements."),
      e,
    );
  }
}
