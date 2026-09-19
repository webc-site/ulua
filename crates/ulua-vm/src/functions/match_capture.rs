use core::{
  ffi::{c_char, c_int},
  ptr::null,
  slice::from_raw_parts,
};

use crate::{functions::check_capture::check_capture, records::match_state::MatchState};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn match_capture(
  ms: *mut MatchState,
  s: *const c_char,
  l: c_int,
) -> *const c_char {
  unsafe {
    let l = check_capture(ms, l);
    let len = (*ms).capture[l as usize].len as usize;

    if ((*ms).src_end as usize).wrapping_sub(s as usize) >= len
      && from_raw_parts((*ms).capture[l as usize].init as *const u8, len)
        == from_raw_parts(s as *const u8, len)
    {
      s.add(len)
    } else {
      null()
    }
  }
}
