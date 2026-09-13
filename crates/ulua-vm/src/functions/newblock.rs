use core::{ffi::c_char, ptr::null_mut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::newclasspage::{K_BLOCK_HEADER, newclasspage},
  macros::{
    asan_unpoison_memory_region::ASAN_UNPOISON_MEMORY_REGION, debugpageset::debugpageset,
    metadata::metadata,
  },
  records::{lua_page::lua_Page, size_class_config::K_SIZE_CLASS_CONFIG},
  type_aliases::{global_state::global_State, lua_state::lua_State},
};

pub(crate) unsafe fn newblock(l: *mut lua_State, size_class: i32) -> *mut u8 {
  unsafe {
    let g: *mut global_State = (*l).global;
    let mut page: *mut lua_Page = (*g).freepages[size_class as usize];

    // slow path: no page in the freelist, allocate a new one
    if page.is_null() {
      page = newclasspage(
        l,
        (*g).freepages.as_mut_ptr(),
        debugpageset!(&mut (*g).allpages),
        size_class as u8,
        true,
      );
    }

    LUAU_ASSERT!((*page).prev.is_null());
    LUAU_ASSERT!(!(*page).free_list.is_null() || (*page).free_next >= 0);
    LUAU_ASSERT!(
      ((*page).block_size as usize)
        == (K_SIZE_CLASS_CONFIG.size_of_class[size_class as usize] as usize
          + K_BLOCK_HEADER as usize)
    );

    let block: *mut u8;

    if (*page).free_next >= 0 {
      block = ((*page).data.as_mut_ptr() as *mut c_char).add((*page).free_next as usize) as *mut u8;
      ASAN_UNPOISON_MEMORY_REGION!(block, (*page).block_size as usize);

      (*page).free_next -= (*page).block_size;
      (*page).busy_blocks += 1;
    } else {
      block = (*page).free_list;
      ASAN_UNPOISON_MEMORY_REGION!(block, (*page).block_size as usize);

      (*page).free_list = metadata!(block);
      (*page).busy_blocks += 1;
    }

    // the first word in a block point back to the page
    metadata!(block) = page as *mut u8;

    // if we allocate the last block out of a page, we need to remove it from free list
    if (*page).free_list.is_null() && (*page).free_next < 0 {
      (*g).freepages[size_class as usize] = (*page).next;
      if !(*page).next.is_null() {
        (*(*page).next).prev = null_mut();
      }
      (*page).next = null_mut();
    }

    // the user data is right after the metadata
    (block as *mut c_char).add(K_BLOCK_HEADER as usize) as *mut u8
  }
}
