//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_m_getnextpage.rs）。
//! 导出壳与 ulua-vm 对应函数签名一致，除把 `page` 裸指针重建为 `&lua_Page` 后透传外，零业务逻辑。
use ulua_vm::{functions::lua_m_getnextpage, records::lua_page::lua_Page};

/// # Safety
/// C ABI 导出壳（符号 `ulua_luaM_getnextpage`），透传至 `lua_m_getnextpage(page)`。调用方须保证：
/// - `page`（`*mut lua_Page`）：非空、对齐，指向 allocator 当前持有、调用期间不被回收的 `lua_Page`。
/// - 返回值（`*mut lua_Page`）：页链表下一页，NULL 表示链尾；有效期随 allocator 页链，调用方只读遍历、不得跨页释放持有。
#[unsafe(export_name = "ulua_luaM_getnextpage")]
pub unsafe extern "C-unwind" fn lua_m_getnextpage(page: *mut lua_Page) -> *mut lua_Page {
  // Safety: 契约保证 page 非空对齐且调用期存活，重建共享引用仅读 listnext 字段
  lua_m_getnextpage::lua_m_getnextpage(unsafe { &*page })
}
