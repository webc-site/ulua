use core::{
  ffi::{CStr, c_char, c_int, c_void},
  slice::from_raw_parts,
};
use std::ffi::CString;

unsafe extern "C" {
  fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
}

pub unsafe fn coverage_callback(
  context: *mut c_void,
  function: *const c_char,
  linedefined: c_int,
  depth: c_int,
  hits: *const c_int,
  size: usize,
) {
  unsafe {
    if context.is_null() {
      return;
    }

    let name = if depth == 0 {
      "<main>".to_string()
    } else if !function.is_null() {
      let func_str = CStr::from_ptr(function).to_string_lossy();
      format!("{}:{}", func_str, linedefined)
    } else {
      format!("<anonymous>:{}", linedefined)
    };

    let name_c = CString::new(name.clone()).unwrap();
    fprintf(
      context,
      c"FN:%d,%s\n".as_ptr(),
      linedefined,
      name_c.as_ptr(),
    );

    let hits_slice = from_raw_parts(hits, size);

    for &hit in hits_slice {
      if hit != -1 {
        let name_c = CString::new(name.clone()).unwrap();
        fprintf(context, c"FNDA:%d,%s\n".as_ptr(), hit, name_c.as_ptr());
        break;
      }
    }

    for (i, &hit) in hits_slice.iter().enumerate() {
      if hit != -1 {
        fprintf(context, c"DA:%d,%d\n".as_ptr(), i as c_int, hit);
      }
    }
  }
}
