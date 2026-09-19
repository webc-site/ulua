use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::ir_cmd::IrCmd,
  macros::{has_op_c::HAS_OP_C, op_c_ref::op_c_ref},
  records::{ir_function::IrFunction, ir_op::IrOp},
};

pub fn try_get_operand_tag(function: &IrFunction, op: IrOp) -> Option<u8> {
  let arg = function.as_inst_op_ref(op)?;
  if arg.cmd == IrCmd::TagVector {
    return Some(LuaType::Vector as u8);
  }

  if arg.cmd == IrCmd::LoadTvalue && HAS_OP_C!(arg) {
    let op_c = op_c_ref(arg);
    return Some(function.tag_op(op_c));
  }

  None
}
