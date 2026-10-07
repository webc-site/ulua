//! 直接字段回调的 64 位整数写面（cpp `lua_userdatadirectfield_setinteger64`，
//! lapi.cpp:2178）。
//!
//! 裸指针保留：宿主运行期注册回调，类型集合非静态可穷举（review.md §4 保留条款）——
//! `result` 是 `LuaUserdataDirectFieldGet` 回调按 C ABI 直取协议交还宿主的 resultarg
//! 槽，其类型在编译期不可枚举，故入口维持 `*mut c_void`；按 §2「unsafe 只留边界」，
//! 本函数的 `unsafe` 收敛为入口处一次指针→句柄收口，体内裸写折叠为 [`Slot`] 句柄 +
//! [`TValue::set_lvalue`] 安全方法面。
//!
//! 死兄弟核查（B1c 票面 4）：本文件曾有 `_64` 后缀的同名转发体
//! `lua_userdatadirectfield_setinteger_64`（及其 `capi_shell_udfield_set!` 透传壳，
//! 导出符号 `ulua_lua_userdatadirectfield_setinteger_64`），系「文件名 → 符号名」机器
//! 派生的重影：cpp oracle 只声明一枚 `lua_userdatadirectfield_setinteger64`
//! （`cpp/VM/include/lua.h:462`、`cpp/VM/src/lapi.cpp:2178`），无后缀与 `_64` 变体均
//! 不存在；全仓 `rg` 三形态核查（宏实参 / 直接路径 / `examples/`）除该壳自身外零消费，
//! 按 §7 零死代码连同导出壳一并删除，唯一保留的导出符号 `ulua_lua_userdatadirectfield_setinteger64`
//! 名称、参数布局与契约文案不变。

use core::ffi::c_void;

use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{records::slot::Slot, type_aliases::t_value::TValue};

/// # Safety
///
/// `result` 须指向一块可写入 `TValue` 的有效内存，且仅在
/// `LuauDirectFieldGet` 开启的 C ABI 直取路径中调用。
pub unsafe fn lua_userdatadirectfield_setinteger64(result: *mut c_void, n: i64) {
  LUAU_ASSERT!(fflag::LuauDirectFieldGet.get());

  // SAFETY: 契约保证 `result` 指向本次调用期内独占可写、按 `TValue` 对齐的槽；本处是
  // 全函数唯一的指针边界，其后仅经句柄写面调用安全方法 `set_lvalue`（先写载荷后写
  // tag，与旧 `setlvalue!` 宏体逐位一致）。
  unsafe { Slot::from_raw(result.cast::<TValue>()) }
    .as_mut()
    .set_lvalue(n);
}
