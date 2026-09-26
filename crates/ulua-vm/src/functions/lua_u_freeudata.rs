use core::{
  ffi::c_void,
  mem::{MaybeUninit, size_of},
  ptr::copy_nonoverlapping,
};

use crate::{
  functions::lua_m_freegco::lua_m_freegco,
  macros::{lua_utag_limit::LUA_UTAG_LIMIT, sizeudata::sizeudata, utag_idtor::UTAG_IDTOR},
  records::{gc_object::GCObject, lua_page::lua_Page, lua_state::LuaState, udata::Udata},
};

type UserdataDtor = Option<unsafe extern "C-unwind" fn(*mut c_void)>;

/// # Safety
/// `l` 须为存活 LuaState 且 `(*l).global.udatagc[..LUA_UTAG_LIMIT]` 可寻址；`u` 须指向正在回收的存活 Udata，
/// 其 `tag`/`len`/`memcat`/`data` 自洽：内建 tag 走 `udatagc[tag]` 析构、`UTAG_IDTOR` 时从 `data + (len - size_of::<UserdataDtor>())`
/// 逐字节读回析构指针（该区间须落在 `data` 内）；`page` 为所属 lua_Page，可空由分配器语义决定。析构可 unwind。
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
      let dtor_size = size_of::<UserdataDtor>();
      let dtor_addr = (*u).data.as_ptr().add(len - dtor_size);
      let mut dtor = MaybeUninit::<UserdataDtor>::uninit();
      copy_nonoverlapping(
        dtor_addr.cast::<u8>(),
        dtor.as_mut_ptr().cast::<u8>(),
        dtor_size,
      );
      let dtor = dtor.assume_init();

      if let Some(dtor_fn) = dtor {
        dtor_fn((*u).data.as_mut_ptr().cast());
      }
    }

    lua_m_freegco(
      l,
      u as *mut GCObject,
      sizeudata((*u).len as usize),
      (*u).memcat,
      page,
    );
  }
}
