use core::{
  ffi::{c_char, c_int},
  mem::size_of,
};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::freeclasspage::freeclasspage,
  macros::asan_poison_memory_region::ASAN_POISON_MEMORY_REGION,
  records::{g_cheader::GCheader, lua_page::lua_Page, size_class_config::K_SIZE_CLASS_CONFIG},
  type_aliases::{global_state::global_State, lua_state::lua_State},
};

const K_GCO_LINK_OFFSET: usize =
  (size_of::<GCheader>() + size_of::<*mut u8>() - 1) & !(size_of::<*mut u8>() - 1);

pub(crate) unsafe fn freegcoblock(
  l: *mut lua_State,
  size_class: c_int,
  block: *mut u8,
  page: *mut lua_Page,
) {
  unsafe {
    LUAU_ASSERT!(!page.is_null() && (*page).busy_blocks > 0);
    LUAU_ASSERT!((*page).block_size == K_SIZE_CLASS_CONFIG.size_of_class[size_class as usize]);
    LUAU_ASSERT!(
      block >= core::ptr::addr_of!((*page).data) as *mut u8
        && (block as *mut c_char) < (page as *mut c_char).add((*page).page_size as usize)
    );

    let g: *mut global_State = (*l).global;
    let freegcopages = core::ptr::addr_of_mut!((*g).freegcopages) as *mut *mut lua_Page;

    // if the page wasn't in the page free list, it should be now since it got a block!
    if (*page).free_list.is_null() && (*page).free_next < 0 {
      LUAU_ASSERT!((*page).prev.is_null());
      LUAU_ASSERT!((*page).next.is_null());

      (*page).next = *freegcopages.add(size_class as usize);
      if !(*page).next.is_null() {
        (*(*page).next).prev = page;
      }
      *freegcopages.add(size_class as usize) = page;
    }

    // when separate block metadata is not used, free list link is stored inside the block data itself
    *((block as *mut c_char).add(K_GCO_LINK_OFFSET) as *mut *mut u8) = (*page).free_list;
    (*page).free_list = block;

    ASAN_POISON_MEMORY_REGION!(
      (block as *mut c_char).add(core::mem::size_of::<GCheader>()) as *mut u8,
      ((*page).block_size as usize).wrapping_sub(core::mem::size_of::<GCheader>())
    );

    (*page).busy_blocks -= 1;

    // if it's the last block in the page, we don't need the page
    if (*page).busy_blocks == 0 {
      freeclasspage(
        l,
        freegcopages,
        core::ptr::addr_of_mut!((*g).allgcopages),
        page,
        size_class as u8,
      );
    }
  }
}
