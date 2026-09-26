use core::ptr::{addr_of_mut, null_mut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::newclasspage::{K_BLOCK_HEADER, newclasspage},
  macros::{
    asan_unpoison_memory_region::ASAN_UNPOISON_MEMORY_REGION, debugpageset::debugpageset,
    metadata::metadata,
  },
  records::{
    global_state::global_State, lua_page::lua_Page, lua_state::LuaState,
    size_class_config::K_SIZE_CLASS_CONFIG,
  },
};

/// # Safety
/// `l` 须为存活 LuaState 且 `(*l).global` 有效，`(*g).freepages[..]`/`allpages` 为分配器自洽的双向页链表；
/// `size_class` 须为 `K_SIZE_CLASS_CONFIG` 已知的合法尺寸类（`>= 0` 且 `< 类数`，由调用方 `sizeclass!` 判定），
/// freelink 为空时 `newclasspage` 可能分配并触发 `frealloc` 回调。返回块首地址仅在页未被回收前有效。
/// cpp/VM/src/lmem.cpp:372 newblock。
pub(crate) unsafe fn newblock(l: *mut LuaState, size_class: i32) -> *mut u8 {
  unsafe {
    let g: *mut global_State = (*l).global;
    let mut page: *mut lua_Page = (*g).freepages[size_class as usize];

    // slow path: no page in the freelist, allocate a new one
    if page.is_null() {
      page = newclasspage(
        l,
        addr_of_mut!((*g).freepages).cast(),
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
      block = addr_of_mut!((*page).data)
        .cast::<u8>()
        .add((*page).free_next as usize);
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
    block.add(K_BLOCK_HEADER as usize)
  }
}
