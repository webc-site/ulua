use crate::{
  functions::{begin_type::begin_union_type, follow_type, get_type},
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, primitive_type::PrimitiveType,
    union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

/// # Safety
/// 调用方须保证 `builtin_types` 句柄目标存活且满足 C++ 原实现的调用契约。
pub unsafe fn is_optional_type(ty: TypeId, builtin_types: Handle<BuiltinTypes>) -> bool {
  // `builtin_types` 为常驻 BuiltinTypes 的句柄（非空、比本次调用长寿），
  // `get()` 重建只读共享借用，仅读取 nil_type/any_type/unknown_type 等常量 TypeId。
  // `ty` 经 follow_type_id 收敛后是 arena 内存活 TypeId（bump 分配、块地址不移动），
  // `get_type::get::<T>(ty).as_ref()` 按 class-index 判定、Some 分支只读 `r#type`。单线程只读，
  // 无别名冲突。
  // 体内全部为 safe 调用（Handle::get 收拢解引用），无需 unsafe 块。
  let ty = follow_type::follow(ty);
  let builtin_types_ref = builtin_types.get();

  if ty == builtin_types_ref.nil_type
    || ty == builtin_types_ref.any_type
    || ty == builtin_types_ref.unknown_type
  {
    return true;
  } else if let Some(ptv) = get_type::get::<PrimitiveType>(ty).as_ref() {
    return ptv.r#type == PrimitiveType::NIL_TYPE;
  } else if let Some(utv) = get_type::get::<UnionType>(ty).as_ref() {
    // C++ `for (TypeId option : ut)`——UnionTypeIterator 展平嵌套 union
    // 并 follow,裸遍历 options 会漏掉嵌套成员。
    for option in begin_union_type(utv) {
      let option = follow_type::follow(option);

      if option == builtin_types_ref.nil_type
        || option == builtin_types_ref.any_type
        || option == builtin_types_ref.unknown_type
      {
        return true;
      } else if let Some(ptv) = get_type::get::<PrimitiveType>(option).as_ref()
        && ptv.r#type == PrimitiveType::NIL_TYPE
      {
        return true;
      }
    }
    return false;
  }

  false
}
