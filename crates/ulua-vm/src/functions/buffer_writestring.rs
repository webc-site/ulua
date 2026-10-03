use crate::{
  enums::lua_type::LuaType,
  functions::{
    buffer_window::{buffer_at_ref, buffer_data_ref},
    lua_l_optinteger::lua_l_optinteger,
  },
  macros::{lua_l_error::luaL_error, lua_lib_fn::lua_lib_fn},
  records::lua_state::LuaState,
};

/// cpp `buffer_writestring`（lbuflib.cpp:216）：把字符串实参（或其前 `count` 字节）
/// 写进 buffer 的 `[offset, offset+count)` 窗口。
///
/// 抛错序逐位保持：#1 typeerror → #2 checkinteger → #3 checklstring → #4 optinteger
/// → argcheck(count≥0) → "string length overflow" → oob。#1 的 typeerror 用
/// `check_type`（只判型不派窗，`tag_error(LUA_TBUFFER)` 与 cpp `luaL_checkbuffer`
/// 失败路径同消息同点位）先行落下，数据窗 `buffer_data_ref` 后置派生到校验通过后、
/// 临近写入处。
///
/// 调用序契约（正确性，非内存安全）：`l` 存活与独占由 `&mut LuaState` 承载且处于本 C
/// 函数受保护帧：栈 1 号为 buffer、2 号偏移、3 号字符串、4 号可选 count。重入面论证：
/// 本函数全部取参均不执行 Lua 代码——`check_integer`/`lua_l_optinteger` 走
/// `lua_tointegerx`→`tonumber` 内联（仅 Number 直取与 String 的 `luaO_str2d` 纯解析，
/// cpp lvm.h:10/lapi.cpp:432，无元方法），`check_bytes` 走 `lua_tolstring`→`luaV_tostring`
/// （cpp lvmutils.cpp:39 仅 Number→String 内置格式化，不调 `__tostring`）；后置派生是
/// 防御性收紧，杜绝 `&mut [u8]` 窗借用在任何取参/校验抛错之前存活。窗口经 `buffer_at_ref`
/// 界校验，`copy_from_slice` 写入区间必落在 buffer 数据界内且与源串等长（count ≤ size）。
/// 内存安全面收口在下方 "string length overflow" 的 `luaL_error` 窄块（真实抛错边界）。
pub(crate) fn buffer_writestring(l: &mut LuaState) -> i32 {
  // 只判型不派窗：#1 非 buffer 即抛 "buffer expected"，保持 typeerror 先于后续取参
  l.check_type(1, LuaType::Buffer);

  let offset = l.check_integer(2);
  let val = l.check_bytes(3);
  let count = lua_l_optinteger(l, 4, val.len() as i32);

  l.arg_check(count >= 0, 4, "count");

  if count as usize > val.len() {
    // SAFETY: `l.as_mut_ptr()` 为借用重建的存活调用帧裸参（有效与独占由 &mut 承载），
    // `luaL_error` 抛错不返回；受保护帧前提属调用序契约（见函数文档）。
    unsafe { luaL_error!(l.as_mut_ptr(), "string length overflow") };
  }

  // 后置派生：全部取参/校验落定后才借出数据窗，窗直达写入点；count ≤ size 已由
  // 上方 overflow 校验保证，源切片视图 `val[..count]` 必可读
  let buf = buffer_data_ref(l, 1);
  let dst = buffer_at_ref(l, buf, offset, count as usize);
  dst.copy_from_slice(&val[..count as usize]);

  0
}

lua_lib_fn!(pub(crate) fn buffer_writestring @ref, buffer_writestring_arm);
