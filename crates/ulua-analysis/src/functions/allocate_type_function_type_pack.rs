use crate::{
  functions::get_type_function_runtime::get_type_function_runtime,
  records::type_function_type_pack_var::TypeFunctionTypePackVar,
  type_aliases::{
    lua_state::LuaState, type_function_type_pack_variant::TypeFunctionTypePackVariant,
  },
};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn allocate_type_function_type_pack(
  l: *mut LuaState,
  type_variant: TypeFunctionTypePackVariant,
) -> *mut TypeFunctionTypePackVar {
  unsafe {
    let ctx = get_type_function_runtime(l);
    (*ctx)
      .type_pack_arena
      .allocate(TypeFunctionTypePackVar::new(type_variant))
  }
}
