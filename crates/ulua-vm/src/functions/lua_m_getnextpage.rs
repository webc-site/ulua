use crate::records::lua_page::lua_Page;

/// cpp `luaM_getnextpage`（lmem.cpp:849）：取页链表的下一页指针，仅拷贝字段、不解引用。
///
/// 形参收敛为 `&lua_Page` 后本函数无任何 unsafe 操作，降为 safe；调用方若持裸指针，
/// 重建引用的合法性由其自身 unsafe 边界承担。
pub fn lua_m_getnextpage(page: &lua_Page) -> *mut lua_Page {
  page.listnext
}
