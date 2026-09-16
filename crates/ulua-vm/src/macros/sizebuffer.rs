use core::mem::offset_of;

use crate::type_aliases::buffer::Buffer;

#[inline]
pub const fn sizebuffer(len: usize) -> usize {
  offset_of!(Buffer, data) + if len < 8 { 8 } else { len }
}
