use crate::{
  functions::visit_vm_reg_defs_uses_ir_visit_use_def::visit_vm_reg_defs_uses_t_ir_function_ir_inst,
  records::{
    block_vm_reg_live_in_computation::BlockVmRegLiveInComputation, ir_block::IrBlock,
    ir_function::IrFunction,
  },
};

pub fn visit_vm_reg_defs_uses_t_ir_function_ir_block(
  visitor: &mut BlockVmRegLiveInComputation<'_>,
  function: &mut IrFunction,
  block: &IrBlock,
) {
  let start = block.start;
  let finish = block.finish;

  for inst_idx in start..=finish {
    // Safety: the callee accesses function methods (int_op, uint_op) but never
    // re-borrows function.instructions[inst_idx], so inst_ptr and &mut function
    // do not alias in the callee's execution.
    let inst_ptr = &mut function.instructions[inst_idx as usize] as *mut _;
    unsafe {
      visit_vm_reg_defs_uses_t_ir_function_ir_inst(visitor, function, &mut *inst_ptr);
    }
  }
}
