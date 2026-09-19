use core::ffi::CStr;

use ulua_vm::{macros::getstr::getstr, records::proto::Proto};

use crate::{functions::try_find_local_name::try_find_local_name, traits::LogAppend};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn log_function_header<B: LogAppend>(build: &mut B, proto: *mut Proto) {
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
      // 首个参数无分隔符（对齐 cpp `i == 0 ? "" : ", "`）
      let sep = if i == 0 { "" } else { ", " };
      match try_find_local_name(proto, i, 0) {
        Some(name_str) => build.log_append(format_args!("{}{}", sep, name_str)),
        None => build.log_append(format_args!("{}$arg{}", sep, i)),
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
