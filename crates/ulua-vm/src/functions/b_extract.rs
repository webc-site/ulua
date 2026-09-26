use crate::{
  functions::{
    fieldargs::fieldargs, lua_l_checkunsigned::lua_l_checkunsigned,
    lua_pushunsigned::lua_pushunsigned,
  },
  macros::mask::mask,
  records::lua_state::LuaState,
  type_aliases::b_uint::BUint,
};

/// # Safety
///
/// `l` 必须指向本次 binary32 C 函数调用的存活 `LuaState`：所需实参按 API 索引约定位于栈上可读（越界或非数值由 check*/argerror 报错），栈顶预留结果空间。
pub(crate) unsafe extern "C-unwind" fn b_extract(l: *mut LuaState) -> i32 {
  // Safety: 契约保证 `l` 为存活调用帧且实参 1..=3 可读，字段位区间经 argcheck 在 [0,64) 界内
  unsafe {
    let r: BUint = lua_l_checkunsigned(l, 1);
    let (f, w) = fieldargs(l, 2);
    let r = (r >> f) & mask(w);
    lua_pushunsigned(l, r);
    1
  }
}
