use core::{
  ffi::c_int,
  ptr::{addr_of_mut, null_mut},
  slice::from_raw_parts_mut,
};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  macros::{lmod::lmod, lua_m_freearray::luaM_freearray, lua_m_newarray::luaM_newarray},
  records::{stringtable::Stringtable, t_string::tstring},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_s_resize(l: *mut lua_State, newsize: c_int) {
  unsafe {
    let newhash = luaM_newarray!(l, newsize as usize, *mut tstring, 0);
    let tb: *mut Stringtable = addr_of_mut!((*(*l).global).strt);

    // 新桶批量清零，替代逐下标推进
    for slot in from_raw_parts_mut(newhash, newsize as usize) {
      *slot = null_mut();
    }

    // 旧桶按序 rehash；链表内仍按 cpp 以 next 指针驱动。
    // f_luaopen 首扩时 size==0 且 hash 尚未分配（null）：
    // from_raw_parts_mut 前置检查要求指针非 null，len 为 0 也不例外，需守卫。
    let oldsize = (*tb).size;
    if oldsize != 0 {
      // SAFETY：tb->hash 为 size 个 *mut tstring 的数组，全部可读。
      for old in from_raw_parts_mut((*tb).hash, oldsize as usize) {
        let mut p = *old;
        while !p.is_null() {
          let next = (*p).next;
          let h = (*p).hash;
          let h1 = lmod!(h, newsize) as c_int;
          LUAU_ASSERT!((h % newsize as u32) as c_int == lmod!(h, newsize));
          (*p).next = *newhash.add(h1 as usize);
          *newhash.add(h1 as usize) = p;
          p = next;
        }
      }
    }

    luaM_freearray!(l, (*tb).hash, (*tb).size as usize, *mut tstring, 0);
    (*tb).size = newsize;
    (*tb).hash = newhash;
  }
}

pub use lua_s_resize as luaS_resize;
