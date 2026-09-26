use crate::{
  records::{
    arena_handle::Handle, type_function_runtime::TypeFunctionRuntime,
    type_function_type_pack_var::TypeFunctionTypePackVar,
  },
  type_aliases::type_function_type_pack_variant::TypeFunctionTypePackVariant,
};

/// 对应 C++ `allocateTypeFunctionTypePack`（TypeFunctionRuntime.cpp:368-372）：在 runtime
/// 的 `type_pack_arena`（TypedAllocator bump 块）上分配一个 `TypeFunctionTypePackVar` 节点。
///
/// 形参由 cpp 的 `lua_State* L` 改为 `Handle<TypeFunctionRuntime>`，理由同
/// `allocate_type_function_type`：取 runtime 的解引用留在调用方，本函数只剩 arena bump
/// 分配，返回的节点地址在 runtime 存活期内不移动。
pub fn allocate_type_function_type_pack(
  runtime: Handle<TypeFunctionRuntime>,
  type_variant: TypeFunctionTypePackVariant,
) -> *mut TypeFunctionTypePackVar {
  runtime
    .get_mut()
    .type_pack_arena
    .allocate(TypeFunctionTypePackVar::new(type_variant))
}
