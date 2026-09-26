use ulua_common::{
  enums::{luau_capture_type::LuauCaptureType, luau_opcode::LuauOpcode},
  macros::{
    luau_assert::LUAU_ASSERT,
    luau_insn_ops::{luau_insn_a, luau_insn_b, luau_insn_d, luau_insn_op},
  },
};
use ulua_vm::type_aliases::{stk_id::StkId, t_value::TValue};

use crate::{
  records::vm_frame::VmFrame,
  type_aliases::{instruction_ir_builder::Instruction, lua_state::LuaState},
};

/// 生成码回写的 DUPCLOSURE 慢路径解释器（cpp `executeDUPCLOSURE`）。
///
/// # Safety
/// VM 回调 ABI 约定：`l` 为存活 `LuaState`，`pc` 指向本帧 code 内一条 DUPCLOSURE 指令
/// （其后随 `kcl.nupvalues` 条 CAPTURE 字），`base`/`k` 为本帧活动栈基址与常量表基址。
/// 边界契约集中于 [`VmFrame::new`]，其余为安全逻辑。
pub unsafe fn execute_dupclosure(
  l: *mut LuaState,
  pc: *const Instruction,
  base: StkId,
  k: *mut TValue,
) -> *const Instruction {
  let frame = unsafe { VmFrame::new(l, base) };

  let cl = frame.current_closure();
  let insn = frame.insns(pc, 1)[0];
  let pc_ptr = frame.insn_offset(pc, 1);
  let ra = frame.reg_a(insn);
  let kcl = frame.clvalue(frame.const_slot(k, luau_insn_d(insn) as u32));
  frame.save_pc(pc_ptr);

  // 捕获字流：nupvalues 条 CAPTURE 指令（字节码结构保证界内，切片视图免逐字 unsafe）。
  let nup = frame.closure_nupvalues(kcl);

  // env 与来源闭包一致则直接复用 kcl, 否则按 kcl 原型新建闭包(cpp DUPCLOSURE 主路径)。
  let same_env = frame.closure_env(kcl) == frame.closure_env(cl);
  let mut ncl = if same_env {
    kcl
  } else {
    frame.new_lclosure(nup as i32, frame.closure_env(cl), frame.closure_proto(kcl))
  };
  frame.set_closure_value(ra, ncl);

  let mut ui: i32 = 0;
  while ui < nup as i32 {
    let uinsn = frame.insns(pc_ptr, nup as usize)[ui as usize];
    // 捕获字必为 VAL/UPVAL 两类 CAPTURE(cpp 同式健全性断言)。
    LUAU_ASSERT!(luau_insn_op(uinsn) == LuauOpcode::LOP_CAPTURE as u32);
    LUAU_ASSERT!(
      luau_insn_a(uinsn) == LuauCaptureType::LCT_VAL as u32
        || luau_insn_a(uinsn) == LuauCaptureType::LCT_UPVAL as u32
    );

    let uv: *mut TValue = if luau_insn_a(uinsn) == LuauCaptureType::LCT_VAL as u32 {
      frame.reg(luau_insn_b(uinsn) as i32)
    } else {
      frame.closure_upvalue(cl, luau_insn_b(uinsn) as usize)
    };

    let uref = frame.closure_upvalue(ncl, ui as usize);

    if ncl == kcl && frame.raw_equal(uref, uv) {
      ui += 1;
      continue;
    }

    // preload 旗标读数; 命中则按新 env 重建闭包并回写 ra, 重启捕获遍历。
    if ncl == kcl && frame.closure_preload(kcl) == 0 {
      ncl = frame.new_lclosure(nup as i32, frame.closure_env(cl), frame.closure_proto(kcl));
      frame.set_closure_value(ra, ncl);

      ui = 0;
      continue;
    }

    frame.copy_value(uref, uv);
    frame.barrier_closure(ncl, uv);
    ui += 1;
  }

  // 活闭包 ncl 清 preload; 换了新闭包时经 protect 走 GC 检查。
  frame.clear_closure_preload(ncl);

  if kcl != ncl {
    frame.protect(pc_ptr, |frame| {
      frame.check_gc();
    });
  }

  // 跳过 nupvalues 条捕获字得下一条指令。
  frame.insn_offset(pc_ptr, nup as usize)
}

/// # Safety
/// C-ABI 导出边界：由生成码/VM 按 codegen 回调约定调用，`l`/`pc`/`base`/`k` 的合法性与
/// 存活性与 [`execute_dupclosure`] 的契约一致（本函数仅原样透传）。
pub unsafe extern "C-unwind" fn execute_dupclosure_export(
  l: *mut LuaState,
  pc: *const Instruction,
  base: StkId,
  k: *mut TValue,
) -> *const Instruction {
  // Safety: 导出 C ABI 入口原样转发 l/pc/base/k 给同契约 unsafe fn execute_dupclosure;
  // 调用方(生成的原生代码/VM)保证这些指针指向活的 LuaState/code/栈/常量, 满足被调前置条件。
  unsafe { execute_dupclosure(l, pc, base, k) }
}
