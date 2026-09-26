use core::{ffi::c_void, mem::size_of, ptr::copy_nonoverlapping};

use crate::{
  enums::lua_type::LuaType,
  functions::{
    ensure_stack::ensure_stack, lapi_barrier::lua_c_threadbarrier_lapi,
    lua_u_newudata::lua_u_newudata,
  },
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, checkliveness::checkliveness,
    lua_c_check_gc::lua_c_check_gc, utag_idtor::UTAG_IDTOR,
  },
  records::{gc_object::GCObject, lua_state::LuaState},
};

type UserdataDtor = Option<unsafe extern "C-unwind" fn(*mut c_void)>;

/// # Safety
/// `l` 须为存活 `LuaState`；`dtor` 须为 `Some`（`api_check`，其函数指针在 udata 回收时被调），`sz`
/// 为请求的 payload 字节数（`sz+size_of::<UserdataDtor>()` 不得溢出，内部已钳位）；`lua_c_check_gc`/
/// `lua_u_newudata`/`ensure_stack` 可分配并把 udata 挂到 `(*l).top`，须在受保护帧内调用。cpp `lapi.cpp:1671`。
pub unsafe fn lua_newuserdatadtor(l: *mut LuaState, sz: usize, dtor: UserdataDtor) -> *mut c_void {
  unsafe {
    api_check!(l, dtor.is_some());
    lua_c_check_gc!(l);
    lua_c_threadbarrier_lapi(l);
    ensure_stack(l, 1);

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
    (*(*l).top).tt = LuaType::UserData as i32;
    checkliveness!((*l).global, (*l).top);
    api_incr_top!(l);

    (*u).data.as_mut_ptr().cast()
  }
}
