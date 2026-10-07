use core::ffi::c_char;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_tolstring::{c_str_out_param, lua_tolstring_ref},
    tag_error::tag_error,
  },
  records::lua_state::LuaState,
};

/// Rust 内部核心（§2/§3 出参收口；r16-p28 锚定形）：cpp `luaL_checklstring`（`laux.cpp:176`）的
/// 切片形态——栈槽 `narg` 处可转成字符串的值返回其全部字节（含内嵌 `\0`，长度即
/// 切片长度），否则按 cpp 抛 "string expected"。
///
/// C 形 `size_t* len` 出参收口为返回值长度，由垫片 [`lua_l_checklstring`] 独家承接
/// 出参写入；Rust 调用方一律直接用本函数（cpp `MatchState` 的 s/p 串游标即对应
/// 这里的借用切片）。
///
/// 调用序契约（正确性，非内存安全）——r16-p28 起返回切片锚定 `l` 的 `&mut` 借用
/// （生命周期省略，未受约束 `'a` 已消灭）：持窗期间 `l` 被借用钉住，不得再经 `l` 读参/
/// 压栈/触发分配或 GC（编译器拒绝）；窗口稳定前提（Lua 串不可变且不被移动，实参被栈槽
/// 持有故不被回收）由该借用承载，存续上界即借用结束点。纯安全代码自此无法把窗口实例化
/// 为 `'static` 取走。`l` 仍须为存活 `LuaState` 且处于可抛错受保护帧（非串实参经
/// `tag_error` 抛错发散），`narg` 为其合法栈索引。
pub fn lua_l_checklstring_ref(l: &mut LuaState, narg: i32) -> &[u8] {
  // SAFETY: 转发 `lua_tolstring_ref`；其裸窗借出在本门面收窄为不长于 `l` 借用（只此
  // 一向收窄；Lua 串不可变不移动、栈槽钉住，见函数文档契约）。`None` 即 cpp 的 `NULL`
  // 失败路径，按 `luaL_checklstring` 语义抛 "string expected"（tag_error 发散，不返回）
  unsafe {
    lua_tolstring_ref(l, narg).unwrap_or_else(|| tag_error(l, narg, LuaType::String as i32))
  }
}

/// C-ABI 适配垫片：把 [`lua_l_checklstring_ref`] 的切片折算回 lua.h 约定的
/// `(const char*, size_t* len)` 形态，仅供 laux C API 边界使用。
///
/// # Safety
/// `len` 裸指针出参为 C 约定面（按 review.md §2 保留 unsafe 形）；`l` 同
/// [`lua_l_checklstring_ref`]（存活由 `&mut` 接收者类型承载）；`len` 的
/// 可写 `usize` 槽或 null 契约由调用方按文档保证。
pub unsafe fn lua_l_checklstring(l: &mut LuaState, narg: i32, len: *mut usize) -> *const c_char {
  // SAFETY: ref 核心永不失败返回（失败即 tag_error 抛错），出参折算收口到
  // `c_str_out_param`，`len` 允许为 null 由其契约给出
  unsafe { c_str_out_param(Some(lua_l_checklstring_ref(l, narg)), len) }
}
