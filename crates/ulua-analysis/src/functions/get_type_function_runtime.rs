use core::ptr::null;

use ulua_common::macros::luau_assert::LUAU_ASSERT;
use ulua_vm::{
  functions::{lua_getthreaddata::lua_getthreaddata, lua_mainthread::lua_mainthread},
  records::lua_state,
};

use crate::{
  records::type_function_runtime::TypeFunctionRuntime,
  type_aliases::{
    lua_state::LuaState, type_function_type_id::TypeFunctionTypeId,
    type_function_type_pack_id::TypeFunctionTypePackId,
    type_function_type_pack_variant::TypeFunctionTypePackVariantMember,
    type_function_type_variant::TypeFunctionTypeVariantMember,
  },
};

/// # Safety
/// `l` 必须是注册阶段经 `setTypeFunctionEnvironment` 在其主线程 userdatum 上挂载了
/// `TypeFunctionRuntime` 的 `lua_State*`（可为协程，内部取 `lua_mainthread`）；返回的裸指针
/// 指向该 userdatum 内部对象，仅在 VM 存活期有效，调用方不得跨调用保存。对应 C++
/// `TypeFunctionRuntime* getTypeFunctionRuntime(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:357`）。
pub unsafe fn get_type_function_runtime(l: *mut LuaState) -> *mut TypeFunctionRuntime {
  // Safety: `l` 由 Lua 虚拟机按 C 函数调用约定传入，为有效存活的 `*mut LuaState`；
  // `l as *mut lua_state::LuaState` 只是同一地址的类型重解释。`lua_mainthread` 返回该
  // 状态所属主线程的有效指针，`lua_getthreaddata` 返回我们在注册时写入主线程的 user
  // data（即 `TypeFunctionRuntime` 地址）。`&*(l as *mut …)` 仅形成只读借用供
  // lua_mainthread 读其 `global`/`mainthread` 字段，全程未解引用返回值、未写任何
  // 状态，仅做指针取回与重解释，
  // 与注册路径 `runtime as *mut ()` 互逆，故返回值即当初存入的合法句柄。
  unsafe {
    let main_thread = lua_mainthread(&*(l as *mut lua_state::LuaState));
    let data = lua_getthreaddata(&*main_thread);
    data as *mut TypeFunctionRuntime
  }
}

// Source: `Analysis/include/Luau/TypeFunctionRuntime.h:167-173` (hand-ported)
/// C++ `template<typename T> const T* get(TypeFunctionTypePackId tv)`.
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn get_type_function_type_pack_id<T: TypeFunctionTypePackVariantMember>(
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
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn get_type_function_type_id<T: TypeFunctionTypeVariantMember>(
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
