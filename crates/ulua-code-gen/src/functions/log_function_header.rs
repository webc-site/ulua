use core::ffi::CStr;

use ulua_vm::{macros::getstr::getstr, records::proto::Proto};

use crate::{functions::try_find_local_name::try_find_local_name, traits::LogAppend};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn log_function_header(build: &mut dyn LogAppend, proto: *mut Proto) {
  unsafe {
    let debugname = (*proto).debugname;
    if !debugname.is_null() {
      let name = getstr(debugname as *const _);
      let name_str = CStr::from_ptr(name).to_string_lossy();
      build.log_append(format_args!("; function {}(", name_str));
    } else {
      build.log_append(format_args!("; function("));
    }

    for i in 0..(*proto).numparams as i32 {
      if let Some(name_str) = try_find_local_name(proto, i, 0) {
        if i == 0 {
          build.log_append(format_args!("{}", name_str));
        } else {
          build.log_append(format_args!(", {}", name_str));
        }
      } else {
        if i == 0 {
          build.log_append(format_args!("$arg{}", i));
        } else {
          build.log_append(format_args!(", $arg{}", i));
        }
      }
    }

    if (*proto).numparams != 0 && (*proto).is_vararg != 0 {
      build.log_append(format_args!(", ...)"));
    } else {
      build.log_append(format_args!(")"));
    }

    if (*proto).linedefined >= 0 {
      build.log_append(format_args!(" line {}\n", (*proto).linedefined));
    } else {
      build.log_append(format_args!("\n"));
    }
  }
}
