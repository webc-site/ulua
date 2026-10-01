//! 直接字段回调的向量写面（cpp `lua_userdatadirectfield_setvector`，lapi.cpp:2140/2155）。
//!
//! 裸指针保留：宿主运行期注册回调，类型集合非静态可穷举（review.md §4 保留条款）——
//! `result` 是 `LuaUserdataDirectFieldGet` 回调按 C ABI 直取协议交还宿主的 resultarg
//! 槽，其类型在编译期不可枚举，故入口维持 `*mut c_void`；按 §2「unsafe 只留边界」，
//! 两枚函数的 `unsafe` 各收敛为入口处一处指针→句柄收口，体内裸写折叠为 [`Slot`] 句柄
//! + [`TValue::set_vvalue`] 方法面。
//!
//! 3/4 lane 两枚均为 cpp 的条件导出（`#if LUA_VECTOR_SIZE == 4` 选一，
//! `cpp/VM/include/lua.h:457-459`），Rust 侧按同一编译期常量折叠：lane 数由
//! `TValue::set_vvalue` 内的 `VVALUE_LANES` 决定，两枚都保留供宿主按构建配置链接，
//! 不属死兄弟。

use core::ffi::c_void;

use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{records::slot::Slot, type_aliases::t_value::TValue};

/// # Safety
///
/// `result` 须指向一块可写入 `TValue` 的有效内存，且仅在
/// `LuauDirectFieldGet` 开启的 C ABI 直取路径中调用。
pub unsafe fn lua_userdatadirectfield_setvector_void_f32_f32_f32_f32(
  result: *mut c_void,
  x: f32,
  y: f32,
  z: f32,
  w: f32,
) {
  LUAU_ASSERT!(fflag::LuauDirectFieldGet.get());

  // SAFETY: 契约保证 `result` 指向本次调用期内独占可写、按 `TValue` 对齐的槽；本处是
  // 全函数唯一的指针边界，随后 `set_vvalue` 在该独占视图上按 `VVALUE_LANES` 写 lane、
  // 末置 tag，与旧 `setvvalue!` 宏体逐位一致。`w` 以 `FnOnce` 惰性传入：3-lane 构建
  // （`LUA_VECTOR_SIZE != 4`）下第 4 分量无槽位、连求值都不做（`macros/setvvalue.rs`
  // 文件头的越界防御 rationale 原样适用）。
  unsafe {
    Slot::from_raw(result.cast::<TValue>())
      .as_mut()
      .set_vvalue(x, y, z, || w)
  };
}

/// # Safety
///
/// `result` 须指向一块可写入 `TValue` 的有效内存，且仅在
/// `LuauDirectFieldGet` 开启的 C ABI 直取路径中调用。
pub unsafe fn lua_userdatadirectfield_setvector_void_f32_f32_f32(
  result: *mut c_void,
  x: f32,
  y: f32,
  z: f32,
) {
  LUAU_ASSERT!(fflag::LuauDirectFieldGet.get());

  // SAFETY: 契约保证 `result` 指向本次调用期内独占可写、按 `TValue` 对齐的槽；本处是
  // 全函数唯一的指针边界。第 4 分量实参与旧宏体同为常量 `0.0`，且仍按 `set_vvalue`
  // 契约只在 4-lane 编译期门内求值（3-lane 构建下 lane3 不写、常量亦不落盘）。
  unsafe {
    Slot::from_raw(result.cast::<TValue>())
      .as_mut()
      .set_vvalue(x, y, z, || 0.0f32)
  };
}
