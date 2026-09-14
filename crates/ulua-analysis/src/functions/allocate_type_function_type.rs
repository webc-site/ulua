use crate::{
  functions::get_type_function_runtime::get_type_function_runtime,
  records::type_function_type::TypeFunctionType,
  type_aliases::{lua_state::LuaState, type_function_type_variant::TypeFunctionTypeVariant},
};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn allocate_type_function_type(
  l: *mut LuaState,
  type_variant: TypeFunctionTypeVariant,
) -> *mut TypeFunctionType {
  unsafe {
    let ctx = get_type_function_runtime(l);
    (*ctx)
      .type_arena
      .allocate(TypeFunctionType::new(type_variant))
  }
}
