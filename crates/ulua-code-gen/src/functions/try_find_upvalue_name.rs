use core::ffi::CStr;

use ulua_vm::{macros::getstr::getstr, records::proto::Proto};

macro_rules! CODEGEN_ASSERT {
  ($expr:expr) => {
    assert!($expr);
  };
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn try_find_upvalue_name<'a>(proto: *const Proto, upval: i32) -> Option<&'a str> {
  unsafe {
    if !(*proto).upvalues.is_null() {
      CODEGEN_ASSERT!(upval < (*proto).sizeupvalues);

      let upvalue = *(*proto).upvalues.add(upval as usize);
      if !upvalue.is_null() {
        let s = getstr(upvalue as *const _);
        return if s.is_null() {
          None
        } else {
          Some(CStr::from_ptr(s).to_str().unwrap_or(""))
        };
      }
    }

    None
  }
}
