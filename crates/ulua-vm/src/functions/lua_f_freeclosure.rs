use crate::{
  functions::lua_m_freegco::lua_m_freegco,
  macros::{size_cclosure::size_cclosure, size_lclosure::size_lclosure},
  records::{closure::Closure, gc_object::GcObject, lua_page::lua_Page, lua_state::LuaState},
};

/// # Safety
/// `l` 须存活（归还内存经其 `global` 分配器）；`c` 必须是尚未释放、`is_c`/`nupvalues` 仍与当初分配
/// 尺寸一致的闭包，`page` 必须是 `c` 实际所属的页。算错 size 或页不符会按错误 size class 归还，
/// 写脏页空闲链。cpp lfunc.cpp:197。
pub(crate) unsafe fn lua_f_freeclosure(l: *mut LuaState, c: *mut Closure, page: *mut lua_Page) {
  // Safety: 契约保证 `c`/`page` 与页分配器元数据一致（闭包确属该页），块内解除 upval 引用后按类归还内存
  unsafe {
    let size = if (*c).is_c != 0 {
      size_cclosure((*c).nupvalues as i32)
    } else {
      size_lclosure((*c).nupvalues as usize)
    };

    lua_m_freegco(l, c as *mut GcObject, size, (*c).hdr.memcat, page);
  }
}
