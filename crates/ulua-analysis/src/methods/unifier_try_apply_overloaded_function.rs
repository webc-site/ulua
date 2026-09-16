//! Source: `Analysis/src/Unifier.cpp` (Unifier::tryApplyOverloadedFunction, L1161-1224)
use alloc::string::String;

use crate::{
  functions::{
    get_type_alt_j::get_type_id, has_unification_too_complex::has_unification_too_complex,
  },
  records::{
    cannot_call_non_function::CannotCallNonFunction, function_type::FunctionType,
    generic_error::GenericError, normalized_function_type::NormalizedFunctionType,
    unifier::Unifier,
  },
  type_aliases::{type_error_data::TypeErrorData, type_id::TypeId, type_pack_id::TypePackId},
};

impl Unifier {
  /// `TypePackId Unifier::tryApplyOverloadedFunction(TypeId function, const NormalizedFunctionType& overloads, TypePackId args)`
  pub fn unifier_try_apply_overloaded_function(
    &mut self,
    function: TypeId,
    overloads: &NormalizedFunctionType,
    args: TypePackId,
  ) -> TypePackId {
    if overloads.is_never() {
      self.report_error_location_type_error_data(
        self.location,
        TypeErrorData::CannotCallNonFunction(CannotCallNonFunction { ty: function }),
      );
      return unsafe { (*self.builtin_types).error_type_pack };
    }

    let mut result: Option<TypePackId> = None;
    let mut first_fun: Option<&FunctionType> = None;

    let parts = overloads.parts.order.clone();
    for overload in parts {
      let Some(ftv) = get_type_id::<FunctionType>(overload) else {
        continue;
      };
      // TODO: instantiate generics?
      if !ftv.generics.is_empty() || !ftv.generic_packs.is_empty() {
        continue;
      }

      first_fun.get_or_insert(ftv);
      let mut inner_state = self.unifier_make_child_unifier();
      inner_state.try_unify_type_pack_id_type_pack_id_bool(args, ftv.arg_types, false);
      if inner_state.errors.is_empty() {
        self.log.concat(inner_state.log.clone());
        if let Some(res) = result {
          inner_state.log.clear();
          inner_state.try_unify_type_pack_id_type_pack_id_bool(res, ftv.ret_types, false);
          if inner_state.errors.is_empty() {
            self.log.concat(inner_state.log.clone());
          }
          // 泛型 type pack 不支持交集，交集可能失败；此时任意地取第一个匹配的 overload
          else if let Some(intersect) =
            unsafe { (*self.normalizer).intersection_of_type_packs(res, ftv.ret_types) }
          {
            result = Some(intersect);
          }
        } else {
          result = Some(ftv.ret_types);
        }
      } else if let Some(e) = has_unification_too_complex(&inner_state.errors) {
        self.report_error_type_error(e);
        return unsafe { (*self.builtin_types).error_recovery_type_pack(args) };
      }
    }

    if let Some(res) = result {
      res
    } else if let Some(first_fun) = first_fun {
      // TODO: better error reporting?
      // overload 解析的错误报告逻辑目前在 TypeInfer.cpp，是否移过来？
      self.report_error_location_type_error_data(
        self.location,
        TypeErrorData::GenericError(GenericError::new(String::from("No matching overload."))),
      );
      unsafe { (*self.builtin_types).error_recovery_type_pack(first_fun.ret_types) }
    } else {
      self.report_error_location_type_error_data(
        self.location,
        TypeErrorData::CannotCallNonFunction(CannotCallNonFunction { ty: function }),
      );
      unsafe { (*self.builtin_types).error_type_pack }
    }
  }
}
