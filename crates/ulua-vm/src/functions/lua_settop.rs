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
      // r16-b2 收编：顶-基槽距读数落既有 get_top 门面——其本体 slot_distance(base, top)
      // 即被替代式 `top.offset_from(base) as i32` 的同址同宽镜像（ensure_stack 实参位
      // 现读、时序不变）；isize→i32 折形在现域无截差（栈槽距受 LUAI_MAXSTACK 约束）、
      // `idx -` 次序保持
      ensure_stack(l, idx - l.get_top());
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
      // r16-b2 收编：顶-基槽距读数落既有 get_top 门面（镜像论证见上方 :17 点位注）。
      // api_check! 系 debug 期断言、release 编译掉 ⇒ 换形等价平凡真（r14 p1 判例②）；
      // `-(idx + 1)` 本为 i32 求值形不变，isize 式与 i32 式在现域逐值同真值、方向不变
      api_check!(l, -(idx + 1) <= l.get_top());
      // 负 idx 截断：`top_slot(idx+1)` 读数 + 重锚提交两步，镜像原一步
      // 顶回退落值形（`subtract' index (index is negative)`）
      let target = l.top_slot((idx + 1) as isize);
      l.reanchor_top(target); // `subtract' index (index is negative)
    }
  }
}
