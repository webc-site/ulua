use crate::{
  records::{
    arena_handle::Handle, type_function_runtime::TypeFunctionRuntime,
    type_function_type::TypeFunctionType,
  },
  type_aliases::type_function_type_variant::TypeFunctionTypeVariant,
};

/// 对应 C++ `allocateTypeFunctionType`（TypeFunctionRuntime.cpp:362-366）：在 runtime 的
/// `type_arena`（TypedAllocator bump 块）上分配一个 `TypeFunctionType` 节点。
///
/// 形参由 cpp 的 `lua_State* L` 改为 `Handle<TypeFunctionRuntime>`：「取 runtime」这一步
/// （`get_type_function_runtime`，需解引用 VM thread data）留在调用方，本函数只剩 arena
/// bump 分配——`Handle` 已排除 null，节点地址在 runtime 存活期内不移动，故为 safe。
/// 返回的裸指针即 `TypeFunctionTypeId` 本体，存活期同 runtime。
pub fn allocate_type_function_type(
  runtime: Handle<TypeFunctionRuntime>,
  type_variant: TypeFunctionTypeVariant,
) -> *mut TypeFunctionType {
  runtime
    .get_mut()
    .type_arena
    .allocate(TypeFunctionType::new(type_variant))
}
