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
  /// cpp `returnSites`：`migrateBlocks` 只登记 (调用者块, 目标 RETURN) 站点，
  /// 真正的替换延后到 `migrateBlockPhis` 之后，确保 `varArgMoves` 已填好。
  pub(crate) return_sites: Vec<(BcOp, BcOp)>,
  pub(crate) call_projections: DenseHashSet<BcOp, BcOpHash>,
  pub(crate) var_arg_moves: DenseHashMap<BcOp, Vec<BcOp>, BcOpHash>,
  /// 目标 phi → 调用者 phi 备忘录：递归映射前先查表，既保证同一目标 phi
  /// 恒等映射到同一调用者 phi（op 身份一致），也避免 loop-carried phi
  /// 自引用导致的无限递归（对齐 cpp `mappedPhis`）。
  pub(crate) mapped_phis: DenseHashMap<BcOp, BcOp, BcOpHash>,
}

impl<'a> CallInliner<'a> {
  pub const K_MAX_INLINER_COMBINED_STACK_SIZE: u8 = 250;
}
