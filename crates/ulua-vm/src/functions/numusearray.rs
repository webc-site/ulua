use core::ffi::c_int;

use crate::{
  macros::{maxbits::MAXBITS, ttisnil::ttisnil},
  records::lua_table::LuaTable,
};

pub(crate) unsafe fn numusearray(t: *const LuaTable, nums: *mut c_int) -> c_int {
  unsafe {
    let mut lg: i32;
    let mut ttlg: i32; // 2^lg
    let mut ause: i32 = 0; // summation of `nums'
    let mut i: i32 = 1; // count to traverse all array keys

    lg = 0;
    ttlg = 1;
    while lg <= MAXBITS {
      let mut lc: i32 = 0; // counter
      let mut lim: i32 = ttlg;

      if lim > (*t).sizearray {
        lim = (*t).sizearray; // adjust upper limit
        if i > lim {
          break; // no more elements to count
        }
      }

      // count elements in range (2^(lg-1), 2^lg]
      while i <= lim {
        if !ttisnil!((*t).array.offset((i - 1) as isize)) {
          lc += 1;
        }
        i += 1;
      }

      *nums.offset(lg as isize) += lc;
      ause += lc;

      lg += 1;
      ttlg *= 2;
    }

    ause
  }
}
