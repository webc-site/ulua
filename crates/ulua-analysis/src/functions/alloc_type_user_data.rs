use core::mem::size_of;

use ulua_vm::{
  functions::{
    lua_l_checkstack::lua_l_checkstack, lua_newuserdatatagged::lua_newuserdatatagged,
    lua_setmetatable::lua_setmetatable,
  },
  macros::lua_l_getmetatable::lua_l_getmetatable,
  records::lua_state,
};

use crate::{
  functions::{
    allocate_type_function_type::allocate_type_function_type,
    get_type_function_runtime::get_type_function_runtime, lua_names::TYPE,
  },
  records::{arena_handle::Handle, type_function_type::TypeFunctionType},
  type_aliases::{
    lua_state::LuaState, type_function_type_id::TypeFunctionTypeId,
    type_function_type_variant::TypeFunctionTypeVariant,
  },
};
const K_TYPE_USERDATA_TAG: i32 = 42;

/// 对应 C++ `allocTypeUserData`（TypeFunctionRuntime.cpp:383-391）：压入承载
/// `TypeFunctionTypeId` 的 tagged 用户数据、写入 `frozen` 并挂 `"type"` 元表。
///
/// # Safety
/// `l` 须为类型函数 runtime 会话期存活的 `lua_State` 裸指针（cpp 侧 `lua_State* L`
/// 的同款隐含契约）：`lua_mainthread(l)` 的 thread data 已在 runtime 安装期写入
/// 非空 `TypeFunctionRuntime`，且本调用发生在单线程 VM 步进内、栈可按
/// `lua_l_checkstack` 语义扩容；函数体内对这些指针的解引用全部收在下方 unsafe
/// 块中，其前提即本段所列。
pub unsafe fn alloc_type_user_data(
  l: *mut LuaState,
  type_variant: TypeFunctionTypeVariant,
  frozen: bool,
) {
  // Safety: l 是 Lua VM 调注册闭包时传入的存活 lua_State；lua_l_checkstack 先保证栈可增 2 槽；
  // lua_newuserdatatagged 分配失败走 VM 错误路径、成功返回按最大对齐的 K_TYPE_USERDATA_TAG
  // 用户数据体，容量恰为 size_of::<TypeFunctionTypeId>()；get_type_function_runtime 取回注册期
  // 写入主线程 thread data 的非空 TypeFunctionRuntime（null 时 Handle::from_ptr 直接 panic），
  // type_id 由其 type_arena（TypedAllocator bump 块）分配、地址不移动且比 l 长寿；
  // TypeFunctionTypeId 即 *const TypeFunctionType 裸值，转 *mut 后写 frozen 指向的是该 arena
  // 可变内存；TYPE 为 NUL 结尾字节串，元表缺失时 lua_setmetatable 为无操作。
  unsafe {
    lua_l_checkstack(l as *mut lua_state::LuaState, 2, "allocating type");

    let ptr = lua_newuserdatatagged(
      l as *mut lua_state::LuaState,
      size_of::<TypeFunctionTypeId>(),
      K_TYPE_USERDATA_TAG,
    ) as *mut TypeFunctionTypeId;

    let runtime = Handle::from_ptr(get_type_function_runtime(l));
    let type_id = allocate_type_function_type(runtime, type_variant);
    *ptr = type_id;

    let type_ptr = *ptr as *mut TypeFunctionType;
    (*type_ptr).frozen = frozen;

    lua_l_getmetatable(l as *mut lua_state::LuaState, TYPE.as_ptr().cast());
    lua_setmetatable(l as *mut lua_state::LuaState, -2);
  }
}
