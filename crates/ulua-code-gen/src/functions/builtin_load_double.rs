use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn builtin_load_double(build: &mut IrBuilder, arg: IrOp) -> IrOp {
  if arg.kind() == IrOpKind::Constant {
    return arg;
  }

  build.inst_ir_cmd_ir_op(IrCmd::LoadDouble, arg)
}
