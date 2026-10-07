use ulua_common::macros::{
  luau_assert::LUAU_ASSERT,
  luau_insn_ops::{luau_insn_a, luau_insn_c},
};
use ulua_vm::type_aliases::{stk_id::StkId, t_value::TValue};

use crate::{
  records::vm_frame::VmFrame,
  type_aliases::{api::LuaState, ir::Instruction},
};

/// 生成码回写的 GETGLOBAL 慢路径解释器（cpp `executeGETGLOBAL`）。
///
/// # Safety
/// VM 回调 ABI 约定：`l` 为存活 `LuaState`（其 `ci.func` 为闭包、`global` 一致有效），
/// `pc` 指向本帧 code 内一条 GETGLOBAL 指令（主字 + AUX 常量字），`base`/`k` 为本帧
/// 活动栈基址与常量表基址。边界契约集中于 [`VmFrame::new`]，其余为安全逻辑。
pub unsafe extern "C-unwind" fn execute_getglobal(
  l: *mut LuaState,
  pc: *const Instruction,
  base: StkId,
  k: *mut TValue,
) -> *const Instruction {
  // Safety: 本函数头 ABI 契约保证 `l`/`base` 为存活 LuaState 与活动帧基址；VmFrame::new 仅收编地址对、不解引用。
  let frame = unsafe { VmFrame::new(l, base) };

  let cl = frame.current_closure();
  // 本指令占两字：主字在 pc、AUX 常量字在 pc+1；pc_ptr 为下一条指令。
  let words = frame.insns(pc, 2);
  let (insn, aux) = (words[0], words[1]);
  let pc_ptr = frame.insn_offset(pc, 2);

  let ra = frame.reg(luau_insn_a(insn) as i32);
  let kv = frame.const_slot(k, aux);
  LUAU_ASSERT!(frame.is_string(kv));

  // fast-path 应已在别处检查过，这里跳过
  let h = frame.closure_env(cl);
  let slot = (luau_insn_c(insn) as i32) & frame.table_nodemask8(h);

  // slow-path，可能经 __index 元方法调用 Lua
  let mut g = TValue::default();
  frame.set_table(&mut g, h);
  frame.set_cachedslot(slot);
  frame.protect(pc_ptr, |frame| {
    let g: *const TValue = &g;
    frame.gettable(g, kv, ra);
  });

  // 写回 cachedslot 以加速后续查找；会 patch 当前执行的指令，因为 pc-2 回退了两条 pc++
  frame.patch_c(pc, frame.cachedslot());

  pc_ptr
}
