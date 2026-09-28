use ulua_vm::records::lua_state::LuaState;

use crate::records::vm_frame::VmFrame;

/// 生成码回写的 FORGLOOP 非表迭代回退（cpp `forgLoopNonTableFallback`）。
///
/// # Safety
/// VM 回调 ABI 约定：`l` 为存活 `LuaState`（FORGLOOP 迭代器帧在 `ra..ra+6` 预留参数
/// 槽），`insn_a`/`aux` 为指令 A 字段与 aux 常量（nresults）。边界契约集中于
/// [`VmFrame::current`]，其余为安全逻辑。
pub unsafe fn forg_loop_non_table_fallback(l: *mut LuaState, insn_a: i32, aux: i32) -> i32 {
  let mut frame = unsafe { VmFrame::current(l) };

  let ra = frame.reg(insn_a);
  // 注：出于复杂原因，把参数压到 top 之外是安全的（见 lvmexecute.cpp）
  let callee = frame.slot_at(ra, 3);
  frame.set_stack_value(frame.slot_at(callee, 2), frame.slot_at(ra, 2));
  frame.set_stack_value(frame.slot_at(callee, 1), frame.slot_at(ra, 1));
  frame.set_stack_value(callee, ra);
  frame.set_top(frame.slot_at(callee, 3)); // func + 2 args (state and index)
  frame.assert_top_reserved();

  // yield/break 以 -1 通知调用方退出原生执行
  if frame.perform_call(callee, aux as u8 as i32) {
    return -1;
  }
  frame.set_top(frame.ci_top());

  // 栈可能已重分配：同步新栈基后重取 ra
  frame.sync_base();
  let ra = frame.reg(insn_a);

  // 把第一个变量拷回迭代索引
  frame.set_stack_value(frame.slot_at(ra, 2), frame.slot_at(ra, 3));

  // 有下一元素→1，迭代结束→0（cpp 返回 int 判别，惯用 i32::from 折叠）
  i32::from(!frame.is_nil(frame.slot_at(ra, 3)))
}
/// # Safety
/// C-ABI 导出边界：由生成码按 codegen 回调约定调用，`l`/`insn_a`/`aux` 的合法性与
/// 存活性与 [`forg_loop_non_table_fallback`] 的契约一致（本函数仅原样透传）。
pub unsafe extern "C-unwind" fn forg_loop_non_table_fallback_export(
  l: *mut LuaState,
  insn_a: i32,
  aux: i32,
) -> i32 {
  // Safety: 导出 C ABI 入口原样转发 l/insn_a/aux 给同契约 unsafe fn forg_loop_non_table_fallback;
  // 调用方按 ABI 保证 l 为活 LuaState, 满足被调前置条件。
  unsafe { forg_loop_non_table_fallback(l, insn_a, aux) }
}
