use core::ptr::{NonNull, null_mut};

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
/// # Safety
/// `normalizer` 必须是存活且在调用期内独占可用的 `*mut Normalizer`（对应 C++
/// `NotNull<Normalizer>&` 形参——非空由 NotNull 约定，独占由单线程调用链保证）；
/// `ty` 必须是类型 arena 中存活的 TypeId，callee 将对其 follow/取变体。
pub unsafe fn should_suppress_errors(normalizer: *mut Normalizer, ty: TypeId) -> ErrorSuppression {
  // Safety: 契约要求 normalizer 非空且调用期内独占——两处调用方均由
  // `&mut self.normalizer`（拥有型字段取址，恒非空）或同型裸指针透传传入；
  // new_unchecked 仅编码该非空约定，重建的 &mut 借用止于本函数返回，单线程
  // 内无并存别名。
  let normalizer = unsafe { NonNull::new_unchecked(normalizer).as_mut() };

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
  normalizer: *mut Normalizer,
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
    // Safety: normalizer 是透传的同一枚裸指针（本函数自身的 unsafe fn 调用
    // 契约已论证其非空与调用期独占）；tys 来自对存活类型包 tp 的 flatten，
    // 各元素均为 arena 存活 TypeId。被调方每次仅短暂重建 &mut Normalizer，
    // 循环迭代间借用互不重叠，单线程无别名。
    let result = unsafe { should_suppress_errors_not_null_normalizer_type_id(normalizer, ty) };
    if result != ErrorSuppression::from_value(Value::DoNotSuppress) {
      return result;
    }
  }

  if let Some(tail_tp) = tail
    && tp != tail_tp
    // Safety: tail_tp 是活类型包 tp 的 tail，即类型 arena 存活的 TypePackId
    // （bump 分配不移动）；log 传 null_mut 等价 C++ 默认实参 nullptr，finite
    // 走无事务日志的直接 follow 路径，仅只读遍历 tail 链，无写操作。
    && unsafe { finite(tail_tp, null_mut()) }
  {
    return should_suppress_errors_not_null_normalizer_type_pack_id(normalizer, tail_tp);
  }

  ErrorSuppression::from_value(Value::DoNotSuppress)
}
