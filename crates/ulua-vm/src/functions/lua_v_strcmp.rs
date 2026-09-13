use core::{cmp::Ordering, ffi::c_int, slice::from_raw_parts};

use crate::{macros::getstr::getstr, type_aliases::t_string::tstring};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn lua_v_strcmp(ls: *const tstring, rs: *const tstring) -> c_int {
  if ls == rs {
    return 0;
  }

  unsafe {
    let l = getstr(ls);
    let r = getstr(rs);

    // always safe to read one character because even empty strings are nul terminated
    let bl = *l as u8;
    let br = *r as u8;

    if bl != br {
      return (bl as c_int) - (br as c_int);
    }

    let ll = (*ls).len as usize;
    let lr = (*rs).len as usize;
    let lmin = if ll < lr { ll } else { lr };

    // u8 切片字典序比较与 C memcmp 语义一致（unsigned char 逐字节比较），
    // 编译期可下沉为 memcmp/bcmp，无需 C ABI
    let ord = from_raw_parts(l, lmin).cmp(from_raw_parts(r, lmin));

    match ord {
      Ordering::Less => -1,
      Ordering::Greater => 1,
      Ordering::Equal => {
        if ll == lr {
          0
        } else if ll < lr {
          -1
        } else {
          1
        }
      }
    }
  }
}

pub use lua_v_strcmp as luaV_strcmp;
