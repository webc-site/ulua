use core::mem::size_of;

use ulua_vm::records::lua_t_value::TValue;

use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  macros::{codegen_assert::CODEGEN_ASSERT, op_b::op_b},
  records::{const_prop_state::ConstPropState, ir_inst::IrInst, ir_op::IrOp},
};

impl ConstPropState {
  pub fn get_combined_array_load_offset_op(
    &mut self,
    array_addr_inst: &mut IrInst,
    load_offset_op: IrOp,
  ) -> IrOp {
    CODEGEN_ASSERT!(array_addr_inst.cmd == IrCmd::GetArrAddr);

    let function = unsafe { &mut *self.function };
    let load_offset = function.as_int_op(load_offset_op).unwrap_or(0);

    let op_b_inst = op_b(array_addr_inst.clone());
    if op_b_inst.kind() == IrOpKind::Constant {
      let array_addr_offset = function.int_op(op_b_inst) * size_of::<TValue>() as i32;

      if array_addr_offset != 0 || load_offset == 0 {
        CODEGEN_ASSERT!(load_offset == 0);
        let build = unsafe { &mut *self.build };
        return build.const_int(array_addr_offset);
      }
    } else {
      CODEGEN_ASSERT!(op_b_inst.kind() == IrOpKind::Inst);
      CODEGEN_ASSERT!(load_offset == 0);
      return op_b_inst;
    }

    CODEGEN_ASSERT!(load_offset_op.kind() == IrOpKind::Constant);
    load_offset_op
  }
}
