//! Node: `cxx:Function:Luau.VM:VM/src/lstrlib.cpp:1407:str_pack`

use core::{
  ffi::{CStr, c_char, c_double, c_float, c_int},
  mem::size_of,
  ptr::null_mut,
};

use crate::{
  enums::k_option::KOption,
  functions::{
    copywithendian::copywithendian, getdetails::getdetails, initheader::initheader,
    lua_l_addchar::lua_l_addchar, lua_l_addlstring::lua_l_addlstring,
    lua_l_buffinit::lua_l_buffinit, lua_l_checklstring::lua_l_checklstring,
    lua_l_checknumber::lua_l_checknumber, lua_l_pushresult::lua_l_pushresult,
    lua_pushnil::lua_pushnil, packint::packint,
  },
  macros::{lua_l_argcheck::luaL_argcheck, lua_l_checkstring::luaL_checkstring},
  records::{
    ftypes::Ftypes,
    header::Header,
    lua_l_strbuf::{LUA_BUFFERSIZE, LuaLStrbuf},
  },
  type_aliases::lua_state::lua_State,
};

pub const LUAL_PACKPADBYTE: u8 = 0x00;

pub(crate) unsafe extern "C-unwind" fn str_pack(l: *mut lua_State) -> c_int {
  unsafe {
    let mut b = LuaLStrbuf {
      p: null_mut(),
      end: null_mut(),
      l: null_mut(),
      storage: null_mut(),
      buffer: [0; LUA_BUFFERSIZE],
    };
    let mut h = Header::default();
    let mut fmt = luaL_checkstring!(l, 1); // format string
    let mut arg = 1; // current argument to pack
    let mut totalsize: usize = 0; // accumulate total size of result
    initheader(l, &mut h);
    lua_pushnil(l); // mark to separate arguments from string buffer
    lua_l_buffinit(l, &mut b);
    while *fmt != 0 {
      let mut size: i32 = 0;
      let mut ntoalign: i32 = 0;
      let opt = getdetails(&mut h, totalsize, &mut fmt, &mut size, &mut ntoalign);
      totalsize += (ntoalign + size) as usize;
      while ntoalign > 0 {
        ntoalign -= 1;
        lua_l_addchar(&mut b, LUAL_PACKPADBYTE as c_char);
        // fill alignment
      }
      arg += 1;
      match opt {
        KOption::Kint => {
          // signed integers
          let n = lua_l_checknumber(l, arg) as i64;
          if size < size_of::<i64>() as c_int {
            // need overflow check?
            let lim = 1i64 << ((size * 8) - 1);
            luaL_argcheck!(l, -lim <= n && n < lim, arg, "integer overflow");
          }
          packint(&mut b, n as u64, h.islittle, size, (n < 0) as i32);
        }
        KOption::Kuint => {
          // unsigned integers
          let n = lua_l_checknumber(l, arg) as i64;
          if size < size_of::<i64>() as c_int {
            // need overflow check?
            luaL_argcheck!(
              l,
              (n as u64) < (1u64 << (size * 8)),
              arg,
              "unsigned overflow"
            );
          }
          packint(&mut b, n as u64, h.islittle, size, 0);
        }
        KOption::Kfloat => {
          // floating-point options
          let mut u = Ftypes { n: 0.0 };
          let mut buff = [0 as c_char; 16]; // MAXINTSIZE
          let n = lua_l_checknumber(l, arg); // get argument
          if size as usize == size_of::<c_float>() {
            u.f = n as c_float; // copy it into 'u'
          } else if size as usize == size_of::<c_double>() {
            u.d = n;
          } else {
            u.n = n;
          }
          // move 'u' to final result, correcting endianness if needed
          copywithendian(buff.as_mut_ptr(), u.buff.as_ptr(), size, h.islittle);
          lua_l_addlstring(&mut b, buff.as_ptr(), size as usize);
        }
        KOption::Kchar => {
          // fixed-size string
          let mut len: usize = 0;
          let s = lua_l_checklstring(l, arg, &mut len);
          luaL_argcheck!(
            l,
            len <= size as usize,
            arg,
            "string longer than given size"
          );
          lua_l_addlstring(&mut b, s, len); // add string
          while len < size as usize {
            len += 1;
            lua_l_addchar(&mut b, LUAL_PACKPADBYTE as c_char);
          }
        }
        KOption::Kstring => {
          // strings with length count
          let mut len: usize = 0;
          let s = lua_l_checklstring(l, arg, &mut len);
          luaL_argcheck!(
            l,
            size >= size_of::<usize>() as c_int || len < (1usize << (size * 8)),
            arg,
            "string length does not fit in given size"
          );
          packint(&mut b, len as u64, h.islittle, size, 0); // pack length
          lua_l_addlstring(&mut b, s, len);
          totalsize += len;
        }
        KOption::Kzstr => {
          // zero-terminated string
          let mut len: usize = 0;
          let s = lua_l_checklstring(l, arg, &mut len);
          luaL_argcheck!(
            l,
            CStr::from_ptr(s).to_bytes().len() == len,
            arg,
            "string contains zeros"
          );
          lua_l_addlstring(&mut b, s, len);
          lua_l_addchar(&mut b, 0); // add zero at the end
          totalsize += len + 1;
        }
        KOption::Kpadding => {
          lua_l_addchar(&mut b, LUAL_PACKPADBYTE as c_char);
          arg -= 1; // undo increment
        }
        KOption::Kpaddalign | KOption::Knop => {
          arg -= 1; // undo increment
        }
      }
    }
    lua_l_pushresult(&mut b);
    1
  }
}
