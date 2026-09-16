use ulua_bytecode::{
  enums::bc_op_kind::BcOpKind, records::bc_op::BcOp,
  type_aliases::comp_time_bc_function::CompTimeBcFunction,
};

pub fn is_phi_of(fn_: &mut CompTimeBcFunction, op: BcOp, left: BcOp, right: BcOp) -> bool {
  if op.kind != BcOpKind::Phi {
    return false;
  }

  let phi = fn_.phi_op(op);
  phi.ops.len() == 2 && phi.ops[0] == left && phi.ops[1] == right
}
