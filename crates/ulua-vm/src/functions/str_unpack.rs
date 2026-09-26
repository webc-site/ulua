use core::{
  ffi::{c_char, c_double, c_float},
  mem::size_of,
  slice::from_raw_parts,
};

use crate::{
  enums::k_option::KOption,
  functions::{
    copywithendian::copywithendian, cstr_bytes, getdetails::getdetails, getnum::FmtCursor,
    initheader::initheader, lua_l_checklstring::lua_l_checklstring,
    lua_l_checkstack::lua_l_checkstack, lua_l_optinteger::lua_l_optinteger,
    lua_pushinteger::lua_pushinteger, lua_pushlstring::lua_pushlstring,
    lua_pushnumber::lua_pushnumber, posrelat::posrelat, unpackint::unpackint,
  },
  macros::{lua_l_argcheck::luaL_argcheck, lua_l_checkstring::luaL_checkstring},
  records::{ftypes::Ftypes, header::Header, lua_state::LuaState},
};

/// # Safety
///
/// `l` 必须是正在执行的 string 库 C 函数帧的存活 `LuaState`：#2 数据串 payload 由
/// `lua_l_checklstring` 契约在本次调用全程存活（循环内 push 不挪动 GC 串），各 `unpackint`
/// 窗口在 `luaL_argcheck` 的剩余量校验后按 `pos`/`size` 切出；结果逐项压栈经
/// `lua_l_checkstack` 保余量。cpp lstrlib.cpp:1562 `str_unpack`。
pub(crate) unsafe extern "C-unwind" fn str_unpack(l: *mut LuaState) -> i32 {
  unsafe {
    let mut h = Header::default();
    let mut fmt = FmtCursor::from_ptr(luaL_checkstring!(l, 1));

    let mut ld: usize = 0;
    let data = lua_l_checklstring(l, 2, &mut ld);
    let mut pos = posrelat(lua_l_optinteger(l, 3, 1), ld) - 1;
    if pos < 0 {
      pos = 0;
    }

    let mut n = 0;
    luaL_argcheck!(l, pos as usize <= ld, 3, "initial position out of string");
    initheader(l, &mut h);

    while fmt.cur() != 0 {
      let (opt, size, ntoalign) = getdetails(&mut h, pos as usize, &mut fmt);
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
        KOption::Kint | KOption::Kuint => {
          // 有/无符号双臂仅差 unpackint 符号扩展位与回推路径（i64 直转 vs 位重解读 u64）
          let signed = opt == KOption::Kint;
          let res = unpackint(
            l,
            from_raw_parts(data.add(pos as usize) as *const u8, size as usize),
            h.islittle,
            size,
            signed as i32,
          );
          lua_pushnumber(
            l,
            if signed {
              res as f64
            } else {
              res as u64 as f64
            },
          );
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
          let len = unpackint(
            l,
            from_raw_parts(data.add(pos as usize) as *const u8, size as usize),
            h.islittle,
            size,
            0,
          ) as usize;
          luaL_argcheck!(
            l,
            len <= ld - pos as usize - size as usize,
            2,
            "data string too short"
          );
          lua_pushlstring(l, data.add(pos as usize + size as usize), len);
          pos += len as i32;
        }
        KOption::Kzstr => {
          let len = cstr_bytes(data.add(pos as usize)).len();
          luaL_argcheck!(
            l,
            pos as usize + len < ld,
            2,
            "unfinished string for format 'z'"
          );
          lua_pushlstring(l, data.add(pos as usize), len);
          pos += len as i32 + 1;
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
