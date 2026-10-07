use ulua_common::macros::{luau_assert::LUAU_ASSERT, luau_insn_ops::luau_insn_a};
use ulua_vm::type_aliases::{stk_id::StkId, t_value::TValue};

use crate::{
  records::vm_frame::VmFrame,
  type_aliases::{api::LuaState, ir::Instruction},
};

/// 生成码回写的 PREPVARARGS 慢路径解释器（cpp `executePREPVARARGS`）。
///
/// # Safety
/// VM 回调 ABI 约定：`l` 为存活 `LuaState`，`pc` 指向本帧 code 内一条 PREPVARARGS
/// 指令（`base`/`k` 在扩栈前不使用）。边界契约集中于 [`VmFrame::new`]，其余为安全逻辑。
pub unsafe extern "C-unwind" fn execute_prepvarargs(
  l: *mut LuaState,
  pc: *const Instruction,
  // 传入的 base 在 protect 同步扩栈前不使用；扩栈后由下方 `let base` 遮蔽为重取的新栈基。
  base: StkId,
  _k: *mut TValue,
) -> *const Instruction {
  // Safety: 本函数头 ABI 契约保证 `l`/`base` 为存活 LuaState 与活动帧基址；VmFrame::new 仅收编地址对、不解引用。
  let mut frame = unsafe { VmFrame::new(l, base) };

  let cl = frame.current_closure();
  let insn = frame.insns(pc, 1)[0];
  let pc_ptr = frame.insn_offset(pc, 1);
  let numparams = luau_insn_a(insn) as i32;

  // 所有固定参数都拷贝到 top 之后，因此需要更多栈空间
  // condhardstacktests 展开（见 VmFrame::check_stack_grow），protect 后同步回写栈基。
  frame.protect_sync_base(pc_ptr, |frame| {
    let n = frame.closure_stacksize(cl) as i32 + numparams;
    frame.check_stack_grow(n);
  });
  let base = frame.base_addr();

  LUAU_ASSERT!(frame.slots_diff(frame.top(), base) >= numparams);

  // 把固定参数移动到最终位置
  let fixed = base; // first fixed argument
  let new_base = frame.top(); // final position of first argument

  if numparams > 0 {
    // 两切片视图取自同一活栈数组的不相交区间（[new_base, +n) 与 [fixed, +n)），
    // 逐槽 setobj_2_s + setnil 与改造前的手写 from_raw_parts_mut 循环地址序一致。
    let n = numparams as usize;
    let moved = frame.slots_mut(new_base, n);
    let fixed_slots = frame.slots_mut(fixed, n);
    for (dst, src) in moved.iter_mut().zip(fixed_slots) {
      frame.set_stack_value(dst, src);
      frame.set_nil(src);
    }
  }

  // 重接我们的栈帧，使其指向新的 base
  let stacksize = frame.closure_stacksize(cl);
  frame.rebind_frame(new_base, stacksize);

  pc_ptr
}
