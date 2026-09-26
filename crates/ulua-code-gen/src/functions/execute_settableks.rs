use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  macros::{
    luau_assert::LUAU_ASSERT,
    luau_insn_ops::{luau_insn_a, luau_insn_aux_kv16, luau_insn_b, luau_insn_c, luau_insn_op},
  },
};
use ulua_vm::{
  enums::tms::TMS,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

use crate::{
  records::vm_frame::VmFrame,
  type_aliases::{instruction_ir_builder::Instruction, lua_state::LuaState},
};

/// 生成码回写的 SETTABLEKS/SETUDATAKS 慢路径解释器（cpp `executeSETTABLEKS`）。
///
/// # Safety
/// VM 回调 ABI 约定：`l` 为存活 `LuaState`，`pc` 指向本帧 code 内一条 SETUDATAKS/
/// SETTABLEKS 指令（主字 + AUX 常量字），`base`/`k` 为本帧活动栈基址与常量表基址。
/// 边界契约集中于 [`VmFrame::new`]，其余为安全逻辑。
pub unsafe fn execute_settableks(
  l: *mut LuaState,
  pc: *const Instruction,
  base: StkId,
  k: *mut TValue,
) -> *const Instruction {
  let frame = unsafe { VmFrame::new(l, base) };

  let cl = frame.current_closure();
  let words = frame.insns(pc, 2);
  let (insn, aux) = (words[0], words[1]);
  // SETUDATAKS/SETTABLEKS 之后即下一条指令（patch 目标回主字 pc）。
  let pc_ptr = frame.insn_offset(pc, 2);
  let ra = frame.reg(luau_insn_a(insn) as i32);
  let rb = frame.reg(luau_insn_b(insn) as i32);
  // SETUDATAKS 的常量号在 AUX 字低 16 位, 其余走整 AUX 字。
  let kw = if luau_insn_op(insn) == LuauOpcode::LOP_SETUDATAKS as u32 {
    luau_insn_aux_kv16(aux)
  } else {
    aux
  };
  let kv = frame.kv(kw, cl, k);
  LUAU_ASSERT!(frame.is_string(kv));

  if frame.is_table(rb) {
    let h = frame.hvalue(rb);

    let direct_set = frame.fast_not_meta(frame.table_metatable(h), TMS::TmNewIndex)
      && frame.table_readonly(h) == 0;
    if direct_set {
      // 快路径: 无 __newindex 且非只读, 直写槽位并回填 patch。
      frame.save_pc(pc_ptr);
      let res = frame.set_str(h, frame.tsvalue(kv) as *mut _);
      frame.patch_c(pc, frame.value_to_slot(h, res));
      frame.set_table_value(res, ra);
      frame.barrier_table(h, ra);
      return pc_ptr;
    }

    // 慢路径: 可能经 __newindex 元方法, cachedslot 记掩码槽号供 patch。
    let slot = (luau_insn_c(insn) as i32) & frame.table_nodemask8(h);
    frame.set_cachedslot(slot);
    frame.protect(pc_ptr, |frame| {
      frame.settable(rb, kv, ra);
    });
    frame.patch_c(pc, frame.cachedslot());
    return pc_ptr;
  }

  // 快路径: userdata 且注册了 C __newindex 元方法(原条件短路链逐序等价展开)。
  if frame.is_userdata(rb) {
    let fn_tm = frame
      .meta_method(frame.udata_metatable(frame.uvalue(rb)), TMS::TmNewIndex)
      .filter(|tm| frame.is_function(*tm) && frame.closure_is_c(frame.clvalue(*tm)));
    if let Some(fn_tm) = fn_tm {
      frame.assert_top_fits(4);
      let top = frame.top();
      frame.set_stack_value(frame.slot_at(top, 0), fn_tm);
      frame.set_stack_value(frame.slot_at(top, 1), rb);
      frame.set_stack_value(frame.slot_at(top, 2), kv);
      frame.set_stack_value(frame.slot_at(top, 3), ra);
      frame.set_top(frame.slot_at(top, 4));

      frame.set_cachedslot(luau_insn_c(insn) as i32);
      frame.protect(pc_ptr, |frame| {
        frame.call_tm(3, -1);
      });
      frame.patch_c(pc, frame.cachedslot());
      return pc_ptr;
    }
  }

  // 慢路径: 通用 settable, 可能经 __newindex 元方法调用 Lua
  frame.protect(pc_ptr, |frame| {
    frame.settable(rb, kv, ra);
  });
  pc_ptr
}

/// # Safety
/// C-ABI 导出边界：由生成码/VM 按 codegen 回调约定调用，`l`/`pc`/`base`/`k` 的合法性与
/// 存活性与 [`execute_settableks`] 的契约一致（本函数仅原样透传）。
pub unsafe extern "C-unwind" fn execute_settableks_export(
  l: *mut LuaState,
  pc: *const Instruction,
  base: StkId,
  k: *mut TValue,
) -> *const Instruction {
  // Safety: 导出 C ABI 入口原样转发 l/pc/base/k 给同契约 unsafe fn execute_settableks;
  // 由 VM/原生代码保证参数指向活 LuaState/code/栈/常量, 满足被调前置条件。
  unsafe { execute_settableks(l, pc, base, k) }
}
