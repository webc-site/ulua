//! Source: `VM/src/lapi.cpp:99-118` (hand-ported)

use crate::{
  functions::pseudo_2_addr::pseudo_2_addr,
  macros::{
    api_check::api_check, lua_o_nilobject::LUA_O_NILOBJECT, lua_registryindex::LUA_REGISTRYINDEX,
  },
  records::lua_state::LuaState,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// `l` 须为存活 LuaState 且当前帧 `(*(*l).ci).top`、`(*l).base`、`(*l).top` 指向同一栈数组并保持
/// `base <= top` 不变式；`idx` 为合法 Lua 栈索引（正数不超帧顶、负数在 `LUA_REGISTRYINDEX` 之上、伪索引走
/// `pseudo_2_addr`），越界正索引按语义返回 `LUA_O_NILOBJECT`。返回的 StkId 仅在栈未重分配前有效。
/// cpp/VM/src/lapi.cpp:115 index2addr。
pub unsafe fn index_2_addr(l: *mut LuaState, idx: i32) -> StkId {
  unsafe {
    if idx > 0 {
      api_check!(l, idx as isize <= (*(*l).ci).top.offset_from((*l).base));
      // 先比较再偏移：C++ 里 `base + (idx - 1)` 只是个悬垂指针，随后与 top 比
      // 较返回 nilobject（lua_type(L, 1000) 是合法调用）；Rust 里对越界 off 做
      // `add` 本身就是 UB，所以用偏移量比较替代指针比较。
      let off = (idx - 1) as usize;
      if off >= (*l).top.offset_from((*l).base) as usize {
        LUA_O_NILOBJECT as *mut TValue
      } else {
        (*l).base.add(off)
      }
    } else if idx > LUA_REGISTRYINDEX {
      api_check!(
        l,
        idx != 0 && (-idx) as isize <= (*l).top.offset_from((*l).base)
      );
      (*l).top.offset(idx as isize)
    } else {
      pseudo_2_addr(l, idx)
    }
  }
}
