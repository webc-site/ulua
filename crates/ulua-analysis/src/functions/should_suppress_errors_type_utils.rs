use crate::{
  enums::value::Value,
  functions::{
    finite::finite, flatten_type_pack::flatten_type_pack_id, follow_type, follow_type_pack,
    get_type, get_type_pack,
    should_suppress_errors_type_utils::should_suppress_errors as should_suppress_errors_not_null_normalizer_type_id,
  },
  records::{
    any_type::AnyType, error_suppression::ErrorSuppression, normalizer::Normalizer,
    type_function_instance_type::TypeFunctionInstanceType, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

/// Rust translation of Luau.Analysis::Analysis::TypeUtils.cpp:should_suppress_errors
///
/// B 档契约前移（先例 abs_index/vmb1/mainthread）：原 `*mut Normalizer` 形参对应 C++
/// `NotNull<Normalizer>&`——非空与调用期独占契约改由 `&mut Normalizer` 形参的引用有效性
/// 规则在调用点承载（调用点既有 `&mut` 字段借用或 `Handle::get_mut` 的存活句柄解引用）。
/// `ty` 仍须是类型 arena 中存活的 TypeId，callee 将对其 follow/取变体。
pub fn should_suppress_errors(normalizer: &mut Normalizer, ty: TypeId) -> ErrorSuppression {
  let ty = follow_type::follow(ty);

  if let Some(tfit) = get_type::get::<TypeFunctionInstanceType>(ty) {
    for &arg_ty in tfit.type_arguments.iter() {
      let Some(norm_type) = normalizer.try_normalize(arg_ty) else {
        return ErrorSuppression::from_value(Value::NormalizationFailed);
      };

      if norm_type.should_suppress_errors() {
        return ErrorSuppression::from_value(Value::Suppress);
      }
    }

    return ErrorSuppression::from_value(Value::DoNotSuppress);
  }

  let Some(norm_type) = normalizer.try_normalize(ty) else {
    return ErrorSuppression::from_value(Value::NormalizationFailed);
  };

  if norm_type.should_suppress_errors() {
    ErrorSuppression::from_value(Value::Suppress)
  } else {
    ErrorSuppression::from_value(Value::DoNotSuppress)
  }
}

pub(crate) fn should_suppress_errors_not_null_normalizer_type_pack_id(
  normalizer: &mut Normalizer,
  tp: TypePackId,
) -> ErrorSuppression {
  let tp = follow_type_pack::follow(tp);

  if let Some(vtp) = get_type_pack::get::<VariadicTypePack>(tp) {
    let ty = follow_type::follow(vtp.ty);
    if get_type::get::<AnyType>(ty).is_some() {
      return ErrorSuppression::from_value(Value::Suppress);
    }
  }

  let (tys, tail) = flatten_type_pack_id(tp);

  for ty in tys {
    // normalizer 透传同一可变借用（每次调用仅短暂重借用），tys 来自对存活类型包
    // tp 的 flatten，各元素均为 arena 存活 TypeId；单线程循环借用互不重叠。
    let result = should_suppress_errors_not_null_normalizer_type_id(normalizer, ty);
    if result != ErrorSuppression::from_value(Value::DoNotSuppress) {
      return result;
    }
  }

  if let Some(tail_tp) = tail
    && tp != tail_tp
    // tail_tp 是活类型包 tp 的 tail，即类型 arena 存活的 TypePackId（bump 分配
    // 不移动）；finite 的 log=None 等价 C++ 默认实参 nullptr，走无事务日志的
    // 直接 follow 路径，仅只读遍历 tail 链，无写操作。
    && finite(tail_tp, None)
  {
    return should_suppress_errors_not_null_normalizer_type_pack_id(normalizer, tail_tp);
  }

  ErrorSuppression::from_value(Value::DoNotSuppress)
}
