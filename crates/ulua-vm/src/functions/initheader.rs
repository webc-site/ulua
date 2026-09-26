use crate::records::{header::Header, lua_state::LuaState};

#[inline]
/// # Safety
///
/// `h` 必须指向可写的 8 字节头区；块头各版本常量按固定偏移写入且不越出头区。
pub(crate) unsafe fn initheader(l: *mut LuaState, h: *mut Header) {
  // Safety: 契约保证 `h` 指向可写的 8 字节头区且各版本常量以固定偏移写入，不越出该头区
  unsafe {
    (*h).l = l;
    (*h).islittle = 1; // nativeendian.little is assumed to be 1 (true) for little-endian systems
    (*h).maxalign = 1;
  }
}
