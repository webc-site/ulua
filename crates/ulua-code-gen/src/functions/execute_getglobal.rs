use ulua_common::macros::{
  luau_assert::LUAU_ASSERT, luau_insn_a::luau_insn_a, luau_insn_c::luau_insn_c,
};
use ulua_vm::type_aliases::{stk_id::StkId, t_value::TValue};

use crate::{
  records::vm_frame::VmFrame,
  type_aliases::{instruction_ir_builder::Instruction, lua_state::LuaState},
};

/// 生成码回写的 GETGLOBAL 慢路径解释器（cpp `executeGETGLOBAL`）。
///
/// # Safety
/// VM 回调 ABI 约定：`l` 为存活 `LuaState`（其 `ci.func` 为闭包、`global` 一致有效），
/// `pc` 指向本帧 code 内一条 GETGLOBAL 指令（主字 + AUX 常量字），`base`/`k` 为本帧
/// 活动栈基址与常量表基址。边界契约集中于 [`VmFrame::new`]，其余为安全逻辑。
pub unsafe fn execute_getglobal(
  l: *mut LuaState,
  pc: *const Instruction,
  base: StkId,
  k: *mut TValue,
) -> *const Instruction {
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

/// # Safety
/// C-ABI 导出边界：由生成码/VM 按 codegen 回调约定调用，`l`/`pc`/`base`/`k` 的合法性与
/// 存活性与 [`execute_getglobal`] 的契约一致（本函数仅原样透传）。
pub unsafe extern "C-unwind" fn execute_getglobal_export(
  l: *mut LuaState,
  pc: *const Instruction,
  base: StkId,
  k: *mut TValue,
) -> *const Instruction {
  // Safety: 导出 C ABI 入口原样转发 l/pc/base/k 给同契约 unsafe fn execute_getglobal; 由
  // 生成的原生代码/VM 保证参数指向活 LuaState/code/栈/常量, 满足被调前置条件。
  unsafe { execute_getglobal(l, pc, base, k) }
}
