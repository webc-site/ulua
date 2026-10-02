use crate::{
  functions::lua_pushlstring::lua_pushlstring_bytes,
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
  // SAFETY: 契约保证捕获槽在 captures 数组界内；-1 表示未参与捕获路径，压串仅取源串界内切片
  unsafe {
    if i >= ms.level {
      if i == 0 {
        // cpp: lua_pushlstring(ms->l, s, e - s); —— Rust 侧直投切片 ref 核心，
        // 不再经 C-API ptr+len 形重建指针
        if let (Some(s), Some(e)) = (s, e) {
          // SAFETY: src_slice 按契约（s <= e <= src.len()）返回界内切片；
          // lua_pushlstring_bytes 仅界内拷入堆串、不留存借用
          let whole = ms.src_slice(s, e - s);
          lua_pushlstring_bytes(&mut *ms.l, whole);
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
        (*ms.l).push_integer(ms.capture[i as usize].init as i32 + 1);
      } else {
        // cpp: lua_pushlstring(ms->l, ms->capture[i].init, l); —— 同上直投切片 ref 核心
        // SAFETY: 捕获槽 init + len <= src.len() 界内（# Safety 契约）；
        // lua_pushlstring_bytes 仅界内拷入堆串、不留存借用
        let cap = ms.src_slice(ms.capture[i as usize].init, l as usize);
        lua_pushlstring_bytes(&mut *ms.l, cap);
      }
    }
  }
}
