//! Node: `cxx:Function:Luau.VM:VM/src/ldebug.cpp:106:auxgetinfo`
//!
//! Fill a `lua_Debug` record from a closure + call-info, driven by the `what`
//! option string (`s` source/what/linedefined/short_src, `l` current line, `u`
//! upvalue count, `a` arity/vararg, `n` name, `f` push the function). Faithful
//! to the C++ field-by-field; returns the closure when `f` was requested.

use core::{ffi::c_char, ptr::null_mut};

use crate::{
  functions::{currentline::currentline, getfuncname::getfuncname, lua_o_chunkid::lua_o_chunkid},
  macros::{ci_func::ci_func, getstr::getstr, is_lua::isLua},
  records::{call_info::CallInfo, closure::Closure, lua_debug::LuaDebug},
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe fn auxgetinfo(
  l: *mut lua_State,
  what: *const c_char,
  ar: *mut LuaDebug,
  f: *mut Closure,
  ci: *mut CallInfo,
) -> *mut Closure {
  unsafe {
    let mut cl: *mut Closure = null_mut();

    let mut w = what;
    while *w != 0 {
      match *w as u8 {
        b's' => {
          if (*f).is_c != 0 {
            (*ar).source = c"=[C]".as_ptr();
            (*ar).what = c"C".as_ptr();
            (*ar).linedefined = -1;
            (*ar).short_src = c"[C]".as_ptr();
          } else {
            let proto = (*f).inner.l.p;
            let source = (*proto).source;
            (*ar).source = getstr(source);
            (*ar).what = c"Lua".as_ptr();
            (*ar).linedefined = (*proto).linedefined;
            (*ar).short_src = lua_o_chunkid(
              (*ar).ssbuf.as_mut_ptr(),
              (*ar).ssbuf.len(),
              getstr(source),
              (*source).len as usize,
            ) as *const c_char;
          }
        }
        b'l' => {
          if !ci.is_null() {
            (*ar).currentline = if isLua!(ci) { currentline(l, ci) } else { -1 };
          } else {
            (*ar).currentline = if (*f).is_c != 0 {
              -1
            } else {
              (*(*f).inner.l.p).linedefined
            };
          }
        }
        b'u' => {
          (*ar).nupvals = (*f).nupvalues;
        }
        b'a' => {
          if (*f).is_c != 0 {
            (*ar).isvararg = 1;
            (*ar).nparams = 0;
          } else {
            let proto = (*f).inner.l.p;
            (*ar).isvararg = (*proto).is_vararg as c_char;
            (*ar).nparams = (*proto).numparams;
          }
        }
        b'n' => {
          (*ar).name = if !ci.is_null() {
            getfuncname(ci_func!(ci))
          } else {
            getfuncname(f)
          };
        }
        b'f' => {
          cl = f;
        }
        _ => {}
      }

      w = w.add(1);
    }

    cl
  }
}
