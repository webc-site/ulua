

use core::mem::size_of;
use ulua_vm::{functions::{lua_l_checkstack::lua_l_checkstack, lua_newuserdatatagged::lua_newuserdatatagged}};
use crate::{functions::{allocate_type_function_type::allocate_type_function_type, get_type_function_runtime::get_type_function_runtime, lua_names::TYPE}, records::{arena_handle::Handle, type_function_type::TypeFunctionType}, type_aliases::{type_function_type_id::TypeFunctionTypeId, type_function_type_variant::TypeFunctionTypeVariant}};
use ulua_vm::records::lua_state::LuaState;
const K_TYPE_USERDATA_TAG: i32 = 42;

/// 对应 C++ `allocTypeUserData`（TypeFunctionRuntime.cpp:383-391）：压入承载
/// `TypeFunctionTypeId` 的 tagged 用户数据、写入 `frozen` 并挂 `"type"` 元表。
///
/// # Safety
/// `l` 的存活/独占前提已由 `&mut` 接收者类型承载（r16-v45 收形）；屏障仍保留是因为体内有真实
/// 裸操作：入口一次就地转手 `lp = l.as_mut_ptr()`（两枚不透明
/// 镜像类型间的地址不变透传），`lua_newuserdatatagged` 返回的用户数据体写入、
/// `get_type_function_runtime` 取回的主线程 thread data、以及 `*ptr = type_id` 与
/// `(*type_ptr).frozen` 两处 arena 写皆按裸指针形制进行。余下调用序前提：`l` 须为类型函数
/// runtime 会话期存活的状态（cpp 侧 `lua_State* L` 的同款隐含契约），其 mainthread 的 thread
/// data 已在 runtime 安装期写入非空 `TypeFunctionRuntime`，且本调用发生在单线程 VM 步进内、
/// 栈可按 `lua_l_checkstack` 语义扩容。
pub(crate) unsafe fn alloc_type_user_data(
  l: &mut LuaState,
  type_variant: TypeFunctionTypeVariant,
  frozen: bool,
) {
  // Safety: `l` 由 `&mut` 保证存活且本次调用独占，转手后的 `lp` 即同一地址在 vm 侧镜像类型上的
  // 裸形；lua_l_checkstack 先保证栈可增 2 槽；
  // lua_newuserdatatagged 分配失败走 VM 错误路径、成功返回按最大对齐的 K_TYPE_USERDATA_TAG
  // 用户数据体，容量恰为 size_of::<TypeFunctionTypeId>()；get_type_function_runtime 取回注册期
  // 写入主线程 thread data 的非空 TypeFunctionRuntime（null 时 Handle::from_ptr 直接 panic），
  // type_id 由其 type_arena（TypedAllocator bump 块）分配、地址不移动且比 l 长寿；
  // TypeFunctionTypeId 即 *const TypeFunctionType 裸值，转 *mut 后写 frozen 指向的是该 arena
  // 可变内存；TYPE 为 NUL 结尾字节串，元表缺失时 lua_setmetatable 为无操作。
  unsafe {
    let lp = l.as_mut_ptr();

    lua_l_checkstack(&mut *lp, 2, "allocating type");

    let ptr = lua_newuserdatatagged(lp, size_of::<TypeFunctionTypeId>(), K_TYPE_USERDATA_TAG)
      as *mut TypeFunctionTypeId;

    let runtime = Handle::from_ptr(get_type_function_runtime(&mut *l));
    let type_id = allocate_type_function_type(runtime, type_variant);
    *ptr = type_id;

    let type_ptr = *ptr as *mut TypeFunctionType;
    (*type_ptr).frozen = frozen;

    (*lp).get_metatable_by_bytes(TYPE);
    (*lp).set_metatable(-2);
  }
}
