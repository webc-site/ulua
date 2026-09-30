use core::{
  ffi::{c_double, c_float},
  mem::size_of,
  slice::from_raw_parts,
};

use memchr::memchr;

use crate::{
  enums::k_option::KOption,
  functions::{
    copywithendian::copywithendian, getdetails::getdetails, getnum::FmtCursor,
    initheader::initheader, lua_l_checklstring::lua_l_checklstring,
    lua_l_checkstack::lua_l_checkstack, lua_l_optinteger::lua_l_optinteger,
    lua_pushlstring::lua_pushlstring_bytes, posrelat::posrelat, unpackint::unpackint,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::{ftypes::Ftypes, header::Header, lua_state::LuaState},
};

/// # Safety
///
/// `l` 必须是正在执行的 string 库 C 函数帧的存活 `LuaState`：#2 数据串 payload 由
/// `lua_l_checklstring` 契约在本次调用全程存活（含尾部终止 NUL，切片含 `ld + 1` 字节），
/// 循环内 push 不挪动 GC 串，各读取窗口在 `arg_check` 的剩余量校验后按 `pos`/`size`
/// 切出；结果逐项压栈经 `lua_l_checkstack` 保余量。cpp lstrlib.cpp:1562 `str_unpack`。
pub(crate) unsafe fn str_unpack(l: *mut LuaState) -> i32 {
  unsafe {
    let mut h = Header::default();
    let mut fmt = FmtCursor::from_ptr((*l).check_bytes(1).as_ptr().cast());

    let mut ld: usize = 0;
    let data = lua_l_checklstring(l, 2, &mut ld);
    // 含终止 NUL 的单一可读切片：全程按下标切窗，免逐点 add/from_raw_parts 指针算术
    let win = from_raw_parts(data as *const u8, ld + 1);
    let mut pos = posrelat(lua_l_optinteger(l, 3, 1), ld) - 1;
    if pos < 0 {
      pos = 0;
    }

    let mut n = 0;
    (*l).arg_check(pos as usize <= ld, 3, "initial position out of string");
    initheader(l, &mut h);

    while fmt.cur() != 0 {
      let (opt, size, ntoalign) = getdetails(&mut h, pos as usize, &mut fmt);
      (*l).arg_check(
        (ntoalign as usize).wrapping_add(size as usize) <= ld - pos as usize,
        2,
        "data string too short",
      );

      pos += ntoalign;
      // 钳位保证 p ∈ [0, ld]，各读取窗口 win[p..p+size] 恒落在含终止 NUL 的切片内
      let p = pos as usize;
      lua_l_checkstack(l, 2, "too many results");
      n += 1;

      match opt {
        KOption::Kint | KOption::Kuint => {
          // 有/无符号双臂仅差 unpackint 符号扩展位与回推路径（i64 直转 vs 位重解读 u64）
          let signed = opt == KOption::Kint;
          let res = unpackint(
            l,
            &win[p..p + size as usize],
            h.islittle,
            size,
            signed as i32,
          );
          (*l).push_number(if signed {
            res as f64
          } else {
            res as u64 as f64
          });
        }
        KOption::Kfloat => {
          let mut u = Ftypes { n: 0.0 };
          copywithendian(
            &mut u.buff[..size as usize],
            &win[p..p + size as usize],
            h.islittle,
          );
          let num = if size as usize == size_of::<c_float>() {
            u.f as f64
          } else if size as usize == size_of::<c_double>() {
            u.d
          } else {
            u.n
          };
          (*l).push_number(num);
        }
        KOption::Kchar => {
          lua_pushlstring_bytes(l, &win[p..p + size as usize]);
        }
        KOption::Kstring => {
          let len = unpackint(l, &win[p..p + size as usize], h.islittle, size, 0) as usize;
          (*l).arg_check(len <= ld - p - size as usize, 2, "data string too short");
          let q = p + size as usize;
          lua_pushlstring_bytes(l, &win[q..q + len]);
          pos += len as i32;
        }
        KOption::Kzstr => {
          // 从当前偏移单遍扫描首个 NUL 求 strlen（含终止 NUL 恒命中）
          let len = memchr(0, &win[p..]).unwrap_or(0);
          (*l).arg_check(p + len < ld, 2, "unfinished string for format 'z'");
          lua_pushlstring_bytes(l, &win[p..p + len]);
          pos += len as i32 + 1;
        }
        KOption::Kpaddalign | KOption::Kpadding | KOption::Knop => {
          n -= 1;
        }
      }

      pos += size;
    }

    (*l).push_integer(pos + 1);
    n + 1
  }
}

lua_lib_fn!(pub(crate) fn str_unpack, str_unpack_arm);
