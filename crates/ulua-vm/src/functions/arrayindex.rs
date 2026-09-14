use core::ffi::c_int;

use crate::macros::{cast_num::cast_num, luai_numeq::luai_numeq};

pub fn arrayindex(key: f64) -> i32 {
  let i = key as c_int;

  if luai_numeq(cast_num!(i), key) { i } else { -1 }
}
