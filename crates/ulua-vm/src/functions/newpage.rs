use core::{ffi::c_int, mem::offset_of, ptr::null_mut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_status::LuaStatus,
  functions::lua_d_throw_ldo::lua_d_throw,
  macros::asan_poison_memory_region::ASAN_POISON_MEMORY_REGION,
  records::lua_page::lua_Page,
  type_aliases::{global_state::global_State, lua_state::lua_State},
};

pub(crate) unsafe fn newpage(
  l: *mut lua_State,
  pageset: *mut *mut lua_Page,
  page_size: c_int,
  block_size: c_int,
  block_count: c_int,
) -> *mut lua_Page {
  unsafe {
    let g: *mut global_State = (*l).global;

    LUAU_ASSERT!(page_size - (offset_of!(lua_Page, data) as c_int) >= block_size * block_count);

    let frealloc_fn = (*g).frealloc;
    let page = if let Some(f) = frealloc_fn {
      f((*g).ud, null_mut(), 0, page_size as usize) as *mut lua_Page
    } else {
      null_mut()
    };

    if page.is_null() {
      lua_d_throw(l, LuaStatus::ErrMem as i32);
    }

    ASAN_POISON_MEMORY_REGION!((*page).data.as_ptr(), (block_size * block_count) as usize);

    // setup page header
    (*page).prev = null_mut();
    (*page).next = null_mut();

    (*page).listprev = null_mut();
    (*page).listnext = null_mut();

    (*page).page_size = page_size;
    (*page).block_size = block_size;

    // note: we start with the last block in the page and move downward
    // either order would work, but that way we don't need to store the block count in the page
    // additionally, GC stores objects in singly linked lists, and this way the GC lists end up in increasing pointer order
    (*page).free_list = null_mut();
    (*page).free_next = (block_count - 1) * block_size;
    (*page).busy_blocks = 0;

    if !pageset.is_null() {
      (*page).listnext = *pageset;
      if !(*page).listnext.is_null() {
        (*(*page).listnext).listprev = page;
      }
      *pageset = page;
    }

    page
  }
}
