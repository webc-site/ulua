//! Source: `VM/src/ldebug.cpp:106`
//!
//! Fill a `lua_Debug` record from a closure + call-info, driven by the `what`
//! option string (`s` source/what/linedefined/short_src, `l` current line, `u`
//! upvalue count, `a` arity/vararg, `n` name, `f` push the function). Faithful
//! to the C++ field-by-field; returns the closure when `f` was requested.

use core::{ffi::c_char, ptr::null_mut};

use crate::{
  functions::{
    cstr_bytes, currentline::currentline, getfuncname::getfuncname, lua_o_chunkid::lua_o_chunkid,
  },
  macros::{ci_func::ci_func, getstr::getstr, is_lua::isLua, short_src_c::SHORT_SRC_C},
  records::{call_info::CallInfo, closure::Closure, lua_debug::LuaDebug, lua_state::LuaState},
};
/// NUL 结尾字节串（`*const c_char` 契约调用点 `.as_ptr().cast()`；§10 不引入 `CStr`/`c"…"`）。
const SRC_C: &[u8] = b"=[C]\0";
const WHAT_C: &[u8] = b"C\0";
const WHAT_LUA: &[u8] = b"Lua\0";

/// # Safety
/// `l` 必须指向存活 `LuaState` 且所查询的调用帧/Proto/输出记录按约定存活可写。
pub(crate) unsafe fn auxgetinfo(
  l: *mut LuaState,
  what: *const c_char,
  ar: *mut LuaDebug,
  f: *mut Closure,
  ci: *mut CallInfo,
) -> *mut Closure {
  // Safety: 契约保证 `ci` 为当前调用栈中的存活帧、`f` 与其一致（或为空由 ci 取得）、`ar`/`what` 可写/可读，块内字段填充与推值不越界
  unsafe {
    let mut cl: *mut Closure = null_mut();

    // C++ `for (; *what; what++)`：CStr 零拷贝迭代选项串
    for &ch in cstr_bytes(what) {
      match ch {
        b's' => {
          if (*f).is_c != 0 {
            (*ar).source = SRC_C.as_ptr().cast();
            (*ar).what = WHAT_C.as_ptr().cast();
            (*ar).linedefined = -1;
            (*ar).short_src = SHORT_SRC_C.as_ptr().cast();
          } else {
            let proto = (*f).inner.l.p;
            let source = (*proto).source;
            (*ar).source = getstr(source);
            (*ar).what = WHAT_LUA.as_ptr().cast();
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
        b'p' => {
          if (*f).is_c != 0 {
            (*ar).protoid = 0;
            (*ar).bytecodeid = -1;
          } else {
            let proto = (*f).inner.l.p;
            (*ar).protoid = (*proto).funid as i32;
            (*ar).bytecodeid = (*proto).bytecodeid;
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
    }

    cl
  }
}
