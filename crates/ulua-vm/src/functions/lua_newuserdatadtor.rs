use core::{
  ffi::{c_int, c_void},
  mem::size_of,
  ptr::copy_nonoverlapping,
};

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_concat::lua_c_threadbarrier_lapi, lua_u_newudata::lua_u_newudata},
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, checkliveness::checkliveness,
    lua_c_check_gc::luaC_checkGC, utag_idtor::UTAG_IDTOR,
  },
  records::{gc_object::GCObject, lua_state::lua_State},
};

type UserdataDtor = Option<unsafe extern "C-unwind" fn(*mut c_void)>;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_newuserdatadtor(l: *mut lua_State, sz: usize, dtor: UserdataDtor) -> *mut c_void {
  unsafe {
    api_check!(l, dtor.is_some());
    luaC_checkGC!(l);
    lua_c_threadbarrier_lapi(l);

    let dtor_size = size_of::<UserdataDtor>();
    let as_ = if sz < usize::MAX - dtor_size {
      sz + dtor_size
    } else {
      usize::MAX
    };

    let u = lua_u_newudata(l, as_, UTAG_IDTOR);
    copy_nonoverlapping(
      (&dtor as *const UserdataDtor).cast::<u8>(),
      (*u).data.as_mut_ptr().add(sz).cast::<u8>(),
      dtor_size,
    );

    (*(*l).top).value.gc = u as *mut GCObject;
    (*(*l).top).tt = LuaType::UserData as c_int;
    checkliveness!((*l).global, (*l).top);
    api_incr_top!(l);

    (*u).data.as_mut_ptr().cast()
  }
}
