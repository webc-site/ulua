use core::{
  ffi::c_char,
  ptr::{addr_of_mut, null_mut},
};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{freeobj::freeobj, lua_m_getpagewalkinfo::lua_m_getpagewalkinfo},
  macros::{
    fixedbit::FIXEDBIT, isdead::isdead, lua_c_white::luaC_white, maskmarks::maskmarks,
    otherwhite::otherwhite, testbit::testbit, whitebits::WHITEBITS,
  },
  records::{gc_object::GCObject, lua_page::lua_Page, lua_state::LuaState},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn sweepgcopage(l: *mut LuaState, page: *mut lua_Page) -> i32 {
  unsafe {
    let mut start: *mut c_char = null_mut();
    let mut end: *mut c_char = null_mut();
    let mut busy_blocks: i32 = 0;
    let mut block_size: i32 = 0;
    lua_m_getpagewalkinfo(
      page,
      addr_of_mut!(start),
      addr_of_mut!(end),
      addr_of_mut!(busy_blocks),
      addr_of_mut!(block_size),
    );

    LUAU_ASSERT!(busy_blocks > 0);

    let g = (*l).global;
    let deadmask = otherwhite!(g);
    LUAU_ASSERT!(testbit!(deadmask, FIXEDBIT) != 0);

    let newwhite = luaC_white!(g);
    let mut pos = start;

    while pos != end {
      let gco = pos as *mut GCObject;

      if !(*gco).header().is_nil() {
        if (((*gco).marked() as i32 ^ WHITEBITS) & deadmask) != 0 {
          LUAU_ASSERT!(!isdead!(g, gco));
          (*gco).header_mut().marked = (((*gco).marked() & maskmarks!()) | newwhite) as u8;
        } else {
          LUAU_ASSERT!(isdead!(g, gco));
          freeobj(l, gco, page);

          busy_blocks -= 1;
          if busy_blocks == 0 {
            return (pos.offset_from(start) as i32) / block_size + 1;
          }
        }
      }

      pos = pos.add(block_size as usize);
    }

    (end.offset_from(start) as i32) / block_size
  }
}
