use core::ffi::{c_char, c_int};

use crate::{
  enums::k_option::KOption, functions::getoption::getoption, macros::lua_l_argerror::luaL_argerror,
  records::header::Header,
};

/// cpp `lstrlib.cpp:getdetails`：解析一个格式选项并算出对齐填充。
///
/// cpp 经 `int* psize, int* ntoalign` 写出参，Rust 版返回 `(选项, 尺寸, 对齐填充)`。
///
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn getdetails(
  h: *mut Header,
  totalsize: usize,
  fmt: *mut *const c_char,
) -> (KOption, c_int, c_int) {
  unsafe {
    let (size, opt) = getoption(h, fmt);
    let mut align = size; // usually, alignment follows size

    if opt == KOption::Kpaddalign {
      // 'X' 的对齐取自后一个选项；cpp 的 `||` 短路（'\0' 时不再前读）需原样保留
      let mut invalid = **fmt == b'\0' as c_char;
      if !invalid {
        let (next, next_opt) = getoption(h, fmt);
        align = next;
        invalid = next_opt == KOption::Kchar || align == 0;
      }
      if invalid {
        luaL_argerror!((*h).l, 1, "invalid next option for option 'X'");
      }
    }

    let ntoalign = if align <= 1 || opt == KOption::Kchar {
      0 // 无需对齐
    } else {
      if align > (*h).maxalign {
        align = (*h).maxalign; // 受最大对齐约束
      }
      if (align & (align - 1)) != 0 {
        luaL_argerror!((*h).l, 1, "format asks for alignment not power of 2");
      }
      (align - (totalsize as c_int & (align - 1))) & (align - 1)
    };

    (opt, size, ntoalign)
  }
}
