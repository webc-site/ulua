//! Source: `VM/src/lapi.cpp:99-118` (hand-ported)

use crate::{
  functions::pseudo_2_addr::pseudo_2_addr,
  macros::{
    api_check::api_check, lua_o_nilobject::luaO_nilobject, lua_registryindex::LUA_REGISTRYINDEX,
  },
  type_aliases::{lua_state::lua_State, stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn index2addr(l: *mut lua_State, idx: i32) -> StkId {
  unsafe {
    if idx > 0 {
      api_check!(l, idx as isize <= (*(*l).ci).top.offset_from((*l).base));
      // 先比较再偏移：C++ 里 `base + (idx - 1)` 只是个悬垂指针，随后与 top 比
      // 较返回 nilobject（lua_type(L, 1000) 是合法调用）；Rust 里对越界 off 做
      // `add` 本身就是 UB，所以用偏移量比较替代指针比较。
      let off = (idx - 1) as usize;
      if off >= (*l).top.offset_from((*l).base) as usize {
        luaO_nilobject as *mut TValue
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

pub use index2addr as index_2_addr;
