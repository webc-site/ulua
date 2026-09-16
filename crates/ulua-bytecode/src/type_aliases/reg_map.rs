use std::collections::HashMap;

use crate::{
  records::{bc_op::BcOp, bc_op_hash::BcOpHash},
  type_aliases::reg::Reg,
};

pub type RegMap = HashMap<BcOp, Reg, BcOpHash>;
