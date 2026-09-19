use core::{ffi::c_char, ptr::null};

use crate::{
  functions::{r#match::match_item, singlematch::singlematch},
  records::match_state::MatchState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn max_expand(
  ms: *mut MatchState,
  s: *const c_char,
  p: *const c_char,
  ep: *const c_char,
) -> *const c_char {
  unsafe {
    let mut i: isize = 0; // counts maximum expand for item

    // while (singlematch(ms, s + i, p, ep))
    //     i++;
    while singlematch(ms, s.offset(i), p, ep) != 0 {
      i += 1;
    }

    // keeps trying to match with the maximum repetitions
    while i >= 0 {
      let res = match_item(ms, s.offset(i), ep.offset(1));

      if !res.is_null() {
        return res;
      }

      i -= 1; // else didn't match; reduce 1 repetition to try again
    }

    null()
  }
}
