use core::ffi::CStr;

use ulua_vm::{macros::getstr::getstr, records::proto::Proto};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn try_find_local_name<'a>(
  proto: *const Proto,
  reg: i32,
  pcpos: i32,
) -> Option<&'a str> {
  unsafe {
    for i in 0..(*proto).sizelocvars {
      let local = &*(*proto).locvars.add(i as usize);

      if reg == local.reg as i32 && pcpos >= local.startpc && pcpos < local.endpc {
        if local.varname.is_null() {
          return None;
        } else {
          let s = getstr(local.varname);
          return if s.is_null() {
            None
          } else {
            Some(CStr::from_ptr(s).to_str().unwrap_or(""))
          };
        }
      }
    }

    None
  }
}
