use crate::{
  enums::ir_cmd::IrCmd,
  macros::{has_op_c::HAS_OP_C, op_c::op_c},
  records::{ir_function::IrFunction, ir_op::IrOp},
};

pub fn try_get_operand_tag(function: &mut IrFunction, op: IrOp) -> Option<u8> {
  let arg_ptr = function.as_inst_op(op);
  if !arg_ptr.is_null() {
    let arg = unsafe { &*arg_ptr };
    if arg.cmd == IrCmd::TagVector {
      return Some(5); // LUA_TVECTOR
    }

    if arg.cmd == IrCmd::LoadTvalue && HAS_OP_C!(arg) {
      let op_c = op_c(arg.clone());
      return Some(function.tag_op(op_c));
    }
  }

  None
}
