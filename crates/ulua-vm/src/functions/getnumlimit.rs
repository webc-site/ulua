use crate::{
  functions::getnum::{FmtCursor, getnum},
  macros::lua_l_error::luaL_error,
  records::header::Header,
};

/// 整数尺寸字段合法上限（cpp `SZINT` = 16 字节）
const MAX_INT_SIZE: i32 = 16;

/// # Safety
///
/// `h.l` 为存活 `lua_State`；`fmt` 游标位于格式串界内（越尾归一为 NUL）。
pub(crate) unsafe fn getnumlimit(h: &mut Header, fmt: &mut FmtCursor, df: i32) -> i32 {
  unsafe {
    let sz = getnum(h, fmt, df);
    if sz > MAX_INT_SIZE || sz <= 0 {
      luaL_error!(
        h.l,
        "integral size ({}) out of limits [1,{}]",
        sz,
        MAX_INT_SIZE,
      );
    }
    sz
  }
}
