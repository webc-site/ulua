use crate::{
  functions::{lua_l_checkstack::lua_l_checkstack, push_onecapture::push_onecapture},
  records::match_state::MatchState,
};

/// cpp `lstrlib.cpp push_captures`：依序压出全部捕获（无捕获时压整窗），
/// 偏移游标版；`s`/`e` 的 `None` 即原 NULL 哨兵。
///
/// 前置条件（由 `prepstate` 建立的 `MatchState` 对象不变式承载）：`s`/`e` 为同源串偏移
/// （或 `None`），捕获位置均界内，压栈发生在存活调用帧当前 top。层级计数为纯整数运算，
/// 故签名安全；`unsafe` 仅剩栈扩容交互内核（w6e 后 `push_onecapture` 已降为安全 fn）。
pub(crate) fn push_captures(ms: &mut MatchState, s: Option<usize>, e: Option<usize>) -> i32 {
  // cpp: int nlevels = (ms->level == 0 && s) ? 1 : ms->level; —— s 非空判定即 Option 判定
  let nlevels = if ms.level == 0 && s.is_some() {
    1
  } else {
    ms.level
  };

  // SAFETY: 对象不变式（`prepstate` 建立的存活 `lua_State`）保证 `ms.l` 可承接栈扩容，
  // 容量不足时其内部经 lua_error 抛 "too many captures" 且不返回
  // SAFETY: `ms.l` 为 prepstate 建立的存活 `lua_State` 裸指针，本帧重建可变引用即结束借用窗口
  unsafe { lua_l_checkstack(&mut *ms.l, nlevels, "too many captures") };

  // 保留下标遍历：i 即捕获层级编号本身——push_onecapture 拿它比对 ms.level 并寻址
  // ms.capture[i]；且该调用可变借用 ms，无法同时持有捕获数组的切片借用
  for i in 0..nlevels {
    // w6e 降级消费点：`push_onecapture` 已降为安全 fn，包裹消亡；`ms.l` 存活帧与
    // 捕获槽界内不变式仍由 `prepstate`（unsafe 入口）建立并沿本次匹配全程持有
    push_onecapture(ms, i, s, e);
  }

  nlevels
}
