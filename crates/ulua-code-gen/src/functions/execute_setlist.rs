use core::ptr::null;

use ulua_common::macros::luau_insn_ops::{luau_insn_a, luau_insn_b, luau_insn_c};
use ulua_vm::{
  macros::lua_multret::LUA_MULTRET,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

use crate::{
  records::vm_frame::VmFrame,
  type_aliases::{instruction_ir_builder::Instruction, lua_state::LuaState},
};

/// 生成码回写的 SETLIST 慢路径解释器（cpp `executeSETLIST`）。
///
/// # Safety
/// VM 回调 ABI 约定：`l` 为存活 `LuaState`（`ci`/`top` 有效），`pc` 指向本帧 code 内
/// 一条 SETLIST 指令（其后随 index AUX 字），`base` 为活动栈帧基址、`k` 为常量表基址
/// （本指令不使用）。慢路径允许重入 VM，参数指针须比本次调用存活。
pub unsafe fn execute_setlist(
  l: *mut LuaState,
  pc: *const Instruction,
  base: StkId,
  _k: *mut TValue,
) -> *const Instruction {
  // 边界契约集中于 VmFrame::new（见 records::vm_frame）；以下为纯安全业务逻辑。
  let frame = unsafe { VmFrame::new(l, base) };
  // cpp CodeGenUtils.cpp:772 `[[maybe_unused]] Closure* cl = clvalue(L->ci->func);`
  // 保留读取以对齐 cpp 指令序，函数体内确实不使用
  let _cl = frame.current_closure();

  // SETLIST 占两字：主字 + index AUX 字；pc_ptr 为下一条指令。
  let words = frame.insns(pc, 3);
  let (insn, index) = (words[0], words[1]);
  // 第三条字即下一条指令（SETLIST 占两字）。
  let pc_ptr: *const Instruction = &words[2];

  let ra = frame.reg(luau_insn_a(insn) as i32);
  // 注：当 c == LUA_MULTRET 时它可能指向 l->top，此时用 VM_REG 不安全
  let rb = frame.slot_at(base, luau_insn_b(insn) as usize);
  let mut c = (luau_insn_c(insn) as i32) - 1;

  if c == LUA_MULTRET {
    c = frame.reg_index(frame.top()) - frame.reg_index(rb);
    frame.set_top(frame.ci_top());
  }

  let h = frame.hvalue(ra);

  // TODO: 现在已经不需要它了
  if !frame.is_table(ra) {
    return null(); // temporary workaround to weaken a rather powerful exploitation primitive in case of a MITM attack on bytecode
  }

  let last = index as i32 + c - 1;
  if last > frame.array_len(h) {
    frame.save_pc(pc_ptr); // luaH_resizearray may fail due to OOM
    frame.resize_array(h, last);
  }

  // resize 后经 array_slot 重取序列数组基址；dst/src 两切片分别落在
  // 已扩容数组 [index-1, index-1+c) 与栈 [rb, rb+c) 内（c 由 index/top 差算得）。
  if c > 0 {
    let dst = frame.slots_mut(frame.array_slot(h, (index as i32 - 1) as usize), c as usize);
    let src = frame.slots(rb, c as usize);
    for (dst, src) in dst.iter_mut().zip(src) {
      frame.set_table_value(dst, src);
    }
  }

  frame.barrier_fast(h);
  pc_ptr
}

/// # Safety
/// C-ABI 导出边界：由生成码/VM 按 Lua codegen 回调约定调用，`l`/`pc`/`base`/`k`
/// 的存活性与界内性与 [`execute_setlist`] 的契约一致（本函数仅原样透传）。
pub unsafe extern "C-unwind" fn execute_setlist_export(
  l: *mut LuaState,
  pc: *const Instruction,
  base: StkId,
  k: *mut TValue,
) -> *const Instruction {
  // Safety: 导出 C ABI 入口原样转发 l/pc/base/k 给同契约 unsafe fn execute_setlist;
  // 调用方按 ABI 提供活 LuaState 及帧内 code/栈指针, 满足被调前置条件。
  unsafe { execute_setlist(l, pc, base, k) }
}
