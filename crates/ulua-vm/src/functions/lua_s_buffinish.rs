use core::{
  ffi::{c_char, c_int},
  slice::from_raw_parts,
};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{lua_s_hash::luaS_hash, lua_s_resize::luaS_resize},
  macros::{atom_undef::ATOM_UNDEF, isdead::isdead, lmod::lmod, whitebits::WHITEBITS},
  records::{gc_object::GCObject, stringtable::Stringtable, t_string::tstring},
  type_aliases::lua_state::lua_State,
};

#[inline]
unsafe fn same_bytes(a: *const c_char, b: *const c_char, len: usize) -> bool {
  unsafe { from_raw_parts(a as *const u8, len) == from_raw_parts(b as *const u8, len) }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_s_buffinish(l: *mut lua_State, ts: *mut tstring) -> *mut tstring {
  unsafe {
    let h = luaS_hash((*ts).data.as_ptr(), (*ts).len as usize);
    let tb: *mut Stringtable = core::ptr::addr_of_mut!((*(*l).global).strt);
    let bucket = lmod!(h, (*tb).size) as i32;

    let mut el = *(*tb).hash.add(bucket as usize);
    while !el.is_null() {
      if (*el).len == (*ts).len
        && same_bytes((*el).data.as_ptr(), (*ts).data.as_ptr(), (*ts).len as usize)
      {
        if isdead!((*l).global, el as *mut GCObject) {
          (*el).hdr.marked ^= WHITEBITS as u8;
        }
        return el;
      }
      el = (*el).next;
    }

    LUAU_ASSERT!((*ts).next.is_null());

    (*ts).hash = h;
    *(*ts).data.as_mut_ptr().add((*ts).len as usize) = 0;
    (*ts).atom = ATOM_UNDEF as i16;
    (*ts).next = *(*tb).hash.add(bucket as usize);
    *(*tb).hash.add(bucket as usize) = ts;

    (*tb).nuse = (*tb).nuse.wrapping_add(1);
    if (*tb).nuse > (*tb).size as u32 && (*tb).size <= c_int::MAX / 2 {
      luaS_resize(l, (*tb).size * 2);
    }

    ts
  }
}

pub use lua_s_buffinish as luaS_buffinish;
