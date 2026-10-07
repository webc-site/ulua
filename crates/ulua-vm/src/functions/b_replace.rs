use crate::{
  functions::{
    fieldargs::fieldargs, lua_l_checkunsigned::lua_l_checkunsigned,
    lua_pushunsigned::lua_pushunsigned,
  },
  macros::{lua_lib_fn::lua_lib_fn, mask::mask},
  records::lua_state::LuaState,
  type_aliases::b_uint::BUint,
};

/// replace 核心：取 1 号数与 2 号新值、经 `fieldargs` 校验字段后换段压回。
///
/// 调用序契约（正确性，非内存安全）：以 Lua 库函数约定被调——实参 1..=4 按 API 索引
/// 约定可读（越界或非数值由 check*/argerror 报错回退），字段位区间经校验落在 [0,32]
/// 界内，栈顶预留结果空间；mask 为纯标量位算。
pub(crate) fn b_replace(l: &mut LuaState) -> i32 {
  let r: BUint = lua_l_checkunsigned(l, 1);
  let mut v: BUint = lua_l_checkunsigned(l, 2);
  let (f, w) = fieldargs(l, 3);
  let m: BUint = mask(w);
  v &= m;
  let r = (r & !(m << f)) | (v << f);
  lua_pushunsigned(l, r);
  1
}

lua_lib_fn!(pub(crate) fn b_replace @ref, b_replace_arm);
