use ulua_vm::type_aliases::stk_id::StkId;

use crate::{
  records::vm_frame::VmFrame,
  type_aliases::{instruction_ir_builder::Instruction, lua_state::LuaState},
};

/// 生成码回写的 GETVARARGS（MultRet）慢路径解释器（cpp `executeGETVARARGS` 多返回分支）。
///
/// # Safety
/// VM 回调 ABI 约定：`l` 为存活 `LuaState`（当前被调为 L 闭包），`pc` 指向本帧 code 内
/// 当前指令（供 savedpc 记录），`base` 为本帧活动栈基址，`rai` 为指令编码的目标寄存器号。
/// 边界契约集中于 [`VmFrame::new`]，其余为安全逻辑。
pub unsafe fn execute_getvarargs_mult_ret(
  l: *mut LuaState,
  pc: *const Instruction,
  base: StkId,
  rai: i32,
) {
  let mut frame = unsafe { VmFrame::new(l, base) };

  let n = frame.varargs_count();

  // 扩栈（condhardstacktests 展开，见 VmFrame::check_stack_grow），protect 后同步回写栈基。
  frame.protect_sync_base(pc, |frame| frame.check_stack_grow(n));

  // 栈可能已重分配，经 reg 重取 ra
  let ra = frame.reg(rai);
  let count = n.max(0) as usize;
  let src = frame.slots(frame.slot_back(frame.base_addr(), count), count);
  let dst = frame.slots_mut(ra, count);
  // setobj_2_s 逐元素拷贝并带 checkliveness，不可合并为整段复制
  for (d, s) in dst.iter_mut().zip(src) {
    frame.set_stack_value(d, s);
  }

  frame.set_top(frame.slot_at(ra, count));
}

/// # Safety
/// C-ABI 导出边界：由生成码/VM 按 codegen 回调约定调用，`l`/`pc`/`base`/`rai` 的合法性与
/// 存活性与 [`execute_getvarargs_mult_ret`] 的契约一致（本函数仅原样透传）。
pub unsafe extern "C-unwind" fn execute_getvarargsmult_ret(
  l: *mut LuaState,
  pc: *const Instruction,
  base: StkId,
  rai: i32,
) {
  // Safety: 导出 C ABI 入口原样转发 l/pc/base/rai 给同契约 unsafe fn execute_getvarargs_mult_ret;
  // 调用方按 ABI 提供活 LuaState 及帧内 code/栈指针, 满足被调前置条件。
  unsafe {
    execute_getvarargs_mult_ret(l, pc, base, rai);
  }
}
