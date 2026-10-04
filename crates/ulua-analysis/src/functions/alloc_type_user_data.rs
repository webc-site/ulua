use core::mem::size_of;

use ulua_vm::{
  functions::{lua_l_checkstack::lua_l_checkstack, lua_newuserdatatagged::lua_newuserdatatagged},
  records::lua_state::LuaState,
};

use crate::{
  functions::{
    allocate_type_function_type::allocate_type_function_type,
    get_type_function_runtime::get_type_function_runtime, lua_names::TYPE,
  },
  records::type_function_type::TypeFunctionType,
  type_aliases::{
    type_function_type_id::TypeFunctionTypeId, type_function_type_variant::TypeFunctionTypeVariant,
  },
};
const K_TYPE_USERDATA_TAG: i32 = 42;

/// 对应 C++ `allocTypeUserData`（TypeFunctionRuntime.cpp:383-391）：压入承载
/// `TypeFunctionTypeId` 的 tagged 用户数据、写入 `frozen` 并挂 `"type"` 元表。
///
/// 本函数是 safe fn：形参全部为受检引用/owned 值，体内裸操作（`l.as_mut_ptr()` 镜像
/// 透传、`lua_newuserdatatagged` 返回体的写入、`*ptr = type_id` 与 `(*type_ptr).frozen`
/// 两处 arena 写）解引用的指针全部源自本次调用体内获取——FFI 返回值或
/// `allocate_type_function_type` 新分配的 arena 节点（bump 块地址不移动），调用方
/// 无需承担任何内存安全前提。
///
/// 调用序契约（正确性，非内存安全）：`l` 须为类型函数 runtime 会话期存活的状态
/// （cpp 侧 `lua_State* L` 的同款隐含契约），其 mainthread 的 thread data 已在 runtime
/// 安装期写入 `TypeFunctionRuntime`——未挂载时 `get_type_function_runtime` 的 `expect`
/// 收敛为确定性 panic，不构成 UB；本调用发生在单线程 VM 步进内，栈可按
/// `lua_l_checkstack` 语义扩容。
pub(crate) fn alloc_type_user_data(
  l: &mut LuaState,
  type_variant: TypeFunctionTypeVariant,
  frozen: bool,
) {
  lua_l_checkstack(l, 2, "allocating type");

  // Safety: `l.as_mut_ptr()` 是 `&mut l` 同一对象的镜像透传（存活与独占由引用承载）；
  // `lua_newuserdatatagged` 是 vm 侧 C 形态门面，分配失败走 VM 错误路径，成功返回按最大
  // 对齐的 `K_TYPE_USERDATA_TAG` 用户数据体，容量恰为 `size_of::<TypeFunctionTypeId>()`，
  // 故其后的 `*ptr = type_id` 写入界内且存活。前置由上方 `lua_l_checkstack` 保证栈可增 2 槽。
  let ptr = unsafe {
    lua_newuserdatatagged(
      l.as_mut_ptr(),
      size_of::<TypeFunctionTypeId>(),
      K_TYPE_USERDATA_TAG,
    )
  } as *mut TypeFunctionTypeId;

  let runtime = get_type_function_runtime(l).expect("runtime 于注册阶段挂载，会话内恒非空");
  let type_id = allocate_type_function_type(runtime, type_variant);

  // Safety: `type_id` 由 runtime 的 type_arena（TypedAllocator bump 块）分配，地址不移动
  // 且比 `l` 长寿；`TypeFunctionTypeId` 即 `*const TypeFunctionType` 裸值，转 `*mut` 后写
  // `frozen` 指向的正是该 arena 的可变内存。
  unsafe {
    *ptr = type_id;
    (*(*ptr as *mut TypeFunctionType)).frozen = frozen;
  }

  // TYPE 为 NUL 结尾字节串；元表缺失时 set_metatable 为无操作。
  l.get_metatable_by_bytes(TYPE);
  l.set_metatable(-2);
}
