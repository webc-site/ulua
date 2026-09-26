use core::ffi::c_char;
#[derive(Debug)]
#[repr(C)]
pub struct lua_Page {
  pub(crate) prev: *mut lua_Page,
  pub(crate) next: *mut lua_Page,
  pub(crate) listprev: *mut lua_Page,
  pub(crate) listnext: *mut lua_Page,
  pub(crate) page_size: i32,
  pub(crate) block_size: i32,
  pub(crate) free_list: *mut u8,
  pub(crate) free_next: i32,
  pub(crate) busy_blocks: i32,
  #[cfg(target_pointer_width = "64")]
  pub(crate) padding: [c_char; 8],
  #[cfg(not(target_pointer_width = "64"))]
  pub(crate) padding: [c_char; 12],
  pub(crate) data: [c_char; 1],
}
