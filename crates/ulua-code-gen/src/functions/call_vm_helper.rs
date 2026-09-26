//! VM 辅助调用共享骨架：`call_get_table`/`call_set_table`/`call_length_helper`/
//! `call_arith_helper`/`call_barrier_table_fast`/`call_step_gc` 六者同形——
//! 构造 `IrCallWrapperX64` → 前置 `R_STATE` 实参 → 逐参 `add_argument` →
//! 调 `NativeContext` 固定偏移处的 VM 辅助函数（可选收尾 `emit_update_base`），
//! 样板在此坍缩为单一核心。

use crate::{
  enums::size_x_64::SizeX64,
  functions::emit_update_base_emit_common_x_64::emit_update_base,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64,
    emit_common_x_64::{R_NATIVE_CONTEXT, R_STATE},
    ir_call_wrapper_x_64::IrCallWrapperX64,
    ir_op::IrOp,
    ir_reg_alloc_x_64::IrRegAllocX64,
    operand_x_64::OperandX64,
    register_x_64::RegisterX64,
  },
};

/// 单个调用实参：(目标大小, 操作数, 关联 IrOp)。
pub(crate) type VmCallArg = (SizeX64, OperandX64, IrOp);

/// 经 `IrCallWrapperX64` 调用 `NativeContext` 偏移 `off` 处的 VM 辅助函数。
/// 自动前置 `R_STATE` 首参；`inst_idx` 传 `K_INVALID_INST_IDX` 或当前指令索引；
/// `update_base` 控制调用收尾是否发射 `emit_update_base`。
pub(crate) fn call_vm_helper(
  regs: &mut IrRegAllocX64,
  build: &mut AssemblyBuilderX64,
  inst_idx: u32,
  args: &[VmCallArg],
  off: i32,
  update_base: bool,
) {
  let mut call_wrap =
    IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(regs, build, inst_idx);

  call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
    SizeX64::Qword,
    OperandX64::reg(R_STATE),
    IrOp::new(),
  );
  for &(size, source, source_op) in args {
    call_wrap.add_argument_size_x_64_operand_x_64_ir_op(size, source, source_op);
  }

  call_wrap.call(&OperandX64::mem(
    SizeX64::Qword,
    RegisterX64::NOREG,
    1,
    R_NATIVE_CONTEXT,
    off,
  ));

  if update_base {
    emit_update_base(build);
  }
}
