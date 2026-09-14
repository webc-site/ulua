use crate::records::udata::Udata;

#[inline]
pub const fn sizeudata(len: usize) -> usize {
  core::mem::offset_of!(Udata, data) + if len > 16 { (len + 15) & !15 } else { len }
}
