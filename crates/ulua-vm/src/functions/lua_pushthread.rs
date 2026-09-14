use crate::{
  functions::lua_c_barrierback::lua_c_barrierback,
  macros::{api_incr_top::api_incr_top, blackbit::BLACKBIT, setthvalue::setthvalue},
  records::gc_object::GCObject,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_pushthread(l: *mut lua_State) -> i32 {
  unsafe {
    // Inline luaC_threadbarrier(l) to avoid buggy macros in the crate
    let marked = (*l).hdr.marked as i32;
    if (marked & (1 << BLACKBIT)) != 0 {
      lua_c_barrierback(
        l,
        l as *mut GCObject,
        &mut (*l).gclist as *mut *mut GCObject,
      );
    }

    setthvalue!(l, (*l).top, l);
    api_incr_top!(l);
    ((*(*l).global).mainthread == l) as i32
  }
}
