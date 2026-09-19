use crate::records::{lua_page::lua_Page, lua_state::lua_State};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn freepage(l: *mut lua_State, pageset: *mut *mut lua_Page, page: *mut lua_Page) {
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
