use core::{
  mem::size_of,
  ptr::{addr_of_mut, null_mut},
};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::newclasspage::newclasspage,
  macros::asan_unpoison_memory_region::ASAN_UNPOISON_MEMORY_REGION,
  records::{
    g_cheader::GCheader, global_state::global_State, lua_page::lua_Page, lua_state::LuaState,
    size_class_config::K_SIZE_CLASS_CONFIG,
  },
};

const K_GCO_LINK_OFFSET: usize =
  (size_of::<GCheader>() + size_of::<*mut u8>() - 1) & !(size_of::<*mut u8>() - 1);

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn newgcoblock(l: *mut LuaState, size_class: i32) -> *mut u8 {
  unsafe {
    let g: *mut global_State = (*l).global;
    // freegcopages 即 [*mut lua_Page; LUA_SIZECLASSES]（cpp `freepages[sizeClass]` 同款）：
    // 本类别槽位经裸 place 下标直读直写，仅向 newclasspage 传数组基址（其契约为基址 + size_class 索引）
    let mut page: *mut lua_Page = (*g).freegcopages[size_class as usize];

    // slow path: no page in the freelist, allocate a new one
    if page.is_null() {
      page = newclasspage(
        l,
        addr_of_mut!((*g).freegcopages).cast::<*mut lua_Page>(),
        addr_of_mut!((*g).allgcopages),
        size_class as u8,
        false,
      );
    }

    LUAU_ASSERT!((*page).prev.is_null());
    LUAU_ASSERT!(!(*page).free_list.is_null() || (*page).free_next >= 0);
    LUAU_ASSERT!((*page).block_size == K_SIZE_CLASS_CONFIG.size_of_class[size_class as usize]);

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
      ASAN_UNPOISON_MEMORY_REGION!(
        block.add(size_of::<GCheader>()),
        ((*page).block_size as usize).wrapping_sub(size_of::<GCheader>())
      );

      // when separate block metadata is not used, free list link is stored inside the block data itself
      (*page).free_list = block.add(K_GCO_LINK_OFFSET).cast::<*mut u8>().read();
      (*page).busy_blocks += 1;
    }

    // if we allocate the last block out of a page, we need to remove it from free list
    if (*page).free_list.is_null() && (*page).free_next < 0 {
      (*g).freegcopages[size_class as usize] = (*page).next;
      if !(*page).next.is_null() {
        (*(*page).next).prev = null_mut();
      }
      (*page).next = null_mut();
    }

    block
  }
}
