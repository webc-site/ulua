use crate::{
  enums::k_option::KOption,
  functions::{getnum::FmtCursor, getoption::getoption},
  macros::lua_l_argerror::luaL_argerror,
  records::header::Header,
};

/// cpp `lstrlib.cpp:getdetails`：解析一个格式选项并算出对齐填充。
///
/// cpp 经 `int* psize, int* ntoalign` 写出参，Rust 版返回 `(选项, 尺寸, 对齐填充)`。
///
/// # Safety
/// `h.l` 为存活 `lua_State`；`fmt` 游标位于格式串界内（'X' 后视读取越尾归一为 NUL）。
pub(crate) unsafe fn getdetails(
  h: &mut Header,
  totalsize: usize,
  fmt: &mut FmtCursor,
) -> (KOption, i32, i32) {
  unsafe {
    let (size, opt) = getoption(h, fmt);
    let mut align = size; // usually, alignment follows size

    if opt == KOption::Kpaddalign {
      // 'X' 的对齐取自后一个选项；cpp 的 `||` 短路（'\0' 时不再前读）需原样保留
      let mut invalid = fmt.cur() == 0;
      if !invalid {
        let (next, next_opt) = getoption(h, fmt);
        align = next;
        invalid = next_opt == KOption::Kchar || align == 0;
      }
      if invalid {
        luaL_argerror!(h.l, 1, "invalid next option for option 'X'");
      }
    }

    let ntoalign = if align <= 1 || opt == KOption::Kchar {
      0 // 无需对齐
    } else {
      if align > h.maxalign {
        align = h.maxalign; // 受最大对齐约束
      }
      if (align & (align - 1)) != 0 {
        luaL_argerror!(h.l, 1, "format asks for alignment not power of 2");
      }
      (align - (totalsize as i32 & (align - 1))) & (align - 1)
    };

    (opt, size, ntoalign)
  }
}
