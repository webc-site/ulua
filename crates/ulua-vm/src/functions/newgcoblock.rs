use core::{
  ffi::{c_char, c_int},
  mem::size_of,
  ptr::null_mut,
};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::newclasspage::newclasspage,
  macros::asan_unpoison_memory_region::ASAN_UNPOISON_MEMORY_REGION,
  records::{g_cheader::GCheader, lua_page::lua_Page, size_class_config::K_SIZE_CLASS_CONFIG},
  type_aliases::{global_state::global_State, lua_state::lua_State},
};

const K_GCO_LINK_OFFSET: usize =
  (size_of::<GCheader>() + size_of::<*mut u8>() - 1) & !(size_of::<*mut u8>() - 1);

pub(crate) unsafe fn newgcoblock(l: *mut lua_State, size_class: c_int) -> *mut u8 {
  unsafe {
    let g: *mut global_State = (*l).global;
    let freegcopages = core::ptr::addr_of_mut!((*g).freegcopages) as *mut *mut lua_Page;
    let mut page: *mut lua_Page = *freegcopages.add(size_class as usize);

    // slow path: no page in the freelist, allocate a new one
    if page.is_null() {
      page = newclasspage(
        l,
        freegcopages,
        core::ptr::addr_of_mut!((*g).allgcopages),
        size_class as u8,
        false,
      );
    }

    LUAU_ASSERT!((*page).prev.is_null());
    LUAU_ASSERT!(!(*page).free_list.is_null() || (*page).free_next >= 0);
    LUAU_ASSERT!((*page).block_size == K_SIZE_CLASS_CONFIG.size_of_class[size_class as usize]);

    let block: *mut u8;

    if (*page).free_next >= 0 {
      let data = core::ptr::addr_of_mut!((*page).data) as *mut c_char;
      block = data.add((*page).free_next as usize) as *mut u8;
      ASAN_UNPOISON_MEMORY_REGION!(block, (*page).block_size as usize);

      (*page).free_next -= (*page).block_size;
      (*page).busy_blocks += 1;
    } else {
      block = (*page).free_list;
      ASAN_UNPOISON_MEMORY_REGION!(
        (block as *mut c_char).add(core::mem::size_of::<GCheader>()) as *mut u8,
        ((*page).block_size as usize).wrapping_sub(core::mem::size_of::<GCheader>())
      );

      // when separate block metadata is not used, free list link is stored inside the block data itself
      (*page).free_list = *((block as *mut c_char).add(K_GCO_LINK_OFFSET) as *mut *mut u8);
      (*page).busy_blocks += 1;
    }

    // if we allocate the last block out of a page, we need to remove it from free list
    if (*page).free_list.is_null() && (*page).free_next < 0 {
      *freegcopages.add(size_class as usize) = (*page).next;
      if !(*page).next.is_null() {
        (*(*page).next).prev = null_mut();
      }
      (*page).next = null_mut();
    }

    block
  }
}
