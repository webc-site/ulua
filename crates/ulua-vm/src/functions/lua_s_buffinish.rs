use core::slice::from_raw_parts;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{lua_s_hash::lua_s_hash, lua_s_resize::lua_s_resize},
  macros::atom_undef::ATOM_UNDEF,
  records::{global_state::global_State, lua_state::LuaState, t_string::tstring},
};

/// # Safety
/// 调用方须保证：`ts` 为 lua_s_bufstart 刚分配、尚未发布的存活 TString——data 区至少
/// len+1 可写（写入终止 NUL）且 `next` 为 null；`l` 存活且处于受保护帧（挂桶后 nuse
/// 超限触发 lua_s_resize，分配失败经 `l` 抛错）。命中同内容旧串时改返旧串、`ts` 成孤儿
/// 缓冲。cpp lstring.cpp:113 `luaS_buffinish`
pub unsafe fn lua_s_buffinish(l: *mut LuaState, ts: *mut tstring) -> *mut tstring {
  unsafe {
    let s = &mut *ts;
    // Safety: s 为存活 TString，data 区 s.len 字节必在界内（空串零长度切片合法）
    let bytes = from_raw_parts(s.data.as_ptr() as *const u8, s.len as usize);
    let h = lua_s_hash(bytes);
    let g: *mut global_State = (*l).global;

    // 查桶链是否已有同内容驻留串（扫描/复活收拢在 Stringtable::interned）
    if let Some(el) = (*g).strt.interned(g, h, bytes) {
      return el;
    }

    LUAU_ASSERT!(s.next.is_null());

    s.hash = h;
    *s.data.as_mut_ptr().add(s.len as usize) = 0; // TString 布局保证 data[len] 可写（终止 NUL）
    s.atom = ATOM_UNDEF as i16;
    let tb = &mut (*g).strt;
    tb.link_front(h, ts);
    if tb.wants_growth() {
      let target = tb.doubled_size();
      lua_s_resize(l, target); // too crowded
    }

    ts
  }
}
