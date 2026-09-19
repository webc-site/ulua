//! Source: `VM/src/lmem.cpp:322-331` (hand-ported)

use crate::{
  functions::freepage::freepage, records::lua_page::lua_Page, type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn freeclasspage(
  l: *mut lua_State,
  freepageset: *mut *mut lua_Page,
  pageset: *mut *mut lua_Page,
  page: *mut lua_Page,
  size_class: u8,
) {
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
