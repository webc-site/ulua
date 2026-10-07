//! 直接字段回调的浮点数写面（cpp `lua_userdatadirectfield_setnumber`，lapi.cpp:2132）。
//!
//! 裸指针保留：宿主运行期注册回调，类型集合非静态可穷举（review.md §4 保留条款）——
//! `result` 是 `LuaUserdataDirectFieldGet` 回调按 C ABI 直取协议交还宿主的 resultarg
//! 槽，其类型在编译期不可枚举，故入口维持 `*mut c_void`；按 §2「unsafe 只留边界」，
//! 本函数的 `unsafe` 收敛为入口处一次指针→句柄收口，体内裸写折叠为 [`Slot`] 句柄 +
//! [`TValue::set_nvalue`] 安全方法面。

use core::ffi::c_void;

use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{records::slot::Slot, type_aliases::t_value::TValue};

/// # Safety
/// `result` 须指向一块可写入 `TValue` 的有效内存（direct-access 回调交还宿主的 resultarg
/// 槽，本次调用期被栈钉住），且仅在 `LuauDirectFieldGet` 开启的 C ABI 直取路径中调用。
pub unsafe fn lua_userdatadirectfield_setnumber(result: *mut c_void, n: f64) {
  LUAU_ASSERT!(fflag::LuauDirectFieldGet.get());

  // SAFETY: 契约保证 `result` 指向本次调用期内独占可写、按 `TValue` 对齐的槽；本处是
  // 全函数唯一的指针边界，其后仅经句柄写面调用安全方法 `set_nvalue`（只写 f64 值与
  // tag，与旧 `setnvalue!` 宏体逐位一致）。
  unsafe { Slot::from_raw(result.cast::<TValue>()) }
    .as_mut()
    .set_nvalue(n);
}
