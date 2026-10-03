use crate::{
  functions::{
    buffer_errors::buffer_oob_error,
    buffer_window::{buffer_at_ref, buffer_data_ref},
    lua_l_checkunsigned::lua_l_checkunsigned,
    lua_l_optinteger::lua_l_optinteger,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全）：`l` 存活与独占由 `&mut LuaState` 承载；索引 1 为
/// buffer、2 为偏移、3 为填充值、4 为可选长度，填充区间经 [`buffer_at_ref`] 界校验落在
/// 数据界内，`fill` 仅触达该窗。
pub(crate) fn buffer_fill(l: &mut LuaState) -> i32 {
  let buf = buffer_data_ref(l, 1);
  let offset = l.check_integer(2);
  let value = lua_l_checkunsigned(l, 3);
  // C++ evaluates `int(len) - offset` as the default eagerly (signed overflow
  // is UB upstream for offset = INT_MIN); wrapping_sub reproduces the two's-
  // complement value C++ relies on, which the `size < 0` check and buffer_at_ref's
  // isoutofbounds gate below then reject. (Upstream UBSan: lbuflib.cpp:278.)
  let size = lua_l_optinteger(l, 4, (buf.len() as i32).wrapping_sub(offset));

  if size < 0 {
    buffer_oob_error(l);
  }

  buffer_at_ref(l, buf, offset, size as usize).fill((value & 0xff) as u8);

  0
}

lua_lib_fn!(pub(crate) fn buffer_fill @ref, buffer_fill_arm);
