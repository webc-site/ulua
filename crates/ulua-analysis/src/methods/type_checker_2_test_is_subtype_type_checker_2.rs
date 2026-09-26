use alloc::vec::Vec;
use core::mem::take;

use ulua_ast::records::location::Location;

use crate::{
  records::{
    normalization_too_complex::NormalizationTooComplex, scope::Scope,
    subtyping_result::SubtypingResult, type_checker_2::TypeChecker2,
  },
  type_aliases::{type_error_data::TypeErrorData, type_id::TypeId, type_pack_id::TypePackId},
};

impl TypeChecker2 {
  pub fn test_is_subtype_type_id_type_id_location(
    &mut self,
    sub_ty: TypeId,
    super_ty: TypeId,
    location: Location,
  ) -> bool {
    let scope: *mut Scope = self.find_innermost_scope(location);
    // subtyping 字段已句柄化：`subtyping_mut` 收口 Option 判空与解引用契约
    // （构造期 `wire_self_pointers` 回填后恒 Some），借用止于本语句。`scope` 由
    // `find_innermost_scope` 返回该 location 命中的最内层存活 `Scope` arena 指针
    // （`not_null_scope` 形参契约）。单线程独占驱动无别名。
    let mut r: SubtypingResult = self
      .subtyping_mut()
      .is_subtype_type_id_type_id_not_null_scope(sub_ty, super_ty, scope);

    if r.is_error_suppressing {
      return r.is_subtype;
    }

    for error in &mut r.errors {
      error.location = location;
    }

    self.report_errors(take(&mut r.errors));

    if r.normalization_too_complex {
      self.report_error_type_error_data_location(
        TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
        &location,
      );
    }

    if !r.is_subtype {
      self.explain_error_type_id_type_id_location_subtyping_result(sub_ty, super_ty, location, &r);
    }

    r.is_subtype
  }

  pub fn test_is_subtype_type_pack_id_type_pack_id_location(
    &mut self,
    sub_tp: TypePackId,
    super_tp: TypePackId,
    location: Location,
  ) -> bool {
    let scope: *mut Scope = self.find_innermost_scope(location);
    // C++: subtyping->isSubtype(sub_ty, super_ty, scope, {}) — empty bindableGenerics.
    let empty_bindable: Vec<TypeId> = Vec::new();
    // 同上——`subtyping_mut` 访问器收口句柄判空与解引用，借用止于本语句。
    // `scope` 是 `find_innermost_scope` 命中的存活 `Scope` arena 指针，`empty_bindable` 为
    // 栈上自有 `Vec` 的共享引用；单线程独占驱动下无并存别名。
    let mut r: SubtypingResult = self
      .subtyping_mut()
      .is_subtype_type_pack_id_type_pack_id_not_null_scope_vector_type_id(
        sub_tp,
        super_tp,
        scope,
        &empty_bindable,
      );

    if !self.is_error_suppressing_location_type_pack_id(location, sub_tp) {
      for e in &mut r.errors {
        e.location = location;
      }
    }
    self.report_errors(take(&mut r.errors));

    if r.normalization_too_complex {
      self.report_error_type_error_data_location(
        TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
        &location,
      );
    }

    if !r.is_subtype {
      self.explain_error_type_pack_id_type_pack_id_location_subtyping_result(
        sub_tp, super_tp, location, &r,
      );
    }

    r.is_subtype
  }
}
