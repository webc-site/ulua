use core::{mem::size_of, slice::from_raw_parts};

use crate::{
  functions::lua_m_freegco::lua_m_freegco,
  macros::{
    lua_utag_limit::LUA_UTAG_LIMIT, obj_2_gco::obj2gco, sizeudata::sizeudata,
    utag_idtor::UTAG_IDTOR,
  },
  records::{lua_page::lua_Page, lua_state::LuaState, udata::Udata},
  type_aliases::lua_destructor::LuaDestructor,
};

/// # Safety
/// `l` 须为存活 LuaState 且 `(*l).global.udatagc[..LUA_UTAG_LIMIT]` 可寻址；`u` 须指向正在回收的存活 Udata，
/// 其 `tag`/`len`/`memcat`/`data` 自洽：内建 tag 走 `udatagc[tag]` 析构、`UTAG_IDTOR` 时从 `data` 尾部
/// `size_of::<LuaDestructor>()` 字节读回析构指针（该区间须落在 `data` 内）；`page` 为所属 lua_Page，可空由分配器语义决定。析构可 unwind。
/// cpp/VM/src/ludata.cpp:23 luaU_freeudata。
pub(crate) unsafe fn lua_u_freeudata(l: *mut LuaState, u: *mut Udata, page: *mut lua_Page) {
  unsafe {
    let tag = (*u).tag as i32;

    if tag < LUA_UTAG_LIMIT {
      let dtor = (*(*l).global).udatagc[tag as usize];
      if let Some(dtor_fn) = dtor {
        dtor_fn(l, (*u).data.as_mut_ptr().cast());
      }
    } else if tag == UTAG_IDTOR {
      let len = (*u).len as usize;
      let dtor_size = size_of::<LuaDestructor>();
      // 析构指针按字节存于 data 尾部（newudata 侧同宽 `write_unaligned` 写入）；
      // 切片视图以 `len` 为界替代手写 `.add(len - dtor_size)` 裸指针算术，unaligned
      // 读回免 MaybeUninit+copy 双步，与原逐字节拷贝同值
      let payload = from_raw_parts((*u).data.as_ptr().cast::<u8>(), len);
      let dtor_addr = payload[len - dtor_size..].as_ptr().cast::<LuaDestructor>();
      let dtor = dtor_addr.read_unaligned();

      if let Some(dtor_fn) = dtor {
        dtor_fn(l, (*u).data.as_mut_ptr().cast());
      }
    }

    lua_m_freegco(l, obj2gco!(u), sizeudata((*u).len as usize), (*u).memcat, page);
  }
}
