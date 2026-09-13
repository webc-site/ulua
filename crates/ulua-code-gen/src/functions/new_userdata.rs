use ulua_common::macros::luau_assert::LUAU_ASSERT;
use ulua_vm::{
  functions::lua_u_newudata::lua_u_newudata,
  macros::isblack::isblack,
  records::{gc_object::GCObject, lua_state::lua_State, udata::Udata},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn new_userdata(l: *mut lua_State, s: usize, tag: i32) -> *mut Udata {
  unsafe {
    let u = lua_u_newudata(l, s, tag);

    let h = (*(*l).global).udatamt[tag as usize];
    if !h.is_null() {
      // currently, we always allocate unmarked objects, so forward barrier can be skipped
      LUAU_ASSERT!(!isblack!(u as *mut GCObject));

      (*u).metatable = h;
    }

    u
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_newUserdata")]
pub unsafe extern "C-unwind" fn new_userdata_export(
  l: *mut lua_State,
  s: usize,
  tag: i32,
) -> *mut Udata {
  unsafe { new_userdata(l, s, tag) }
}
