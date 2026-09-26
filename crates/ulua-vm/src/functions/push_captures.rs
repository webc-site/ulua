use crate::{
  functions::{lua_l_checkstack::lua_l_checkstack, push_onecapture::push_onecapture},
  records::match_state::MatchState,
};

/// cpp `lstrlib.cpp push_captures`：依序压出全部捕获（无捕获时压整窗），
/// 偏移游标版；`s`/`e` 的 `None` 即原 NULL 哨兵。
///
/// # Safety
/// `ms` 必须是 `prepstate` 建立、仍处本次匹配调用中的 `MatchState`；
/// `s`/`e` 为同源串偏移（或 `None`），捕获位置均界内，压栈发生在存活调用帧当前 top。
pub(crate) unsafe fn push_captures(ms: &mut MatchState, s: Option<usize>, e: Option<usize>) -> i32 {
  // Safety: 契约保证捕获偏移均在源串界内，压栈发生在存活调用帧
  unsafe {
    // cpp: int nlevels = (ms->level == 0 && s) ? 1 : ms->level; —— s 非空判定即 Option 判定
    let nlevels = if ms.level == 0 && s.is_some() {
      1
    } else {
      ms.level
    };

    lua_l_checkstack(ms.l, nlevels, "too many captures");

    // 保留下标遍历：i 即捕获层级编号本身——push_onecapture 拿它比对 ms.level 并寻址
    // ms.capture[i]；且该调用可变借用 ms，无法同时持有捕获数组的切片借用
    for i in 0..nlevels {
      push_onecapture(ms, i, s, e);
    }

    nlevels
  }
}
