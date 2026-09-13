use crate::{
  functions::{
    compute_cfg_block_edges::compute_cfg_block_edges,
    compute_cfg_dominance_tree_children::compute_cfg_dominance_tree_children,
    compute_cfg_immediate_dominators::compute_cfg_immediate_dominators,
    compute_cfg_live_in_out_reg_sets::compute_cfg_live_in_out_reg_sets,
  },
  records::ir_function::IrFunction,
};

pub fn compute_cfg_info(function: &mut IrFunction) {
  compute_cfg_block_edges(function);
  compute_cfg_immediate_dominators(function);
  compute_cfg_dominance_tree_children(function);
  compute_cfg_live_in_out_reg_sets(function);
}
