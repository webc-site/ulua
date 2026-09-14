use core::ffi::{c_char, c_int};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::lua_page::lua_Page;

pub(crate) unsafe fn lua_m_getpagewalkinfo(
  page: *mut lua_Page,
  start: *mut *mut c_char,
  end: *mut *mut c_char,
  busy_blocks: *mut c_int,
  block_size: *mut c_int,
) {
  unsafe {
    let page_ref = &*page;
    let block_count =
      (page_ref.page_size - core::mem::offset_of!(lua_Page, data) as c_int) / page_ref.block_size;

    LUAU_ASSERT!(
      page_ref.free_next >= -page_ref.block_size
        && page_ref.free_next <= (block_count - 1) * page_ref.block_size
    );

    let data_ptr = page_ref.data.as_ptr() as *mut c_char;

    *start = data_ptr.add((page_ref.free_next + page_ref.block_size) as usize);
    *end = data_ptr.add((block_count * page_ref.block_size) as usize);
    *busy_blocks = page_ref.busy_blocks;
    *block_size = page_ref.block_size;
  }
}
