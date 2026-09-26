use crate::records::{lua_page::lua_Page, lua_state::LuaState};

/// # Safety
/// `l` 须为存活 `LuaState` 且 `(*l).global` 有效（取其 `frealloc/ud`）；`page` 须为待释放的存活
/// `lua_Page`，读其 `listnext/listprev/page_size`；`pageset` 指向该页所属的头指针数组，允许为 NULL
/// （此时跳过链表摘除，仅回调 `frealloc` 释放）。cpp `lmem.cpp:337`。
pub(crate) unsafe fn freepage(l: *mut LuaState, pageset: *mut *mut lua_Page, page: *mut lua_Page) {
  unsafe {
    let g = (*l).global;

    if !pageset.is_null() {
      // remove page from alllist
      if !(*page).listnext.is_null() {
        (*(*page).listnext).listprev = (*page).listprev;
      }

      if !(*page).listprev.is_null() {
        (*(*page).listprev).listnext = (*page).listnext;
      } else if *pageset == page {
        *pageset = (*page).listnext;
      }
    }

    // so long
    if let Some(frealloc) = (*g).frealloc {
      frealloc((*g).ud, page as *mut u8, (*page).page_size as usize, 0);
    }
  }
}
