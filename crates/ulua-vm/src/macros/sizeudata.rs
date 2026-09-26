use core::mem::offset_of;

use crate::records::udata::Udata;

#[inline]
pub const fn sizeudata(len: usize) -> usize {
  offset_of!(Udata, data) + if len > 16 { (len + 15) & !15 } else { len }
}
