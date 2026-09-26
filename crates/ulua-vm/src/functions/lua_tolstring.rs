use core::{ffi::c_char, ptr::null, slice::from_raw_parts};

use crate::{
  functions::{
    index_2_addr::index_2_addr, lapi_barrier::lua_c_threadbarrier_lapi,
    lua_v_tostring::lua_v_tostring,
  },
  macros::{getstr::getstr, lua_c_check_gc::lua_c_check_gc},
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// Rust 内部核心（§2/§3 出参收口）：cpp `lua_tolstring`（`VM/src/lapi.cpp:497`）——`idx`
/// 槽可转成字符串时返回其全部字节（含内嵌 `\0`，长度即切片长度），否则 `None`。
///
/// C 形 `size_t* len` 出参与「指针 + 长度」二元组收口为 `Option<&'a [u8]>`：`None` 即
/// cpp 失败路径（`*len = 0` + 返回 NULL），其出参写入由 C-ABI 垫片 [`lua_tolstring`]
/// 独家承接。非串数值仍经 `luaV_tostring` 就地改写栈槽（分配新串、`lua_c_check_gc` 挪栈
/// 后重取 `index_2_addr` 的次序与栈顶调整语义逐字保持）。
///
/// # Safety
///
/// `l` 必须是正在执行的 C 函数帧的存活 `LuaState`，`idx` 为其合法栈索引（越界读错槽）；
/// 返回切片指向栈槽串内部字节，下一次操作 `l` 前有效（寿命 `'a` 与 [`lua_touserdata`]
/// 等既有收口同形，由调用方保证不跨 VM 操作持有）。cpp lapi.cpp:497。
pub unsafe fn lua_tolstring_ref<'a>(l: *mut LuaState, idx: i32) -> Option<&'a [u8]> {
  unsafe {
    let mut o: StkId = index_2_addr(l, idx);

    if !(*o).is_string() {
      lua_c_threadbarrier_lapi(l);
      if lua_v_tostring(l, o) == 0 {
        return None;
      }
      lua_c_check_gc!(l);
      o = index_2_addr(l, idx);
    }

    let ts = (*o).as_string_ptr();
    // Safety: `tsvalue!` 在 `ttisstring!` 闸门下取栈槽 TString；`getstr` 契约（存活
    // tstring、可读 `len + 1` 字节）成立，切片取前 `len` 字节 payload（不含终止 NUL）。
    Some(from_raw_parts(getstr(ts) as *const u8, (*ts).len as usize))
  }
}

/// C-ABI 出参收口共享核心：把 `Option<&[u8]>` 切片折算回 lua.h 约定的
/// `(const char*, size_t* len)` 形态——`Some` 写串长并返回首字节，`None` 写 0 并
/// 返回 NULL（与 cpp 失败路径逐位一致）。[`lua_tolstring`] 与 [`lua_l_tolstring_ref`]
/// 的 C-ABI 垫片共用，消除双份硬编码折算。
///
/// # Safety
///
/// `len` 必须为可写 `usize` 槽或 null；返回值指向切片字节区，寿命同输入切片
/// （Lua 串缓冲恒有终止 NUL，C 串语义不变）。
pub(crate) unsafe fn c_str_out_param(s: Option<&[u8]>, len: *mut usize) -> *const c_char {
  // Safety: 契约保证 `len` 非空即指向可写 usize 槽
  unsafe {
    if !len.is_null() {
      *len = s.map_or(0, <[u8]>::len);
    }
    s.map_or(null(), <[u8]>::as_ptr).cast::<c_char>()
  }
}

/// C-ABI 适配垫片：把 [`lua_tolstring_ref`] 的切片折算回 lua.h 约定的
/// `(const char*, size_t* len)` 形态——`len` 非空时写出串长（失败路径写 0，与 cpp
/// 一致），`None` 折算回 NULL。仅供真实 C ABI 边界（`ulua-capi` 生成层、lua.h 兼容
/// 宏 [`lua_tostring!`] 与直传 `size_t*` 出参的 laux 函数）使用；Rust 调用方一律直接
/// 消费 [`lua_tolstring_ref`]。
///
/// # Safety
///
/// 同 [`lua_tolstring_ref`]；此外 `len` 必须为可写 `usize` 槽或 null。
pub unsafe fn lua_tolstring(l: *mut LuaState, idx: i32, len: *mut usize) -> *const c_char {
  // Safety: 转发同契约的 `lua_tolstring_ref`；出参折算收口到 [`c_str_out_param`]，
  // 返回值指向切片首字节（Lua 串缓冲恒有终止 NUL，C 串语义不变）。
  unsafe { c_str_out_param(lua_tolstring_ref(l, idx), len) }
}
