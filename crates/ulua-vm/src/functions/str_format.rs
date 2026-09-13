//! Node: `cxx:Function:Luau.VM:VM/src/lstrlib.cpp:966:str_format`
//!
//! `string.format` — walk the format string, copying literals and dispatching
//! each `%` spec to the matching argument: `%c/d/i/o/u/x/X/e/E/f/g/G` (and
//! padded `%s`) go through the pure-Rust directive formatter (the C++ original
//! forwards to `snprintf`, which `wasm32-unknown-unknown` cannot bind — no
//! libc — so the numeric path is implemented natively for every target), `%q`
//! quotes, `%s` fast-paths long strings, `%*` appends any value, and `%%` is a
//! literal percent.

use core::{
  ffi::{c_char, c_int},
  ptr::null_mut,
  slice::from_raw_parts,
};

use crate::{
  functions::{
    addquoted::addquoted,
    format_directive::{
      format_bytes, format_char, format_float, format_int, format_uint, parse_format_spec,
    },
    lua_gettop::lua_gettop,
    lua_l_addchar::lua_l_addchar,
    lua_l_addlstring::lua_l_addlstring,
    lua_l_addvalueany::lua_l_addvalueany,
    lua_l_buffinit::lua_l_buffinit,
    lua_l_checkinteger_64::lua_l_checkinteger_64,
    lua_l_checklstring::lua_l_checklstring,
    lua_l_checknumber::lua_l_checknumber,
    lua_l_pushresult::lua_l_pushresult,
    scanformat::scanformat,
  },
  macros::{
    l_esc::L_ESC, lua_isinteger_64::lua_isinteger_64, lua_l_error::luaL_error,
    max_format::MAX_FORMAT,
  },
  records::lua_l_strbuf::{LUA_BUFFERSIZE, LuaLStrbuf},
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe extern "C-unwind" fn str_format(l: *mut lua_State) -> c_int {
  unsafe {
    let top = lua_gettop(l);
    let mut arg: i32 = 1;
    let mut sfl: usize = 0;
    let mut strfrmt = lua_l_checklstring(l, arg, &mut sfl);
    let strfrmt_end = strfrmt.add(sfl);

    let mut b: LuaLStrbuf = LuaLStrbuf {
      p: null_mut(),
      end: null_mut(),
      l: null_mut(),
      storage: null_mut(),
      buffer: [0; LUA_BUFFERSIZE],
    };
    lua_l_buffinit(l, &mut b);

    while strfrmt < strfrmt_end {
      if *strfrmt != L_ESC {
        lua_l_addchar(&mut b, *strfrmt);
        strfrmt = strfrmt.add(1);
      } else {
        strfrmt = strfrmt.add(1); // *++strfrmt
        if *strfrmt == L_ESC {
          lua_l_addchar(&mut b, *strfrmt);
          strfrmt = strfrmt.add(1); // %%
        } else if *strfrmt == b'*' as c_char {
          strfrmt = strfrmt.add(1);
          arg += 1;
          if arg > top {
            luaL_error!(l, "missing argument #{}", arg);
          }
          lua_l_addvalueany(&mut b, arg);
        } else {
          // format item
          let mut form: [c_char; MAX_FORMAT as usize] = [0; MAX_FORMAT as usize];
          arg += 1;
          if arg > top {
            luaL_error!(l, "missing argument #{}", arg);
          }
          let mut format_item_size: usize = 0;
          strfrmt = scanformat(l, strfrmt, form.as_mut_ptr(), &mut format_item_size);
          let format_indicator = *strfrmt;
          strfrmt = strfrmt.add(1);
          // `form` is `%<flags><width><.prec><conv>\0`; the spec bytes
          // sit between the `%` and the conversion character.
          let spec = parse_format_spec(from_raw_parts(
            form.as_ptr().add(1) as *const u8,
            format_item_size - 1,
          ));
          match format_indicator as u8 {
            b'c' => {
              let out = format_char(&spec, lua_l_checknumber(l, arg) as c_int as u8);
              lua_l_addlstring(&mut b, out.as_ptr() as *const c_char, out.len());
            }
            b'd' | b'i' => {
              let value: i64 = if lua_isinteger_64!(l, arg) {
                lua_l_checkinteger_64(l, arg)
              } else {
                lua_l_checknumber(l, arg) as i64
              };
              let out = format_int(&spec, value);
              lua_l_addlstring(&mut b, out.as_ptr() as *const c_char, out.len());
            }
            b'o' | b'u' | b'x' | b'X' => {
              let v: u64 = if lua_isinteger_64!(l, arg) {
                lua_l_checkinteger_64(l, arg) as u64
              } else {
                let arg_value = lua_l_checknumber(l, arg);
                if arg_value < 0.0 {
                  (arg_value as i64) as u64
                } else {
                  arg_value as u64
                }
              };
              let out = format_uint(&spec, format_indicator as u8, v);
              lua_l_addlstring(&mut b, out.as_ptr() as *const c_char, out.len());
            }
            b'e' | b'E' | b'f' | b'g' | b'G' => {
              let out = format_float(&spec, format_indicator as u8, lua_l_checknumber(l, arg));
              lua_l_addlstring(&mut b, out.as_ptr() as *const c_char, out.len());
            }
            b'q' => {
              addquoted(l, &mut b, arg);
            }
            b's' => {
              let mut slen: usize = 0;
              let s = lua_l_checklstring(l, arg, &mut slen);
              // no precision and string too long to format, or no format necessary
              if format_item_size == 1 || (spec.precision.is_none() && slen >= 100) {
                lua_l_addlstring(&mut b, s, slen);
              } else {
                let out = format_bytes(&spec, from_raw_parts(s as *const u8, slen));
                lua_l_addlstring(&mut b, out.as_ptr() as *const c_char, out.len());
              }
            }
            b'*' => {
              // %* is parsed above, so if we got here we must have %...*
              luaL_error!(l, "'%*' does not take a form");
            }
            _ => {
              // also treat cases 'pnLlh'
              luaL_error!(
                l,
                "invalid option '%{}' to 'format'",
                *(strfrmt.offset(-1)) as u8 as char
              );
            }
          }
        }
      }
    }

    lua_l_pushresult(&mut b);
    1
  }
}
