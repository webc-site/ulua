use crate::{
  functions::ensure_stack::ensure_stack,
  macros::{api_check::api_check, setnilvalue::setnilvalue},
  records::lua_state::LuaState,
};

/// `lua_settop` 核心（cpp VM/src/lapi.cpp:267）。调用序契约（正确性，非内存安全）：
/// `idx≥0` 时须 `idx ≤ l.stack_last-l.base`（api_check），`ensure_stack` 先把可写界
/// 抬到 `l.ci.top` 内再对 `l.top..base+idx` 逐个 `setnilvalue`（写出不得越过栈数组），末置 `top=base+idx`；
/// `idx<0` 时须 `-(idx+1) ≤ top-base`（不得截到 base 之下）。`ensure_stack` 可触发 GC。
pub fn lua_settop(l: &mut LuaState, idx: i32) {
  unsafe {
    if idx >= 0 {
      api_check!(l, idx as isize <= l.stack_last.offset_from(l.base));
      // cpp `ensure_stack(L, idx - (L->top - L->base))`：把栈顶抬高到 idx 之
      // 前必须保证 idx 落在 ci->top 之内，否则下面的 setnilvalue 会写出栈数组。
      ensure_stack(l, idx - l.top.offset_from(l.base) as i32);
      // 栈窗口 top..target 补空：原 `while top < target { setnilvalue; top += 1 }`
      // 逐格走查收为一次预留槽窗填充；top 已越过 target 时切片取 0 长、
      // 直落 `reanchor_top(target)` 截断，与原循环空转分支逐指令等价
      let target = l.base.add(idx as usize);
      let fill = target.offset_from(l.top).max(0) as usize;
      // SAFETY: ensure_stack 已把可写界抬到覆盖 target（api_check 保证在 stack_last
      // 内），预留窗 [top, top+fill) 可独占写入——契约见 `reserved_slots_mut`
      for slot in l.reserved_slots_mut(fill) {
        setnilvalue!(slot);
      }
      l.reanchor_top(target);
    } else {
      api_check!(l, -(idx + 1) as isize <= l.top.offset_from(l.base));
      // 负 idx 截断：`top_slot(idx+1)` 读数 + 重锚提交两步，镜像原一步
      // 顶回退落值形（`subtract' index (index is negative)`）
      let target = l.top_slot((idx + 1) as isize);
      l.reanchor_top(target); // `subtract' index (index is negative)
    }
  }
}
