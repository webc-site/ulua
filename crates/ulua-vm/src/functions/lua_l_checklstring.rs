use core::ffi::c_char;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_tolstring::{c_str_out_param, lua_tolstring_ref},
    tag_error::tag_error,
  },
  records::lua_state::LuaState,
};

/// Rust 内部核心（§2/§3 出参收口）：cpp `luaL_checklstring`（`laux.cpp:176`）的
/// 切片形态——栈槽 `narg` 处可转成字符串的值返回其全部字节（含内嵌 `\0`，长度即
/// 切片长度），否则按 cpp 抛 "string expected"。
///
/// C 形 `size_t* len` 出参收口为返回值长度，由垫片 [`lua_l_checklstring`] 独家承接
/// 出参写入；Rust 调用方一律直接用本函数（cpp `MatchState` 的 s/p 串游标即对应
/// 这里的借用切片）。
///
/// # Safety
/// `l` 须为存活 `LuaState` 且处于可抛错受保护帧（非串实参经 `tag_error` 抛错发散），
/// `narg` 为其合法栈索引。返回切片指向栈槽串内部字节，在本次 C 函数调用期间有效
/// （Lua 串不可变且不被移动，实参被栈槽持有故不被回收）——即 cpp 侧
/// 「`luaL_checklstring` 结果在本次调用内可读」的同一契约。
pub unsafe fn lua_l_checklstring_ref<'a>(l: *mut LuaState, narg: i32) -> &'a [u8] {
  // Safety: 转发同契约的 `lua_tolstring_ref`；`None` 即 cpp 的 `NULL` 失败路径，
  // 按 `luaL_checklstring` 语义抛 "string expected"（tag_error 发散，不返回）
  unsafe {
    lua_tolstring_ref(l, narg).unwrap_or_else(|| tag_error(l, narg, LuaType::String as i32))
  }
}

/// C-ABI 适配垫片：把 [`lua_l_checklstring_ref`] 的切片折算回 lua.h 约定的
/// `(const char*, size_t* len)` 形态，仅供 laux C API 边界使用。
///
/// # Safety
/// `l` 同 [`lua_l_checklstring_ref`]；`len` 必须为可写 `usize` 槽或 null。
pub unsafe fn lua_l_checklstring(l: *mut LuaState, narg: i32, len: *mut usize) -> *const c_char {
  // Safety: ref 核心永不失败返回（失败即 tag_error 抛错），出参折算收口到
  // `c_str_out_param`，`len` 允许为 null 由其契约给出
  unsafe { c_str_out_param(Some(lua_l_checklstring_ref(l, narg)), len) }
}
