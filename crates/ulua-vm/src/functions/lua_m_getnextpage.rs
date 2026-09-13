use crate::records::lua_page::lua_Page;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaM_getnextpage")]
pub unsafe fn lua_m_getnextpage(page: *mut lua_Page) -> *mut lua_Page {
  unsafe { (*page).listnext }
}

pub use lua_m_getnextpage as luaM_getnextpage;
