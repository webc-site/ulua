use core::{ffi::c_char, slice::from_raw_parts};

use crate::{
  functions::{lua_s_hash::luaS_hash, newlstr::newlstr},
  macros::{getstr::getstr, isdead::isdead, lmod::lmod, whitebits::WHITEBITS},
  records::{gc_object::GCObject, t_string::tstring},
  type_aliases::lua_state::lua_State,
};

#[inline]
unsafe fn same_bytes(a: *const c_char, b: *const c_char, len: usize) -> bool {
  unsafe { from_raw_parts(a as *const u8, len) == from_raw_parts(b as *const u8, len) }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_s_newlstr(l: *mut lua_State, str_: *const c_char, len: usize) -> *mut tstring {
  unsafe {
    let h = luaS_hash(str_, len);
    let bucket = lmod!(h, (*(*l).global).strt.size);
    let mut el = *(*(*l).global).strt.hash.add(bucket as usize);

    while !el.is_null() {
      if (*el).len as usize == len && same_bytes(str_, getstr(el), len) {
        if isdead!((*l).global, el as *mut GCObject) {
          (*el).hdr.marked ^= WHITEBITS as u8;
        }
        return el;
      }
      el = (*el).next;
    }

    newlstr(l, str_, len, h)
  }
}

pub use lua_s_newlstr as luaS_newlstr;
