use ulua_ast::records::location::Location;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{context_error::Context, subtyping_variance::SubtypingVariance, value::Value},
  functions::should_suppress_errors_type_utils::{
    should_suppress_errors, should_suppress_errors_not_null_normalizer_type_pack_id,
  },
  records::{
    normalization_too_complex::NormalizationTooComplex, overload_resolver::OverloadResolver,
    subtyping_reasoning::SubtypingReasoning, type_error::TypeError, type_mismatch::TypeMismatch,
    type_pack_mismatch::TypePackMismatch,
  },
  type_aliases::{
    error_vec::ErrorVec, module_name_type::ModuleName, type_error_data::TypeErrorData,
    type_id::TypeId, type_or_pack::TypeOrPack, type_pack_id::TypePackId,
  },
};

impl OverloadResolver<'_> {
  /// 仅在 wanted/given 均存在时按抑制策略落错误；normalizer 为裸指针沿用
  /// records/ 中的字段定义，调用处由构造方保证有效。
  pub fn maybe_emplace_error_error_vec_location_module_name_subtyping_reasoning_optional_type_id_optional_type_id(
    &self,
    errors: &mut ErrorVec,
    arg_location: Location,
    module_name: &ModuleName,
    reason: &SubtypingReasoning,
    wanted_type: Option<TypeId>,
    given_type: Option<TypeId>,
  ) {
    if let (Some(wanted_type), Some(given_type)) = (wanted_type, given_type) {
      // SAFETY: normalizer 指向 TypeChecker 持有的 Normalizer（C++ 同契约）。
      let suppression = unsafe { should_suppress_errors(self.normalizer.as_ptr(), wanted_type) }
        .or_else(&unsafe { should_suppress_errors(self.normalizer.as_ptr(), given_type) });

      // C++ switch on the suppression policy. The NormalizationFailed case
      // emplaces NormalizationTooComplex and then *falls through* to the
      // DoNotSuppress case, which emits the type mismatch.
      let value = suppression.error_suppression_value();
      if value == Value::Suppress {
        return;
      }

      if value == Value::NormalizationFailed {
        errors.push(TypeError::type_error_location_module_name_type_error_data(
          arg_location,
          module_name.clone(),
          TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
        ));
        // intentionally fallthrough here since we couldn't prove this was error-suppressing
      }

      // DoNotSuppress (and fallthrough from NormalizationFailed):
      // TODO extract location from the SubtypingResult path and argExprs
      match reason.variance {
        SubtypingVariance::Covariant | SubtypingVariance::Contravariant => {
          errors.push(TypeError::type_error_location_module_name_type_error_data(
            arg_location,
            module_name.clone(),
            TypeErrorData::TypeMismatch(TypeMismatch::from_wanted_given_context(
              wanted_type,
              given_type,
              Context::CovariantContext,
            )),
          ));
        }
        SubtypingVariance::Invariant => {
          errors.push(TypeError::type_error_location_module_name_type_error_data(
            arg_location,
            module_name.clone(),
            TypeErrorData::TypeMismatch(TypeMismatch::from_wanted_given_context(
              wanted_type,
              given_type,
              Context::InvariantContext,
            )),
          ));
        }
        _ => {
          LUAU_ASSERT!(false);
        }
      }
    }
  }

  /// 仅在 wanted/given 均存在时按抑制策略落错误。
  pub fn maybe_emplace_error_error_vec_location_module_name_subtyping_reasoning_optional_type_pack_id_optional_type_pack_id(
    &self,
    errors: &mut ErrorVec,
    arg_location: Location,
    _module_name: &ModuleName,
    _reason: &SubtypingReasoning,
    wanted_tp: Option<TypePackId>,
    given_tp: Option<TypePackId>,
  ) {
    let (Some(wanted_tp), Some(given_tp)) = (wanted_tp, given_tp) else {
      return;
    };

    let suppression =
      should_suppress_errors_not_null_normalizer_type_pack_id(self.normalizer.as_ptr(), wanted_tp)
        .or_else(&should_suppress_errors_not_null_normalizer_type_pack_id(
          self.normalizer.as_ptr(),
          given_tp,
        ));

    match suppression.error_suppression_value() {
      Value::Suppress => {}
      Value::NormalizationFailed => errors.push(TypeError::type_error_location_type_error_data(
        arg_location,
        TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
      )),
      _ => errors.push(TypeError::type_error_location_type_error_data(
        arg_location,
        TypeErrorData::TypePackMismatch(TypePackMismatch {
          wanted_tp,
          given_tp,
          reason: String::new(),
        }),
      )),
    }
  }

  /// 按wanted/given 的具体类别（Type 或 TypePack）分发到对应落错误实现。
  pub(crate) fn maybe_emplace_error_error_vec_location_module_name_subtyping_reasoning_optional_type_or_pack_optional_type_or_pack(
    &self,
    errors: &mut ErrorVec,
    arg_location: Location,
    module_name: &ModuleName,
    reason: &SubtypingReasoning,
    wanted_type: Option<TypeOrPack>,
    given_type: Option<TypeOrPack>,
  ) {
    let (Some(wanted_type), Some(given_type)) = (wanted_type, given_type) else {
      return;
    };

    if let (Some(&wanted_ty), Some(&given_ty)) = (wanted_type.get_if_0(), given_type.get_if_0()) {
      self.maybe_emplace_error_error_vec_location_module_name_subtyping_reasoning_optional_type_id_optional_type_id(
        errors,
        arg_location,
        module_name,
        reason,
        Some(wanted_ty),
        Some(given_ty),
      );
      return;
    }

    if let (Some(&wanted_tp), Some(&given_tp)) = (wanted_type.get_if_1(), given_type.get_if_1()) {
      self.maybe_emplace_error_error_vec_location_module_name_subtyping_reasoning_optional_type_pack_id_optional_type_pack_id(
        errors,
        arg_location,
        module_name,
        reason,
        Some(wanted_tp),
        Some(given_tp),
      );
    }
  }
}
