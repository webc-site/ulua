use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::remove_use::remove_use,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_function::IrFunction, ir_inst::IrInst, ir_op::IrOp},
};

pub fn apply_substitutions_ir_function_ir_op(function: &mut IrFunction, op: &mut IrOp) {
  if op.kind() == IrOpKind::Inst {
    let src: *mut IrInst = &mut function.instructions[op.index() as usize];

    if unsafe { (*src).cmd } == IrCmd::SUBSTITUTE {
      let src_a: IrOp = unsafe { (*src).ops.as_slice()[0] };
      *op = src_a;

      // If we substitute with the result of a different instruction, update the use count
      if op.kind() == IrOpKind::Inst {
        let dst = &mut function.instructions[op.index() as usize];
        CODEGEN_ASSERT!(dst.cmd != IrCmd::SUBSTITUTE);
        dst.use_count += 1;
      }

      CODEGEN_ASSERT!(unsafe { (*src).use_count } > 0);
      unsafe {
        (*src).use_count -= 1;
      }

      if unsafe { (*src).use_count } == 0 {
        unsafe {
          (*src).cmd = IrCmd::NOP;
        }
        let src_a2: IrOp = unsafe { (*src).ops.as_slice()[0] };
        remove_use(function, src_a2);
        unsafe {
          (*src).ops.clear();
        }
      }
    }
  }
}
