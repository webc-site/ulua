use core::ffi::c_int;

use ulua_common::macros::{luau_assert::LUAU_ASSERT, luau_noinline::LUAU_NOINLINE};

use crate::{
  functions::newpage::newpage,
  records::{lua_page::lua_Page, size_class_config::K_SIZE_CLASS_CONFIG},
  type_aliases::lua_state::lua_State,
};

pub(crate) const K_LARGE_PAGE_THRESHOLD: i32 = 1024;
pub(crate) const K_LARGE_PAGE_SIZE: i32 = 65536;
pub(crate) const K_SMALL_PAGE_SIZE: i32 = 16384;
pub(crate) const K_BLOCK_HEADER: i32 = 8;

LUAU_NOINLINE! {
    pub(crate) unsafe fn newclasspage(
        l: *mut lua_State,
        freepageset: *mut *mut lua_Page,
        pageset: *mut *mut lua_Page,
        size_class: u8,
        store_metadata: bool,
    ) -> *mut lua_Page { unsafe {
        let size_of_class = K_SIZE_CLASS_CONFIG.size_of_class[size_class as usize];
        let page_size = if size_of_class > K_LARGE_PAGE_THRESHOLD {
            K_LARGE_PAGE_SIZE
        } else {
            K_SMALL_PAGE_SIZE
        };
        let block_size = size_of_class + if store_metadata { K_BLOCK_HEADER } else { 0 };
        let block_count = (page_size - core::mem::offset_of!(lua_Page, data) as c_int) / block_size;

        let page = newpage(l, pageset, page_size, block_size, block_count);

        LUAU_ASSERT!((*freepageset.add(size_class as usize)).is_null());
        *freepageset.add(size_class as usize) = page;

        page
    }}
}
