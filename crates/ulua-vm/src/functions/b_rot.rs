use crate::{
  functions::{lua_l_checkunsigned::lua_l_checkunsigned, lua_pushunsigned::lua_pushunsigned},
  macros::{nbits::NBITS, trim::trim},
  records::lua_state::LuaState,
  type_aliases::b_uint::BUint,
};

/// # Safety
///
/// `l` 必须指向本次 binary32 C 函数调用的存活 `LuaState`：所需实参按 API 索引约定位于栈上可读（越界或非数值由 check*/argerror 报错），栈顶预留结果空间。
pub(crate) unsafe fn b_rot(l: *mut LuaState, mut i: i32) -> i32 {
  // Safety: 契约保证 `l` 为存活调用帧且实参 2 可读，旋转量归一后纯数值运算无指针访问
  unsafe {
    let mut r: BUint = lua_l_checkunsigned(l, 1);

    // i = i % NBITS (avoid undefined shift when i == 0)
    i &= NBITS - 1;

    r = trim(r);
    if i != 0 {
      let i_u = i as u32;
      r = (r << i_u) | (r >> (NBITS as u32 - i_u));
    }

    lua_pushunsigned(l, trim(r));
    1
  }
}
