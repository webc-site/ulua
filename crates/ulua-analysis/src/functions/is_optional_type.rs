use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{builtin_types::BuiltinTypes, primitive_type::PrimitiveType, union_type::UnionType},
  type_aliases::type_id::TypeId,
};

/// # Safety
/// 调用方须保证 `builtin_types` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn is_optional_type(ty: TypeId, builtin_types: *mut BuiltinTypes) -> bool {
  unsafe {
    let ty = follow_type_id(ty);
    let builtin_types_ref = &*builtin_types;

    if ty == builtin_types_ref.nil_type
      || ty == builtin_types_ref.any_type
      || ty == builtin_types_ref.unknown_type
    {
      return true;
    } else if let Some(ptv) = get_type_id::<PrimitiveType>(ty).as_ref() {
      return ptv.r#type == PrimitiveType::NIL_TYPE;
    } else if let Some(utv) = get_type_id::<UnionType>(ty).as_ref() {
      for option in utv.options.iter() {
        let option = follow_type_id(*option);

        if option == builtin_types_ref.nil_type
          || option == builtin_types_ref.any_type
          || option == builtin_types_ref.unknown_type
        {
          return true;
        } else if let Some(ptv) = get_type_id::<PrimitiveType>(option).as_ref()
          && ptv.r#type == PrimitiveType::NIL_TYPE
        {
          return true;
        }
      }
      return false;
    }

    false
  }
}
