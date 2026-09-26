use core::{
  ffi::c_void,
  ptr::{eq, from_mut},
};

use crate::{
  functions::{
    ensure_stack::ensure_stack, f_call::f_call, index_2_addr::index_2_addr,
    lua_d_pcall::lua_d_pcall,
  },
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, lua_multret::LUA_MULTRET,
    lua_o_nilobject::LUA_O_NILOBJECT, savestack::savestack,
  },
  records::{call_s::CallS, lua_state::LuaState},
  type_aliases::{pfunc::Pfunc, stk_id::StkId},
};

/// # Safety
/// `l` 须为存活 LuaState、`(*l).status == 0` 且处于可建保护帧处；栈顶 `nargs + 1` 项为 func + 实参
/// （`api_checknelems` 校验，release 由调用方保证），`nargs >= 0`、`nresults >= LUA_MULTRET`；`errfunc != 0` 时经
/// `index_2_addr` 解析为非 nil 栈槽（`savestack` 记录跨重分配偏移）；`lua_d_pcall` 建立保护帧，捕获错误返回状态码。
/// cpp/VM/src/lapi.cpp:1176 lua_pcall。
pub unsafe fn lua_pcall(l: *mut LuaState, nargs: i32, nresults: i32, errfunc: i32) -> i32 {
  unsafe {
    api_check!(l, nargs >= 0);
    api_check!(l, nresults >= LUA_MULTRET);
    api_checknelems!(l, nargs + 1);
    api_check!(l, (*l).status == 0);

    // cpp `ensure_stack(L, nresults - (nargs + 1))`
    if nresults > nargs + 1 {
      ensure_stack(l, nresults - (nargs + 1));
    }

    let mut func: isize = 0;
    if errfunc != 0 {
      let o: StkId = index_2_addr(l, errfunc);
      api_check!(l, !eq(o, LUA_O_NILOBJECT));
      func = savestack!(l, o) as isize;
    }

    let mut c = CallS {
      func: (*l).top.sub((nargs + 1) as usize),
      nresults,
    };

    let pfunc: Pfunc = Some(f_call);

    let status = lua_d_pcall(
      l,
      pfunc,
      from_mut(&mut c).cast::<c_void>(),
      savestack!(l, c.func) as isize,
      func,
    );

    if nresults == LUA_MULTRET && (*l).top.offset_from((*(*l).ci).top) >= 0 {
      (*(*l).ci).top = (*l).top;
    }

    status
  }
}
