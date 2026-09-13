use core::ffi::c_uint;

use crate::{
  functions::arrayindex::arrayindex,
  macros::{ceillog_2::ceillog2, maxbits::MAXSIZE},
};

pub(crate) fn countint(key: f64, nums: &mut [i32]) -> i32 {
  let k = arrayindex(key);
  if k > 0 && k <= MAXSIZE {
    nums[ceillog2(k as c_uint) as usize] += 1;
    1
  } else {
    0
  }
}
