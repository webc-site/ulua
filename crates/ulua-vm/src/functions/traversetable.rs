//! Source: `VM/src/lgc.cpp` (lgc.cpp:321-366, hand-ported)

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_type::LuaType,
  functions::{cstr_bytes, gettablemode::gettablemode, removeentry::removeentry},
  macros::{
    gkey::{gkey, gval},
    gnode::gnode,
    markobject::markobject,
    markvalue::markvalue,
    sizenode::sizenode,
    ttype::ttype,
  },
  records::{gc_object::GCObject, global_state::global_State, lua_table::LuaTable},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn traversetable(g: *mut global_State, h: *mut LuaTable) -> i32 {
  unsafe {
    if !(*h).metatable.is_null() {
      markobject!(g, (*h).metatable);
    }

    // is there a weak mode?
    // C++ `strchr(modev, 'k'/'v') != NULL`：contains 等价（mode 串仅 1~2 字节）
    let modev = gettablemode(g, h);
    let (weakkey, weakvalue) = if modev.is_null() {
      (0, 0)
    } else {
      let mode = cstr_bytes(modev);
      let weakkey = mode.contains(&b'k') as i32;
      let weakvalue = mode.contains(&b'v') as i32;
      if weakkey != 0 || weakvalue != 0 {
        // is really weak?
        (*h).gclist = (*g).weak; // must be cleared after GC, ...
        (*g).weak = h as *mut GCObject; // ... so put in the appropriate list
      }
      (weakkey, weakvalue)
    };

    if weakkey != 0 && weakvalue != 0 {
      return 1;
    }
    if weakvalue == 0 {
      let mut i = (*h).sizearray;
      while i > 0 {
        i -= 1;
        markvalue!(g, (*h).array.add(i as usize));
      }
    }
    let mut i: i32 = sizenode!(h);
    while i > 0 {
      i -= 1;
      let n = gnode!(h, i);
      LUAU_ASSERT!(ttype!(gkey!(n)) != LuaType::DeadKey as u32 || (*gval!(n)).is_nil());
      if (*gval!(n)).is_nil() {
        removeentry(n); // remove empty entries
      } else {
        LUAU_ASSERT!(!(*gkey!(n)).is_nil());
        if weakkey == 0 {
          markvalue!(g, gkey!(n));
        }
        if weakvalue == 0 {
          markvalue!(g, gval!(n));
        }
      }
    }
    (weakkey != 0 || weakvalue != 0) as i32
  }
}
