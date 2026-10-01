use core::mem::offset_of;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::lua_page::lua_Page;

/// 页遍历窗口元数据：cpp `luaM_getpagewalkinfo` 四个 C 风格出参的 Rust 化返回
/// （review §3 出参 → 返回值）。`start`/`end` 为块区首/尾后字节游标。
pub(crate) struct PageWalkInfo {
  pub start: *mut u8,
  pub end: *mut u8,
  pub busy_blocks: i32,
  pub block_size: i32,
}

/// # Safety
/// 调用方须保证：`page` 为页分配器分配的存活 `lua_Page` 且页头自洽（block_size>0、
/// free_next ∈ [-block_size, (blockCount-1)*block_size]，即未经 freeblock 破坏）。
/// 返回的 `start..end` 字节游标即该页的存活块窗口。cpp lmem.cpp:827
pub(crate) unsafe fn lua_m_getpagewalkinfo(page: *mut lua_Page) -> PageWalkInfo {
  // SAFETY: 契约保证 `page` 为存活页，遍历终止时 walkedBytes/objects 等输出反映界内实际对象
  unsafe {
    let page_ref = &*page;
    let block_count =
      (page_ref.page_size - offset_of!(lua_Page, data) as i32) / page_ref.block_size;

    LUAU_ASSERT!(
      page_ref.free_next >= -page_ref.block_size
        && page_ref.free_next <= (block_count - 1) * page_ref.block_size
    );

    let data_ptr = page_ref.data.as_ptr() as *mut u8;

    PageWalkInfo {
      start: data_ptr.add((page_ref.free_next + page_ref.block_size) as usize),
      end: data_ptr.add((block_count * page_ref.block_size) as usize),
      busy_blocks: page_ref.busy_blocks,
      block_size: page_ref.block_size,
    }
  }
}
