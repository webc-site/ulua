use core::{
  ffi::{c_int, c_void},
  ptr::eq,
};

use crate::{
  functions::{f_call::f_call, index_2_addr::index2addr, lua_d_pcall::luaD_pcall},
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, lua_multret::LUA_MULTRET,
    lua_o_nilobject::luaO_nilobject, savestack::savestack,
  },
  records::{call_s::CallS, lua_state::lua_State},
  type_aliases::{pfunc::Pfunc, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_pcall(l: *mut lua_State, nargs: c_int, nresults: c_int, errfunc: c_int) -> c_int {
  unsafe {
    api_check!(l, nargs >= 0);
    api_check!(l, nresults >= LUA_MULTRET);
    api_checknelems!(l, nargs + 1);
    api_check!(l, (*l).status == 0);
    api_check!(
      l,
      nresults == LUA_MULTRET
        || (*(*l).ci).top.offset_from((*l).top) >= (nresults - nargs) as isize
    );

    let mut func: isize = 0;
    if errfunc != 0 {
      let o: StkId = index2addr(l, errfunc);
      api_check!(l, !eq(o, luaO_nilobject));
      func = savestack!(l, o) as isize;
    }

    let mut c = CallS {
      func: (*l).top.sub((nargs + 1) as usize),
      nresults,
    };

    let pfunc: Pfunc = Some(f_call);

    let status = luaD_pcall(
      l,
      pfunc,
      &mut c as *mut CallS as *mut c_void,
      savestack!(l, c.func) as isize,
      func,
    );

    if nresults == LUA_MULTRET && (*l).top.offset_from((*(*l).ci).top) >= 0 {
      (*(*l).ci).top = (*l).top;
    }

    status
  }
}
