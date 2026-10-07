use core::mem::offset_of;

use crate::records::luau_buffer::LuauBuffer as Buffer;

#[inline]
pub const fn sizebuffer(len: usize) -> usize {
  offset_of!(Buffer, data) + if len < 8 { 8 } else { len }
}
