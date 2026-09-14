//! Node: `cxx:Function:Luau.VM:VM/src/lvmutils.cpp:399:luaV_concat`
//! Source: `VM/src/lvmutils.cpp:399-461` (hand-ported)

use core::{
  ffi::c_char,
  ptr::{copy_nonoverlapping, null_mut},
};

use crate::{
  enums::tms::TMS,
  functions::{
    call_bin_tm::call_bin_tm, lua_g_concaterror::lua_g_concaterror,
    lua_s_buffinish::luaS_buffinish, lua_s_bufstart::luaS_bufstart, lua_s_newlstr::luaS_newlstr,
  },
  macros::{
    lua_g_runerror::lua_g_runerror, maxssize::MAXSSIZE, setsvalue::setsvalue, svalue::svalue,
    tostring::tostring, tsvalue::tsvalue, ttisnumber::ttisnumber, ttisstring::ttisstring,
  },
  records::{lua_l_strbuf::LUA_BUFFERSIZE, t_string::tstring},
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_v_concat(l: *mut lua_State, mut total: i32, mut last: i32) {
  unsafe {
    loop {
      let top: StkId = (*l).base.add((last + 1) as usize);
      let mut n = 2; // number of elements handled in this pass (at least 2)
      if !(ttisstring!(top.sub(2)) || ttisnumber!(top.sub(2))) || !tostring!(l, top.sub(1)) {
        if call_bin_tm(l, top.sub(2), top.sub(1), top.sub(2), TMS::TmConcat) == 0 {
          lua_g_concaterror(l, top.sub(2), top.sub(1));
        }
      } else if (*tsvalue!(top.sub(1))).len == 0 {
        // second op is empty? result is first op (as string)
        let _ = tostring!(l, top.sub(2));
      } else {
        // at least two string values; get as many as possible
        let mut tl = (*tsvalue!(top.sub(1))).len as usize;
        // collect total length
        n = 1;
        while n < total && tostring!(l, top.sub((n + 1) as usize)) {
          let slen = (*tsvalue!(top.sub((n + 1) as usize))).len as usize;
          if slen > MAXSSIZE as usize - tl {
            lua_g_runerror!(l, "string length overflow");
          }
          tl += slen;
          n += 1;
        }

        let mut buf = [0 as c_char; LUA_BUFFERSIZE];
        let mut ts: *mut tstring = null_mut();

        let buffer: *mut c_char = if tl < LUA_BUFFERSIZE {
          buf.as_mut_ptr()
        } else {
          ts = luaS_bufstart(l, tl);
          (*ts).data.as_mut_ptr()
        };

        // concat all strings
        tl = 0;
        let mut i = n;
        while i > 0 {
          let l = (*tsvalue!(top.sub(i as usize))).len as usize;
          copy_nonoverlapping(svalue!(top.sub(i as usize)), buffer.add(tl), l);
          tl += l;
          i -= 1;
        }

        if tl < LUA_BUFFERSIZE {
          setsvalue!(l, top.sub(n as usize), luaS_newlstr(l, buffer, tl));
        } else {
          setsvalue!(l, top.sub(n as usize), luaS_buffinish(l, ts));
        }
      }
      total -= n - 1; // got `n` strings to create 1 new
      last -= n - 1;
      if total <= 1 {
        break; // repeat until only 1 result left
      }
    }
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaV_concat")]
pub unsafe extern "C-unwind" fn lua_v_concat_export(l: *mut lua_State, total: i32, last: i32) {
  unsafe {
    lua_v_concat(l, total, last);
  }
}

pub use lua_v_concat as luaV_concat;
