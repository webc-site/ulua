//! Source: `Analysis/src/OverloadResolver.cpp:660-696` (hand-ported)
use ulua_ast::records::location::Location;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{context_error::Context, subtyping_variance::SubtypingVariance, value::Value},
  functions::should_suppress_errors_type_utils::should_suppress_errors,
  records::{
    normalization_too_complex::NormalizationTooComplex, overload_resolver::OverloadResolver,
    subtyping_reasoning::SubtypingReasoning, type_error::TypeError, type_mismatch::TypeMismatch,
  },
  type_aliases::{
    error_vec::ErrorVec, module_name_type::ModuleName, type_error_data::TypeErrorData,
    type_id::TypeId,
  },
};

impl OverloadResolver {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn maybe_emplace_error_error_vec_location_module_name_subtyping_reasoning_optional_type_id_optional_type_id(
    &self,
    errors: *mut ErrorVec,
    arg_location: Location,
    module_name: &ModuleName,
    reason: *const SubtypingReasoning,
    wanted_type: Option<TypeId>,
    given_type: Option<TypeId>,
  ) {
    if let (Some(wanted_type), Some(given_type)) = (wanted_type, given_type) {
      let suppression = unsafe { should_suppress_errors(self.normalizer, wanted_type) }
        .or_else(&unsafe { should_suppress_errors(self.normalizer, given_type) });

      // C++ switch on the suppression policy. The NormalizationFailed case
      // emplaces NormalizationTooComplex and then *falls through* to the
      // DoNotSuppress case, which emits the type mismatch.
      let value = suppression.error_suppression_value();
      if value == Value::Suppress {
        return;
      }

      if value == Value::NormalizationFailed {
        unsafe {
          (*errors).push(TypeError::type_error_location_module_name_type_error_data(
            arg_location,
            module_name.clone(),
            TypeErrorData::NormalizationTooComplex(NormalizationTooComplex { _unused: None }),
          ));
        }
        // intentionally fallthrough here since we couldn't prove this was error-suppressing
      }

      // DoNotSuppress (and fallthrough from NormalizationFailed):
      // TODO extract location from the SubtypingResult path and argExprs
      let reason = unsafe { &*reason };
      match reason.variance {
        SubtypingVariance::Covariant | SubtypingVariance::Contravariant => unsafe {
          (*errors).push(TypeError::type_error_location_module_name_type_error_data(
            arg_location,
            module_name.clone(),
            TypeErrorData::TypeMismatch(TypeMismatch::from_wanted_given_context(
              wanted_type,
              given_type,
              Context::CovariantContext,
            )),
          ));
        },
        SubtypingVariance::Invariant => unsafe {
          (*errors).push(TypeError::type_error_location_module_name_type_error_data(
            arg_location,
            module_name.clone(),
            TypeErrorData::TypeMismatch(TypeMismatch::from_wanted_given_context(
              wanted_type,
              given_type,
              Context::InvariantContext,
            )),
          ));
        },
        _ => {
          LUAU_ASSERT!(false);
        }
      }
    }
  }
}
