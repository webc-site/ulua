use crate::{
  macros::{cap_unfinished::CAP_UNFINISHED, lua_l_error::luaL_error},
  records::match_state::MatchState,
};

/// cpp `lstrlib.cpp:200 check_capture`：解析捕获序号字符并校验对应槽已闭合。
///
/// `ms` 由 `prepstate` 建立（`capture[..ms.level]` 界内可访问、`ms.l` 为处于可抛错
/// 受保护帧的存活 `lua_State`），偏移/序号判定全走切片与整数值，故签名安全；
/// 唯一的 `unsafe` 收窄到下方抛错调用内核。`l` 为待解析的捕获序号字符。
pub(crate) fn check_capture(ms: &mut MatchState, mut l: i32) -> i32 {
  l -= '1' as i32;
  if l < 0 || l >= ms.level || ms.capture[l as usize].len == CAP_UNFINISHED as isize {
    // SAFETY: 对象不变式由 `prepstate`（unsafe 入口）建立——`ms.l` 为可抛错的存活
    // lua_State；cpp `check_capture` 的 lua 错误面文本不变，`luaL_error!` 走 format_args 通道
    unsafe { luaL_error!(&mut *ms.l, "invalid capture index %{}", l + 1) };
  }
  l
}
