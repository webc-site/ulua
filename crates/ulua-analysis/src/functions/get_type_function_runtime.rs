use core::ptr::null;

use ulua_common::macros::luau_assert::LUAU_ASSERT;
use ulua_vm::{
  functions::{lua_getthreaddata::lua_getthreaddata, lua_mainthread::lua_mainthread},
  records::lua_state::LuaState,
};

use crate::{
  records::type_function_runtime::TypeFunctionRuntime,
  type_aliases::{
    type_function_type_id::TypeFunctionTypeId, type_function_type_pack_id::TypeFunctionTypePackId,
    type_function_type_pack_variant::TypeFunctionTypePackVariantMember,
    type_function_type_variant::TypeFunctionTypeVariantMember,
  },
};

/// 取回挂载在 `l` 所属主线程 userdatum 上的 `TypeFunctionRuntime` 裸句柄。
///
/// # Safety
/// `l` 必须是注册阶段经 `setTypeFunctionEnvironment` 在其主线程 userdatum 上挂载了
/// `TypeFunctionRuntime` 的 `lua_State*`（可为协程，内部取 `lua_mainthread`）；返回的裸指针
/// 指向该 userdatum 内部对象，仅在 VM 存活期有效，调用方不得跨调用保存。对应 C++
/// `TypeFunctionRuntime* getTypeFunctionRuntime(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:357`）。
pub(crate) unsafe fn get_type_function_runtime(l: &mut LuaState) -> *mut TypeFunctionRuntime {
  // Safety: `&*main_thread` 解引用 lua_mainthread 返回的 `*mut LuaState`；
  // lua_mainthread 返回状态所属主线程的有效指针，lua_getthreaddata 返回注册时
  // 写入主线程的 userdata。
  unsafe {
    let main_thread = lua_mainthread(&*l);
    let data = lua_getthreaddata(&*main_thread);
    data as *mut TypeFunctionRuntime
  }
}

// Source: `Analysis/include/Luau/TypeFunctionRuntime.h:167-173` (hand-ported)
/// C++ `template<typename T> const T* get(TypeFunctionTypePackId tv)`.
///
/// # Safety
/// `tv` 须为空，或指向 `TypeFunctionRuntime` 的 `type_pack_arena` 分配的存活 `TypeFunctionTypePackVar`
/// 节点（地址随 arena 存活期不迁移）；调用方单线程独占该 arena，且按 `repr(C)` class-index 命中时
/// 返回的内部字段指针在 arena 存活期内有效。
pub(crate) unsafe fn get_type_function_type_pack_id<T: TypeFunctionTypePackVariantMember>(
  tv: TypeFunctionTypePackId,
) -> *const T {
  LUAU_ASSERT!(!tv.is_null());
  if tv.is_null() {
    return null();
  }
  // Safety: 上方判空已确保 `tv` 非 null——它是 `TypeFunctionTypePackId = *const ...`，
  // 指向 type function arena 中存活且对齐的打包变体节点，故 `&(*tv).type_variant` 重建
  // 共享引用有效。`T::get_if` 按 variant 标签匹配：命中 ⇒ 该标签确属 `T`，`repr(C)` 布局
  // 下 `&T` 与所指变体首字段基址重合，回 `*const T` 类型正确；未命中返回 null。全程只读。
  unsafe {
    match T::get_if(&(*tv).type_variant) {
      Some(r) => r as *const T,
      None => null(),
    }
  }
}

// Source: `Analysis/include/Luau/TypeFunctionRuntime.h:275-281` (hand-ported)
/// C++ `template<typename T> const T* get(TypeFunctionTypeId tv)`.
///
/// # Safety
/// `tv` 须为空，或指向 `TypeFunctionRuntime` 的 `type_arena` 分配的存活 `TypeFunctionType`
/// 节点（地址随 arena 存活期不迁移）；调用方单线程独占该 arena，且按 `repr(C)` class-index 命中时
/// 返回的内部字段指针在 arena 存活期内有效。
pub(crate) unsafe fn get_type_function_type_id<T: TypeFunctionTypeVariantMember>(
  tv: TypeFunctionTypeId,
) -> *const T {
  LUAU_ASSERT!(!tv.is_null());
  if tv.is_null() {
    return null();
  }
  // Safety: 上方判空已确保 `tv` 非 null——它是 `TypeFunctionTypeId = *const ...`，
  // 指向 type function arena 中存活且对齐的类型变体节点，故 `&(*tv).type_variant` 重建
  // 共享引用有效。`T::get_if` 按 variant 标签匹配：命中 ⇒ 该标签确属 `T`，`repr(C)` 布局
  // 下 `&T` 与所指变体首字段基址重合，回 `*const T` 类型正确；未命中返回 null。全程只读。
  unsafe {
    match T::get_if(&(*tv).type_variant) {
      Some(r) => r as *const T,
      None => null(),
    }
  }
}
