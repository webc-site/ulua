use alloc::collections::VecDeque;

use ulua_bytecode::records::{bc_function::BcFunction, bc_op::BcOp};
use ulua_common::enums::luau_opcode::LuauOpcode;
pub fn check_ops(
  fn_: &mut BcFunction<'_>,
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
