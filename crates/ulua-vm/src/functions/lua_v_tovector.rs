use core::ptr::null;

use crate::{
  macros::{ttisvector::ttisvector, vvalue::vvalue},
  type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_v_tovector(obj: *const TValue) -> *const f32 {
  unsafe {
    if ttisvector!(obj) {
      let v = vvalue!(obj);
      v.as_ptr()
    } else {
      null()
    }
  }
}

pub use lua_v_tovector as luaV_tovector;
