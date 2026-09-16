use crate::{
  enums::host_metamethod::HostMetamethod,
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};

pub type HostUserdataMetamethodHandler = Option<
  unsafe extern "C-unwind" fn(
    builder: *mut IrBuilder,
    lhs_ty: u8,
    rhs_ty: u8,
    result_reg: i32,
    lhs: IrOp,
    rhs: IrOp,
    method: HostMetamethod,
    pcpos: i32,
  ) -> bool,
>;
