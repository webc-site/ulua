use core::{
  ffi::{CStr, c_char, c_double, c_float, c_int},
  mem::size_of,
};

use crate::{
  enums::k_option::KOption,
  functions::{
    copywithendian::copywithendian, getdetails::getdetails, initheader::initheader,
    lua_l_checklstring::lua_l_checklstring, lua_l_checkstack::lua_l_checkstack,
    lua_l_optinteger::lua_l_optinteger, lua_pushinteger::lua_pushinteger,
    lua_pushlstring::lua_pushlstring, lua_pushnumber::lua_pushnumber, posrelat::posrelat,
    unpackint::unpackint,
  },
  macros::{lua_l_argcheck::luaL_argcheck, lua_l_checkstring::luaL_checkstring},
  records::{ftypes::Ftypes, header::Header},
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe extern "C-unwind" fn str_unpack(l: *mut lua_State) -> c_int {
  unsafe {
    let mut h = Header::default();
    let mut fmt = luaL_checkstring!(l, 1);

    let mut ld: usize = 0;
    let data = lua_l_checklstring(l, 2, &mut ld);
    let mut pos = posrelat(lua_l_optinteger(l, 3, 1), ld) - 1;
    if pos < 0 {
      pos = 0;
    }

    let mut n = 0;
    luaL_argcheck!(l, pos as usize <= ld, 3, "initial position out of string");
    initheader(l, &mut h);

    while *fmt != 0 {
      let mut size: i32 = 0;
      let mut ntoalign: i32 = 0;
      let opt = getdetails(&mut h, pos as usize, &mut fmt, &mut size, &mut ntoalign);
      luaL_argcheck!(
        l,
        (ntoalign as usize).wrapping_add(size as usize) <= ld - pos as usize,
        2,
        "data string too short"
      );

      pos += ntoalign;
      lua_l_checkstack(l, 2, "too many results");
      n += 1;

      match opt {
        KOption::Kint => {
          let res = unpackint(l, data.add(pos as usize), h.islittle, size, 1);
          lua_pushnumber(l, res as f64);
        }
        KOption::Kuint => {
          let res = unpackint(l, data.add(pos as usize), h.islittle, size, 0) as u64;
          lua_pushnumber(l, res as f64);
        }
        KOption::Kfloat => {
          let mut u = Ftypes { n: 0.0 };
          copywithendian(
            u.buff.as_mut_ptr(),
            data.add(pos as usize) as *const c_char,
            size,
            h.islittle,
          );
          let num = if size as usize == size_of::<c_float>() {
            u.f as f64
          } else if size as usize == size_of::<c_double>() {
            u.d
          } else {
            u.n
          };
          lua_pushnumber(l, num);
        }
        KOption::Kchar => {
          lua_pushlstring(l, data.add(pos as usize), size as usize);
        }
        KOption::Kstring => {
          let len = unpackint(l, data.add(pos as usize), h.islittle, size, 0) as usize;
          luaL_argcheck!(
            l,
            len <= ld - pos as usize - size as usize,
            2,
            "data string too short"
          );
          lua_pushlstring(l, data.add(pos as usize + size as usize), len);
          pos += len as c_int;
        }
        KOption::Kzstr => {
          let len = CStr::from_ptr(data.add(pos as usize)).to_bytes().len();
          luaL_argcheck!(
            l,
            pos as usize + len < ld,
            2,
            "unfinished string for format 'z'"
          );
          lua_pushlstring(l, data.add(pos as usize), len);
          pos += len as c_int + 1;
        }
        KOption::Kpaddalign | KOption::Kpadding | KOption::Knop => {
          n -= 1;
        }
      }

      pos += size;
    }

    lua_pushinteger(l, pos + 1);
    n + 1
  }
}
