//! Source: `VM/src/lmem.cpp:322-331` (hand-ported)

use crate::{
  functions::freepage::freepage,
  records::{lua_page::lua_Page, lua_state::LuaState},
};

/// # Safety
/// `l` 须存活（`freepage` 经其 `global` 归还内存）；`page` 必须是确属 `size_class` 对应 size 类、
/// 仍在 `freepageset`/`pageset` 双向链表协议内的存活页，两个链表头指针槽可写。
/// 违反（size_class 与页实际类别不符等）会把页挂错/摘错链表，破坏页分配器元数据。cpp lmem.cpp:358。
pub(crate) unsafe fn freeclasspage(
  l: *mut LuaState,
  freepageset: *mut *mut lua_Page,
  pageset: *mut *mut lua_Page,
  page: *mut lua_Page,
  size_class: u8,
) {
  // Safety: 契约保证 `page` 确属对应 size_class 的 freelist 语义（含空页上界检查），摘链与归还仅改写页头字段
  unsafe {
    // remove page from freelist
    if !(*page).next.is_null() {
      (*(*page).next).prev = (*page).prev;
    }

    if !(*page).prev.is_null() {
      (*(*page).prev).next = (*page).next;
    } else {
      let idx = size_class as usize;
      if *freepageset.add(idx) == page {
        *freepageset.add(idx) = (*page).next;
      }
    }

    freepage(l, pageset, page);
  }
}
