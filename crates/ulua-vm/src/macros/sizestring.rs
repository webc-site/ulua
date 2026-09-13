use crate::records::t_string::tstring;

#[inline]
pub const fn sizestring(len: usize) -> usize {
  core::mem::offset_of!(tstring, data) + len + 1
}
