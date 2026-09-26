use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  macros::{
    luau_assert::LUAU_ASSERT,
    luau_insn_ops::{luau_insn_a, luau_insn_aux_kv16, luau_insn_c, luau_insn_op},
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

/// 生成码回写的 NAMECALL/NAMECALLUDATA 慢路径解释器（cpp `executeNAMECALL`）。
///
/// # Safety
/// VM 回调 ABI 约定：`l` 为存活 `LuaState`，`pc` 指向本帧 code 内一条 NAMECALL/
/// NAMECALLUDATA 指令（主字 + AUX 常量字），`base`/`k` 为本帧活动栈基址与常量表基址。
/// 边界契约集中于 [`VmFrame::new`]，其余为安全逻辑。
pub unsafe fn execute_namecall(
  l: *mut LuaState,
  pc: *const Instruction,
  base: StkId,
  k: *mut TValue,
) -> *const Instruction {
  let mut frame = unsafe { VmFrame::new(l, base) };

  let cl = frame.current_closure();
  let words = frame.insns(pc, 2);
  let (insn, aux) = (words[0], words[1]);
  // pc_ptr 为下一条指令地址(拆分前由 pc_ptr 两次自增推得, 即 pc+2; patch 目标为主字 pc)。
  let pc_ptr = frame.insn_offset(pc, 2);
  let ra = frame.reg_a(insn);
  let rb = frame.reg_b(insn);
  // NAMECALLUDATA 的常量号取 AUX 低 16 位, 其余走整 AUX 字。
  let kw = if luau_insn_op(insn) == LuauOpcode::LOP_NAMECALLUDATA as u32 {
    luau_insn_aux_kv16(aux)
  } else {
    aux
  };
  let kv = frame.kv(kw, cl, k);
  LUAU_ASSERT!(frame.is_string(kv));

  // 方法缺失统一收口：base 可能已被 protect 回写，重取 ra 后判 nil 即报错。
  let method_missing = |frame: &VmFrame| {
    let ra = frame.reg(luau_insn_a(insn) as i32);
    if frame.is_nil(ra) {
      frame.method_error(frame.slot_at(ra, 1), frame.tsvalue(kv));
    }
  };

  if frame.is_table(rb) {
    frame.set_stack_value(frame.slot_at(ra, 1), rb);
    frame.set_cachedslot(luau_insn_c(insn) as i32);
    frame.protect_sync_base(pc_ptr, |frame| {
      frame.gettable(rb, kv, ra);
    });
    frame.patch_c(pc, frame.cachedslot());

    method_missing(&frame);
  } else {
    let mt = if frame.is_userdata(rb) {
      frame.udata_metatable(frame.uvalue(rb))
    } else {
      frame.global_metatable(frame.value_type(rb))
    };

    let fn_nc = frame.meta_method(mt, TMS::TmNameCall);
    if let Some(fn_nc) = fn_nc {
      frame.set_stack_value(frame.slot_at(ra, 1), rb);
      frame.set_stack_value(ra, fn_nc);

      frame.set_namecall(frame.tsvalue(kv) as *mut _);
    } else {
      let tmi = frame
        .meta_method(mt, TMS::TmIndex)
        .filter(|tm| frame.is_table(*tm));
      if let Some(tmi) = tmi {
        let h = frame.hvalue(tmi);
        let slot = (luau_insn_c(insn) as i32) & frame.table_nodemask8(h);
        let (node_key, val_slot) = frame.table_node(h, slot as usize);
        let cached_hit = frame.is_string(node_key)
          && frame.tsvalue(node_key) == frame.tsvalue(kv)
          && !frame.is_nil(val_slot);
        if cached_hit {
          frame.set_stack_value(frame.slot_at(ra, 1), rb);
          frame.set_stack_value(ra, val_slot);
        } else {
          frame.set_stack_value(frame.slot_at(ra, 1), rb);
          frame.set_cachedslot(slot);
          frame.protect_sync_base(pc_ptr, |frame| {
            frame.gettable(rb, kv, ra);
          });
          frame.patch_c(pc, frame.cachedslot());

          method_missing(&frame);
        }
      } else {
        // 无 __index 表命中路径: protect 调 lua_v_gettable 走通用元方法。
        frame.set_stack_value(frame.slot_at(ra, 1), rb);
        frame.protect_sync_base(pc_ptr, |frame| {
          frame.gettable(rb, kv, ra);
        });

        method_missing(&frame);
      }
    }
  }

  // pc_ptr 为活字节码字, 断言其后随 CALL/CALLFB。
  let next_insn = frame.insns(pc_ptr, 1)[0];
  LUAU_ASSERT!(
    luau_insn_op(next_insn) == LuauOpcode::LOP_CALL as u32
      || luau_insn_op(next_insn) == LuauOpcode::LOP_CALLFB as u32
  );
  pc_ptr
}

/// # Safety
/// C-ABI 导出边界：由生成码/VM 按 codegen 回调约定调用，`l`/`pc`/`base`/`k` 的合法性与
/// 存活性与 [`execute_namecall`] 的契约一致（本函数仅原样透传）。
pub unsafe extern "C-unwind" fn execute_namecall_export(
  l: *mut LuaState,
  pc: *const Instruction,
  base: StkId,
  k: *mut TValue,
) -> *const Instruction {
  // Safety: 导出 C ABI 入口原样转发 l/pc/base/k 给同契约 unsafe fn execute_namecall;
  // 调用方(原生代码/VM)按 ABI 提供活的 LuaState 与帧内 code/栈/常量指针, 满足被调前置条件。
  unsafe { execute_namecall(l, pc, base, k) }
}
