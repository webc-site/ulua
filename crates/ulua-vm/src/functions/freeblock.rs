use core::ffi::{c_char, c_int};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{freeclasspage::freeclasspage, newclasspage::K_BLOCK_HEADER},
  macros::{
    asan_poison_memory_region::ASAN_POISON_MEMORY_REGION, debugpageset::debugpageset,
    metadata::metadata,
  },
  records::{lua_page::lua_Page, size_class_config::K_SIZE_CLASS_CONFIG},
  type_aliases::{global_state::global_State, lua_state::lua_State},
};

pub(crate) unsafe fn freeblock(l: *mut lua_State, size_class: c_int, block: *mut u8) {
  unsafe {
    let g: *mut global_State = (*l).global;

    LUAU_ASSERT!(!block.is_null());
    // 回退块头（存储所属页指针），K_BLOCK_HEADER 字节已预留，指针运算不会越出页
    let block = (block as *mut c_char).sub(K_BLOCK_HEADER as usize) as *mut u8;

    let page: *mut lua_Page = metadata!(block) as *mut lua_Page;
    LUAU_ASSERT!(!page.is_null() && (*page).busy_blocks > 0);
    LUAU_ASSERT!(
      ((*page).block_size as usize)
        == (K_SIZE_CLASS_CONFIG.size_of_class[size_class as usize] as usize
          + K_BLOCK_HEADER as usize)
    );
    LUAU_ASSERT!(
      block >= (*page).data.as_ptr() as *mut u8
        && (block as *mut c_char) < (page as *mut c_char).add((*page).page_size as usize)
    );

    if (*page).free_list.is_null() && (*page).free_next < 0 {
      LUAU_ASSERT!((*page).prev.is_null());
      LUAU_ASSERT!((*page).next.is_null());

      (*page).next = (*g).freepages[size_class as usize];
      if !(*page).next.is_null() {
        (*(*page).next).prev = page;
      }
      (*g).freepages[size_class as usize] = page;
    }

    metadata!(block) = (*page).free_list;
    (*page).free_list = block;

    ASAN_POISON_MEMORY_REGION!(block, (*page).block_size as usize);

    (*page).busy_blocks -= 1;

    if (*page).busy_blocks == 0 {
      freeclasspage(
        l,
        (*g).freepages.as_mut_ptr(),
        debugpageset!(&mut (*g).allpages),
        page,
        size_class as u8,
      );
    }
  }
}
