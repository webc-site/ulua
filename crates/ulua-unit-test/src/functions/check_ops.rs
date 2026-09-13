use alloc::collections::VecDeque;

use ulua_bytecode::{
  records::bc_op::BcOp, type_aliases::comp_time_bc_function::CompTimeBcFunction,
};
use ulua_common::enums::luau_opcode::LuauOpcode;
pub fn check_ops(
  fn_: &mut CompTimeBcFunction,
  ops: &VecDeque<BcOp>,
  expected_ops: &[LuauOpcode],
) -> bool {
  if ops.len() != expected_ops.len() {
    return false;
  }

  for (op, expected) in ops.iter().zip(expected_ops.iter()) {
    if fn_.inst_op(*op).op != *expected {
      return false;
    }
  }

  true
}
