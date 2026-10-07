//! Source: `CodeGen/src/CodeGenUtils.cpp`

use ulua_common::macros::luau_insn_ops::{luau_insn_a, luau_insn_d};
use ulua_vm::{
  enums::tms::TMS,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

use crate::{
  records::vm_frame::VmFrame,
  type_aliases::{api::LuaState, ir::Instruction},
};

/// 生成码回写的 FORGPREP 慢路径解释器（cpp `executeFORGPREP`）。
///
/// # Safety
/// VM 回调 ABI 约定：`l` 为存活 `LuaState`，`pc` 指向本帧 code 内一条 FORGPREP 指令，
/// `base` 为本帧活动栈基址（`k` 未用）。边界契约集中于 [`VmFrame::new`]，其余为安全逻辑。
pub unsafe extern "C-unwind" fn execute_forgprep(
  l: *mut LuaState,
  pc: *const Instruction,
  base: StkId,
  _k: *mut TValue,
) -> *const Instruction {
  // Safety: 本函数头 ABI 契约保证 `l`/`base` 为存活 LuaState 与活动帧基址；VmFrame::new 仅收编地址对、不解引用。
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
