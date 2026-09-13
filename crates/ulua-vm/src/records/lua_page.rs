use core::ffi::{c_char, c_int};
#[derive(Debug)]
#[repr(C)]
pub struct lua_Page {
  pub(crate) prev: *mut lua_Page,
  pub(crate) next: *mut lua_Page,
  pub(crate) listprev: *mut lua_Page,
  pub(crate) listnext: *mut lua_Page,
  pub(crate) page_size: c_int,
  pub(crate) block_size: c_int,
  pub(crate) free_list: *mut u8,
  pub(crate) free_next: c_int,
  pub(crate) busy_blocks: c_int,
  #[cfg(target_pointer_width = "64")]
  pub(crate) padding: [c_char; 8],
  #[cfg(not(target_pointer_width = "64"))]
  pub(crate) padding: [core::ffi::c_char; 12],
  pub(crate) data: [c_char; 1],
}
