//! Source: `CodeGen/src/CodeGenUtils.cpp`

use ulua_common::macros::{luau_insn_a::luau_insn_a, luau_insn_d::luau_insn_d};
use ulua_vm::{
  enums::tms::TMS,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

use crate::{
  records::vm_frame::VmFrame,
  type_aliases::{instruction_ir_builder::Instruction, lua_state::LuaState},
};

/// 生成码回写的 FORGPREP 慢路径解释器（cpp `executeFORGPREP`）。
///
/// # Safety
/// VM 回调 ABI 约定：`l` 为存活 `LuaState`，`pc` 指向本帧 code 内一条 FORGPREP 指令，
/// `base` 为本帧活动栈基址（`k` 未用）。边界契约集中于 [`VmFrame::new`]，其余为安全逻辑。
pub unsafe fn execute_forgprep(
  l: *mut LuaState,
  pc: *const Instruction,
  base: StkId,
  _k: *mut TValue,
) -> *const Instruction {
  let mut frame = unsafe { VmFrame::new(l, base) };

  // FORGPREP 为单字指令（D 跳距在主字内）；pc 推进到 AUX/跳转源字位置。
  let insn = frame.insns(pc, 1)[0];
  let pc = frame.insn_offset(pc, 1);

  let mut ra = frame.reg(luau_insn_a(insn) as i32);

  if !frame.is_function(ra) {
    let mt = if frame.is_table(ra) {
      frame.table_metatable(frame.hvalue(ra))
    } else if frame.is_userdata(ra) {
      frame.udata_metatable(frame.uvalue(ra))
    } else {
      None
    };

    if let Some(fn_iter) = frame.meta_method(mt, TMS::TmIter) {
      frame.set_stack_value(frame.slot_at(ra, 1), ra);
      frame.set_stack_value(ra, fn_iter);

      frame.set_top(frame.slot_at(ra, 2)); // func + self arg

      // savedpc 记录后受保护调用（base 由 protect 回写），结束把 top 收回帧界。
      frame.protect_sync_base(pc, |frame| frame.call(ra, 3));
      frame.set_top(frame.ci_top());

      // 栈可能已重分配，重新计算 ra
      ra = frame.reg(luau_insn_a(insn) as i32);

      // 防护 __iter 返回 nil
      if frame.is_nil(ra) {
        frame.save_pc(pc);
        frame.type_error(ra, "call");
      }
    } else if frame.meta_method(mt, TMS::TmCall).is_some() {
      // 带 __call 的 table 或 userdata，将在 FORGLOOP 期间被调用
    } else if frame.is_table(ra) {
      // 为内建迭代设置寄存器
      frame.set_stack_value(frame.slot_at(ra, 1), ra);
      frame.set_iterator_done(frame.slot_at(ra, 2));
      frame.set_nil(ra);
    } else {
      frame.save_pc(pc);
      frame.type_error(ra, "iterate over");
    }
  }

  // insn_d 编码跳距落在本字节码数组界内(offset 仅算不读)。
  frame.insn_jump(pc, luau_insn_d(insn) as isize)
}

/// # Safety
/// C-ABI 导出边界：由生成码/VM 按 codegen 回调约定调用，`l`/`pc`/`base`/`k` 的合法性与
/// 存活性与 [`execute_forgprep`] 的契约一致（本函数仅原样透传）。
pub unsafe extern "C-unwind" fn execute_forgprep_export(
  l: *mut LuaState,
  pc: *const Instruction,
  base: StkId,
  k: *mut TValue,
) -> *const Instruction {
  // Safety: `extern "C-unwind"` FFI 壳，按 Lua/C API 与 codegen 约定由 VM 宿主传入存活的 `LuaState`、
  // 合法字节码 `pc`、栈 `base` 与常量表 `k`；本行原样转发给 `execute_forgprep`，其 unsafe 前置条件
  // 即由上述宿主调用协议满足。
  unsafe { execute_forgprep(l, pc, base, k) }
}
