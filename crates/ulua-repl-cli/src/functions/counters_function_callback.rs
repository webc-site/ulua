use alloc::string::String;
use core::ffi::{CStr, c_char, c_void};

use crate::records::{function_counters::FunctionCounters, module_counters::ModuleCounters};

// Faithful port of Counters.cpp's `countersFunctionCallback`.
pub unsafe fn counters_function_callback(
  context: *mut c_void,
  function: *const c_char,
  line_defined: i32,
) {
  unsafe {
    let counters = &mut *(context as *mut ModuleCounters);

    let name: String = if function.is_null() && line_defined == 1 {
      "<main>".into()
    } else if !function.is_null() {
      let func = CStr::from_ptr(function).to_string_lossy();
      alloc::format!("{}:{}", func, line_defined)
    } else {
      alloc::format!("<anonymous>:{}", line_defined)
    };

    counters.functions.push(FunctionCounters {
      name,
      ..Default::default()
    });
  }
}
