use core::{
  ffi::{c_char, c_void},
  ptr::null_mut,
};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::lua_m_getpagewalkinfo::lua_m_getpagewalkinfo,
  records::{gc_object::GCObject, lua_page::lua_Page},
  type_aliases::gc_visitor::GcVisitor,
};

/// # Safety
/// 调用方须保证：`page` 为存活页且页头自洽（按 [`lua_m_getpagewalkinfo`] 契约）；`visitor` 能以
/// (context, page, 块内存活对象) 安全调用，且返回 true 删除块后不得再引用被删对象；当最后一个存活块
/// 被删时页本身可能已被释放，调用方不得再触碰 `page`。cpp lmem.cpp:854 `luaM_visitpage`
pub(crate) unsafe fn lua_m_visitpage(
  page: *mut lua_Page,
  context: *mut c_void,
  visitor: GcVisitor,
) {
  // Safety: 契约保证 `page` 与页头元数据自洽，回调仅接收块内各存活对象指针
  unsafe {
    let mut start: *mut c_char = null_mut();
    let mut end: *mut c_char = null_mut();
    let mut busy_blocks: i32 = 0;
    let mut block_size: i32 = 0;

    lua_m_getpagewalkinfo(
      page,
      &mut start,
      &mut end,
      &mut busy_blocks,
      &mut block_size,
    );

    let mut pos = start;
    while pos != end {
      let gco = pos.cast::<GCObject>();

      // skip memory blocks that are already freed
      if !(*gco).header().is_nil() {
        // when true is returned it means that the element was deleted
        if visitor(context, page, gco) {
          LUAU_ASSERT!(busy_blocks > 0);

          // if the last block was removed, page would be removed as well
          busy_blocks -= 1;
          if busy_blocks == 0 {
            break;
          }
        }
      }

      pos = pos.add(block_size as usize);
    }
  }
}
