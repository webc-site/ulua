use core::{
  mem::size_of,
  ptr::{addr_of, addr_of_mut},
};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::freeclasspage::freeclasspage,
  macros::asan_poison_memory_region::ASAN_POISON_MEMORY_REGION,
  records::{
    g_cheader::GCheader, global_state::global_State, lua_page::lua_Page, lua_state::LuaState,
    size_class_config::K_SIZE_CLASS_CONFIG,
  },
};

const K_GCO_LINK_OFFSET: usize =
  (size_of::<GCheader>() + size_of::<*mut u8>() - 1) & !(size_of::<*mut u8>() - 1);

/// # Safety
/// `l` 须存活（取 `global.freegcopages`）；`block` 必须是从 `page` 划出、尚未归还的 GCObject 块，
/// `size_class` 须等于该页 `block_size` 对应的类别，且 `page.busy_blocks > 0`。
/// 重复归还或类别不符会写脏页空闲链与位图，后续分配返回重叠内存。cpp lmem.cpp:508。
pub(crate) unsafe fn freegcoblock(
  l: *mut LuaState,
  size_class: i32,
  block: *mut u8,
  page: *mut lua_Page,
) {
  // Safety: 契约保证 `block` 来自页分配器此前分配且 size_class 与对象实际大小一致，归还仅更新位图与链表
  unsafe {
    LUAU_ASSERT!(!page.is_null() && (*page).busy_blocks > 0);
    LUAU_ASSERT!((*page).block_size == K_SIZE_CLASS_CONFIG.size_of_class[size_class as usize]);
    LUAU_ASSERT!(
      block >= addr_of!((*page).data).cast_mut().cast::<u8>()
        && block < page.cast::<u8>().add((*page).page_size as usize)
    );

    let g: *mut global_State = (*l).global;

    // if the page wasn't in the page free list, it should be now since it got a block!
    if (*page).free_list.is_null() && (*page).free_next < 0 {
      LUAU_ASSERT!((*page).prev.is_null());
      LUAU_ASSERT!((*page).next.is_null());

      (*page).next = (*g).freegcopages[size_class as usize];
      if !(*page).next.is_null() {
        (*(*page).next).prev = page;
      }
      (*g).freegcopages[size_class as usize] = page;
    }

    // when separate block metadata is not used, free list link is stored inside the block data itself
    block
      .add(K_GCO_LINK_OFFSET)
      .cast::<*mut u8>()
      .write((*page).free_list);
    (*page).free_list = block;

    ASAN_POISON_MEMORY_REGION!(
      block.add(size_of::<GCheader>()),
      ((*page).block_size as usize).wrapping_sub(size_of::<GCheader>())
    );

    (*page).busy_blocks -= 1;

    // if it's the last block in the page, we don't need the page
    if (*page).busy_blocks == 0 {
      // freeclasspage 的契约是数组基址 + size_class 索引（cpp 同款），传基址指针
      freeclasspage(
        l,
        addr_of_mut!((*g).freegcopages).cast::<*mut lua_Page>(),
        addr_of_mut!((*g).allgcopages),
        page,
        size_class as u8,
      );
    }
  }
}
