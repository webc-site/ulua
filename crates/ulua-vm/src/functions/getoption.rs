use core::ffi::c_char;

use crate::{
  enums::k_option::KOption,
  functions::{
    getnum::{FmtCursor, getnum},
    getnumlimit::getnumlimit,
  },
  macros::{lua_l_error::luaL_error, maxalign::MAXALIGN},
  records::header::Header,
};

/// `i`/`I`/`s` 系选项缺省字段尺寸（cpp `sizeof(int)` = 4 的字面量形态）。
const K_DEFAULT_INT_SIZE: i32 = 4;

/// cpp `lstrlib.cpp:getoption`：读一个格式选项，返回 `(尺寸, 选项)`。
///
/// cpp 用 `int* size` 出参 + 返回 KOption，Rust 版折叠为元组。
///
/// 前置条件（由 `str_format` 建立的 `Header` 对象不变式承载，非调用方裸指针契约）：
/// `h.l` 必须指向本次 strlib 调用的存活 `lua_State`，`fmt` 游标位于格式串界内。
/// 选项分派为纯字节匹配，故签名安全；`unsafe` 只留在两处报错内核。
pub(crate) fn getoption(h: &mut Header, fmt: &mut FmtCursor) -> (i32, KOption) {
  let opt = fmt.cur() as c_char;
  fmt.bump();

  // 各选项对应的字段尺寸；'x'/'X'/'!' 为对齐控制，'c'/'i'/'I'/'s' 带位宽参数
  match opt as u8 {
    b'b' => (1, KOption::Kint),
    b'B' => (1, KOption::Kuint),
    b'h' => (2, KOption::Kint),
    b'H' => (2, KOption::Kuint),
    b'l' => (8, KOption::Kint),
    b'L' => (8, KOption::Kuint),
    b'j' => (4, KOption::Kint),
    b'J' => (4, KOption::Kuint),
    b'T' => (4, KOption::Kuint),
    b'f' => (4, KOption::Kfloat),
    b'd' | b'n' => (8, KOption::Kfloat),
    b'i' => (getnumlimit(h, fmt, K_DEFAULT_INT_SIZE), KOption::Kint),
    b'I' => (getnumlimit(h, fmt, K_DEFAULT_INT_SIZE), KOption::Kuint),
    b's' => (getnumlimit(h, fmt, K_DEFAULT_INT_SIZE), KOption::Kstring),
    b'c' => {
      let n = getnum(h, fmt, -1);
      if n == -1 {
        // SAFETY: 前置条件保证 `h.l` 为可抛错的存活 `lua_State`，报错后不返回
        unsafe { luaL_error!(&mut *h.l, "missing size for format option 'c'") };
      }
      (n, KOption::Kchar)
    }
    b'z' => (0, KOption::Kzstr),
    b'x' => (1, KOption::Kpadding),
    b'X' => (0, KOption::Kpaddalign),
    b' ' => (0, KOption::Knop),
    b'<' => {
      h.islittle = 1;
      (0, KOption::Knop)
    }
    b'>' => {
      h.islittle = 0;
      (0, KOption::Knop)
    }
    // '='：跟随宿主字节序
    b'=' => {
      h.islittle = cfg!(target_endian = "little") as i32;
      (0, KOption::Knop)
    }
    b'!' => {
      h.maxalign = getnumlimit(h, fmt, MAXALIGN);
      (0, KOption::Knop)
    }
    _ => {
      // SAFETY: 前置条件保证 `h.l` 为可抛错的存活 `lua_State`，报错后不返回
      unsafe { luaL_error!(&mut *h.l, "invalid format option '{}'", opt as u8 as char) }
    }
  }
}
