use core::{
  ffi::c_void,
  mem::{MaybeUninit, size_of},
  ptr::copy_nonoverlapping,
};

use crate::{
  functions::lua_m_freegco::luaM_freegco_,
  macros::{lua_utag_limit::LUA_UTAG_LIMIT, sizeudata::sizeudata, utag_idtor::UTAG_IDTOR},
  records::{gc_object::GCObject, lua_page::lua_Page, udata::Udata},
  type_aliases::lua_state::lua_State,
};

type UserdataDtor = Option<unsafe extern "C-unwind" fn(*mut c_void)>;

pub(crate) unsafe fn lua_u_freeudata(l: *mut lua_State, u: *mut Udata, page: *mut lua_Page) {
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

    luaM_freegco_(
      l,
      u as *mut GCObject,
      sizeudata((*u).len as usize),
      (*u).memcat,
      page,
    );
  }
}
