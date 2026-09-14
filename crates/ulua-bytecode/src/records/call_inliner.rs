use alloc::vec::Vec;

use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  records::{bc_call_fb::BcCallFB, bc_function::BcFunction, bc_op::BcOp, bc_op_hash::BcOpHash},
  type_aliases::reg::Reg,
};

#[derive(Debug)]
pub struct CallInliner<'a> {
  pub(crate) caller: &'a mut BcFunction,
  pub(crate) target: &'a mut BcFunction,
  pub(crate) call: BcCallFB<'a>,
  pub(crate) call_params: Vec<BcOp>,
  pub(crate) target_reg: Reg,

  pub(crate) caller_blocks_size_before_inline: u32,
  pub(crate) caller_inst_size_before_inline: u32,
  pub(crate) caller_vm_const_size_before_inline: u32,
  pub(crate) caller_proto_size_before_inline: u32,
  pub(crate) caller_up_val_size_before_inline: u8,

  pub(crate) return_ops: Vec<BcOp>,
  pub(crate) call_projections: DenseHashSet<BcOp, BcOpHash>,
  pub(crate) var_arg_moves: DenseHashMap<BcOp, Vec<BcOp>, BcOpHash>,
}

impl<'a> CallInliner<'a> {
  pub const K_MAX_INLINER_COMBINED_STACK_SIZE: u8 = 250;
}
