use core::ffi::c_int;

use crate::records::{
  closure::Closure, gc_object::GCObject, lua_state::lua_State, lua_t_value::TValue,
  lua_table::LuaTable, luau_buffer::LuauBuffer, luau_class::LuauClass, luau_object::LuauObject,
  proto::Proto, t_key::TKey, t_string::tstring, udata::Udata, up_val::UpVal,
};
#[macro_export]
macro_rules! ttype {
  ($o:expr) => {
    $crate::macros::ttype::TTypeOf::ttype($o)
  };
}

pub use ttype;

pub trait TTypeOf {
  /// # Safety
  ///
  /// If `self` is a raw pointer, it must be valid and properly aligned.
  unsafe fn ttype(self) -> c_int;
}

impl TTypeOf for *const TValue {
  unsafe fn ttype(self) -> c_int {
    unsafe { (*self).tt() }
  }
}

impl TTypeOf for *mut TValue {
  unsafe fn ttype(self) -> c_int {
    unsafe { (*self).tt() }
  }
}

impl TTypeOf for &TValue {
  unsafe fn ttype(self) -> c_int {
    self.tt()
  }
}

impl TTypeOf for &mut TValue {
  unsafe fn ttype(self) -> c_int {
    self.tt()
  }
}

impl TTypeOf for *const TKey {
  unsafe fn ttype(self) -> c_int {
    unsafe { (*self).tt() }
  }
}

impl TTypeOf for *mut TKey {
  unsafe fn ttype(self) -> c_int {
    unsafe { (*self).tt() }
  }
}

impl TTypeOf for &TKey {
  unsafe fn ttype(self) -> c_int {
    self.tt()
  }
}

impl TTypeOf for &mut TKey {
  unsafe fn ttype(self) -> c_int {
    self.tt()
  }
}

macro_rules! impl_gc_ttype_direct {
  ($t:ty) => {
    impl TTypeOf for *const $t {
      unsafe fn ttype(self) -> core::ffi::c_int {
        unsafe { (*self).tt as core::ffi::c_int }
      }
    }

    impl TTypeOf for *mut $t {
      unsafe fn ttype(self) -> core::ffi::c_int {
        unsafe { (*self).tt as core::ffi::c_int }
      }
    }
  };
}

macro_rules! impl_gc_ttype_hdr {
  ($t:ty) => {
    impl TTypeOf for *const $t {
      unsafe fn ttype(self) -> core::ffi::c_int {
        unsafe { (*self).hdr.tt as core::ffi::c_int }
      }
    }

    impl TTypeOf for *mut $t {
      unsafe fn ttype(self) -> core::ffi::c_int {
        unsafe { (*self).hdr.tt as core::ffi::c_int }
      }
    }
  };
}

impl TTypeOf for *const GCObject {
  unsafe fn ttype(self) -> c_int {
    unsafe { (*self).gch.tt as c_int }
  }
}

impl TTypeOf for *mut GCObject {
  unsafe fn ttype(self) -> c_int {
    unsafe { (*self).gch.tt as c_int }
  }
}

impl_gc_ttype_hdr!(tstring);
impl_gc_ttype_hdr!(Closure);
impl_gc_ttype_hdr!(lua_State);
impl_gc_ttype_hdr!(Proto);
impl_gc_ttype_hdr!(UpVal);

impl_gc_ttype_direct!(LuaTable);
impl_gc_ttype_direct!(Udata);
impl_gc_ttype_direct!(LuauBuffer);
impl_gc_ttype_direct!(LuauClass);
impl_gc_ttype_direct!(LuauObject);
