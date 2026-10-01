use core::mem::offset_of;

use crate::records::t_string::tstring;

#[inline]
pub const fn sizestring(len: usize) -> usize {
  offset_of!(tstring, data) + len + 1
}
