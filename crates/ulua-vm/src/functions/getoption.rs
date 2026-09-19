use core::ffi::c_char;

use crate::{
  enums::k_option::KOption,
  functions::{getnum::getnum, getnumlimit::getnumlimit},
  macros::{lua_l_error::luaL_error, maxalign::MAXALIGN},
  records::header::Header,
};

/// cpp `lstrlib.cpp:getoption`：读一个格式选项，返回 `(尺寸, 选项)`。
///
/// cpp 用 `int* size` 出参 + 返回 KOption，Rust 版折叠为元组。
///
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn getoption(h: *mut Header, fmt: *mut *const c_char) -> (i32, KOption) {
  unsafe {
    let opt = **fmt as c_char;
    *fmt = (*fmt).add(1);

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
      b'i' => (getnumlimit(h, fmt, 4), KOption::Kint),
      b'I' => (getnumlimit(h, fmt, 4), KOption::Kuint),
      b's' => (getnumlimit(h, fmt, 4), KOption::Kstring),
      b'c' => {
        let n = getnum(h, fmt, -1);
        if n == -1 {
          luaL_error!((*h).l, "missing size for format option 'c'");
        }
        (n, KOption::Kchar)
      }
      b'z' => (0, KOption::Kzstr),
      b'x' => (1, KOption::Kpadding),
      b'X' => (0, KOption::Kpaddalign),
      b' ' => (0, KOption::Knop),
      b'<' => {
        (*h).islittle = 1;
        (0, KOption::Knop)
      }
      b'>' => {
        (*h).islittle = 0;
        (0, KOption::Knop)
      }
      b'=' => (
        0,
        if cfg!(target_endian = "little") {
          (*h).islittle = 1;
          KOption::Knop
        } else {
          (*h).islittle = 0;
          KOption::Knop
        },
      ),
      b'!' => {
        (*h).maxalign = getnumlimit(h, fmt, MAXALIGN);
        (0, KOption::Knop)
      }
      _ => {
        luaL_error!((*h).l, "invalid format option '{}'", opt as u8 as char);
      }
    }
  }
}
