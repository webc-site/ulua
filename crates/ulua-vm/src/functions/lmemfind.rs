#[cfg(target_arch = "wasm32")]
use core::slice::from_raw_parts;
use core::{
  ffi::{c_char, c_int, c_void},
  ptr::null,
};
pub(crate) unsafe fn lmemfind(
  mut s1: *const c_char,
  mut l1: usize,
  s2: *const c_char,
  mut l2: usize,
) -> *const c_char {
  unsafe {
    if l2 == 0 {
      s1 // empty strings are everywhere
    } else if l2 > l1 {
      null() // avoids a negative `l1'
    } else {
      let mut init: *const c_char; // to search for a `*s2' inside `s1'
      l2 -= 1; // 1st char will be checked by `memchr'
      l1 -= l2; // `s2' cannot be found after that
      while l1 > 0 {
        init = libc_memchr(s1 as *const c_void, *s2 as c_int, l1) as *const c_char;
        if init.is_null() {
          break;
        }
        init = init.add(1); // 1st char is already checked
        if libc_memcmp(init as *const c_void, s2.add(1) as *const c_void, l2) == 0 {
          return init.sub(1);
        } else {
          // correct `l1' and `s1' to try again
          l1 -= init.offset_from(s1) as usize;
          s1 = init;
        }
      }
      null() // not found
    }
  }
}

unsafe fn libc_memchr(s: *const c_void, c: c_int, n: usize) -> *mut c_void {
  unsafe {
    #[cfg(target_arch = "wasm32")]
    {
      let slice = from_raw_parts(s as *const u8, n);
      match slice.iter().position(|&b| b == c as u8) {
        Some(i) => s.add(i) as *mut core::ffi::c_void,
        None => core::ptr::null_mut(),
      }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
      unsafe extern "C" {
        fn memchr(s: *const c_void, c: c_int, n: usize) -> *mut c_void;
      }
      memchr(s, c, n)
    }
  }
}

unsafe fn libc_memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int {
  unsafe {
    #[cfg(target_arch = "wasm32")]
    {
      let slice1 = from_raw_parts(s1 as *const u8, n);
      let slice2 = from_raw_parts(s2 as *const u8, n);
      // zip 单次遍历，首个不等字节决定序
      for (a, b) in slice1.iter().zip(slice2) {
        if a != b {
          return if a < b { -1 } else { 1 };
        }
      }
      0
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
      unsafe extern "C" {
        fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
      }
      memcmp(s1, s2, n)
    }
  }
}
