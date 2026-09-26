use crate::records::ir_op::IrOp;

#[derive(Debug, Clone, Copy)]
pub struct BuiltinArgs {
  pub ra: i32,
  pub arg: i32,
  pub args: IrOp,
  pub arg3: IrOp,
  pub nparams: i32,
  pub nresults: i32,
  pub pcpos: i32,
}
