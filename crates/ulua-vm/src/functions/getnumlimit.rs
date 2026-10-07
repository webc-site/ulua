use crate::{
  functions::getnum::{FmtCursor, getnum},
  macros::lua_l_error::luaL_error,
  records::header::Header,
};

/// 整数尺寸字段合法上限（cpp `SZINT` = 16 字节）
const MAX_INT_SIZE: i32 = 16;

/// 前置条件（由 `str_format` 建立的 `Header` 对象不变式承载）：`h.l` 为存活
/// `lua_State`；`fmt` 游标位于格式串界内（越尾归一为 NUL）。尺寸上下界判定为纯整数
/// 比较，故签名安全；`unsafe` 只留在下方报错内核。
pub(crate) fn getnumlimit(h: &mut Header, fmt: &mut FmtCursor, df: i32) -> i32 {
  let sz = getnum(h, fmt, df);
  if sz > MAX_INT_SIZE || sz <= 0 {
    // SAFETY: 前置条件保证 `h.l` 为可抛错的存活 `lua_State`，报错后不返回
    unsafe {
      luaL_error!(
        &mut *h.l,
        "integral size ({}) out of limits [1,{}]",
        sz,
        MAX_INT_SIZE,
      )
    };
  }
  sz
}
