use core::ffi::c_char;

use crate::{
  functions::lua_l_error_l::lua_l_error_l, macros::l_esc::L_ESC, records::match_state::MatchState,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn classend(ms: *mut MatchState, p: *const c_char) -> *const c_char {
  unsafe {
    let p = p.add(1);
    match *p.offset(-1) as u8 {
      x if x == L_ESC as u8 => {
        if p == (*ms).p_end {
          lua_l_error_l(
            (*ms).l,
            c"malformed pattern (ends with '%%')".as_ptr(),
            core::format_args!("malformed pattern (ends with '%%')"),
          );
        }
        p.add(1)
      }
      b'[' => {
        let mut p = p;
        if *p == b'^' as c_char {
          p = p.add(1);
        }
        loop {
          if p == (*ms).p_end {
            lua_l_error_l(
              (*ms).l,
              c"malformed pattern (missing ']')".as_ptr(),
              core::format_args!("malformed pattern (missing ']')"),
            );
          }
          p = p.add(1);
          if *p.offset(-1) == L_ESC && p < (*ms).p_end {
            p = p.add(1);
          }
          if *p == b']' as c_char {
            break;
          }
        }
        p.add(1)
      }
      _ => p,
    }
  }
}
