use ulua_ast::records::location::Location;

use crate::{
  enums::{context_error::Context, value::Value},
  functions::should_suppress_errors_type_utils::{
    should_suppress_errors, should_suppress_errors_not_null_normalizer_type_pack_id,
  },
  records::{
    normalization_too_complex::NormalizationTooComplex, subtyping_result::SubtypingResult,
    type_checker_2::TypeChecker2, type_error::TypeError, type_mismatch::TypeMismatch,
    type_pack_mismatch::TypePackMismatch,
  },
  type_aliases::{type_error_data::TypeErrorData, type_id::TypeId, type_pack_id::TypePackId},
};

impl TypeChecker2 {
  /// 对 sub/super 类型对做不匹配错误的抑制判定与上报（cpp
  /// `TypeChecker2::explainError` 的 TypeId 重载）。
  pub(crate) fn explain_error_type_id_type_id_location_subtyping_result(
    &mut self,
    sub_ty: TypeId,
    super_ty: TypeId,
    location: Location,
    result: &SubtypingResult,
  ) {
    if result.is_error_suppressing {
      return;
    }

    // Safety: should_suppress_errors 的契约是 normalizer 非空且在调用期内
    // 独占可用（C++ `NotNull<Normalizer>&` 形参）；这里传 &mut self.normalizer
    // 取址的裸指针必非空，借用只存活于本次调用、单线程串行无别名。sub_ty
    // 来自类型检查记录，为 arena 存活 TypeId，callee 仅 follow+normalize
    // （写的是 normalizer 自身缓存，不触碰类型节点）。
    let suppression_sub = unsafe { should_suppress_errors(&mut self.normalizer, sub_ty) };
    // Safety: 同上——super_ty 亦为 arena 存活 TypeId，&mut self.normalizer
    // 取址非空；两次的可变借用先后接续、互不重叠（原式本就先求值两侧再
    // or_else，改写为两条语句保持同一求值顺序）。
    let suppression_super = unsafe { should_suppress_errors(&mut self.normalizer, super_ty) };
    let suppression = suppression_sub.or_else(&suppression_super);

    match suppression.error_suppression_value() {
      Value::Suppress => return,
      Value::NormalizationFailed => {
        self.report_error_type_error(TypeError::type_error_location_type_error_data(
          location,
          TypeErrorData::NormalizationTooComplex(NormalizationTooComplex { _unused: None }),
        ));
      }
      _ => {}
    }

    let reasonings = self.explain_reasonings_type_id_type_id_location_subtyping_result(
      sub_ty, super_ty, location, result,
    );

    if !reasonings.suppressed {
      self.report_error_type_error(TypeError::type_error_location_type_error_data(
        location,
        TypeErrorData::TypeMismatch(TypeMismatch {
          wanted_type: super_ty,
          given_type: sub_ty,
          reason: reasonings.to_string(),
          error: None,
          context: Context::COVARIANT,
        }),
      ));
    }
  }

  pub fn explain_error_type_pack_id_type_pack_id_location_subtyping_result(
    &mut self,
    sub_tp: TypePackId,
    super_tp: TypePackId,
    location: Location,
    result: &SubtypingResult,
  ) {
    if result.is_error_suppressing {
      return;
    }

    let suppression =
      should_suppress_errors_not_null_normalizer_type_pack_id(&mut self.normalizer, sub_tp)
        .or_else(&should_suppress_errors_not_null_normalizer_type_pack_id(
          &mut self.normalizer,
          super_tp,
        ));

    match suppression.error_suppression_value() {
      Value::Suppress => return,
      Value::NormalizationFailed => {
        self.report_error_type_error(TypeError::type_error_location_type_error_data(
          location,
          TypeErrorData::NormalizationTooComplex(NormalizationTooComplex { _unused: None }),
        ));
      }
      _ => {}
    }

    let reasonings = self.explain_reasonings_type_pack_id_type_pack_id_location_subtyping_result(
      sub_tp, super_tp, location, result,
    );

    if !reasonings.suppressed {
      self.report_error_type_error(TypeError::type_error_location_type_error_data(
        location,
        TypeErrorData::TypePackMismatch(TypePackMismatch {
          wanted_tp: super_tp,
          given_tp: sub_tp,
          reason: reasonings.to_string(),
        }),
      ));
    }
  }
}
