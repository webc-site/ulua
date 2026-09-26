use core::ffi::c_char;

use crate::{
  functions::{lua_pushinteger::lua_pushinteger, lua_pushlstring::lua_pushlstring},
  macros::{cap_position::CAP_POSITION, cap_unfinished::CAP_UNFINISHED, lua_l_error::luaL_error},
  records::match_state::MatchState,
};

/// cpp `lstrlib.cpp push_onecapture`：把第 `i` 个捕获压栈，偏移游标版。
/// `s`/`e` 为整窗匹配的源偏移对，`None` 即 cpp 传 `NULL` 哨兵
/// （`str_find_aux` 的 find 分支：`level == 0` 时不再压整窗）。
///
/// # Safety
/// `ms` 必须是 `prepstate` 建立、仍处本次匹配调用中的 `MatchState`；
/// 捕获槽 `init + len <= ms.src.len()` 界内；`s`/`e` 为同源串偏移且 `e >= s`。
pub(crate) unsafe fn push_onecapture(
  ms: &mut MatchState,
  i: i32,
  s: Option<usize>,
  e: Option<usize>,
) {
  // Safety: 契约保证捕获槽在 captures 数组界内；-1 表示未参与捕获路径，压串仅取源串界内切片
  unsafe {
    if i >= ms.level {
      if i == 0 {
        // cpp: lua_pushlstring(ms->l, s, e - s); —— C-API 边界从源串切片重建指针。
        // 到达此处时调用方（push_captures）已保证 s/e 非 NULL（否则 nlevels==0）
        if let (Some(s), Some(e)) = (s, e) {
          // Safety: src_slice 按契约（s <= e <= src.len()）返回界内切片，重建指针在
          // whole.len() 内有效、`c_char` 对齐为 1；lua_pushlstring 仅做界内拷贝不留存
          let whole = ms.src_slice(s, e - s);
          lua_pushlstring(ms.l, whole.as_ptr() as *const c_char, whole.len());
        }
      } else {
        luaL_error!(ms.l, "invalid capture index");
      }
    } else {
      let l = ms.capture[i as usize].len;
      if l == CAP_UNFINISHED as isize {
        luaL_error!(ms.l, "unfinished capture");
      } else if l == CAP_POSITION as isize {
        // cpp: lua_pushinteger(ms->l, (int)(ms->capture[i].init - ms->src_init) + 1);
        // —— 偏移化后 init 即指针差值
        lua_pushinteger(ms.l, ms.capture[i as usize].init as i32 + 1);
      } else {
        // cpp: lua_pushlstring(ms->l, ms->capture[i].init, l); —— 同上由源偏移切片重建
        // Safety: 捕获槽 init + len <= src.len() 界内（# Safety 契约），重建指针在
        // cap.len() 内有效、`c_char` 对齐为 1；lua_pushlstring 仅做界内拷贝不留存
        let cap = ms.src_slice(ms.capture[i as usize].init, l as usize);
        lua_pushlstring(ms.l, cap.as_ptr() as *const c_char, cap.len());
      }
    }
  }
}
