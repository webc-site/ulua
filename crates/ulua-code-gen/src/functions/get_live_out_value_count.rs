use crate::{
  functions::get_live_in_out_value_count::get_live_in_out_value_count,
  records::ir_function::IrFunction,
};

/// 起始块以索引传入，与链式统计入口保持一致。
pub fn get_live_out_value_count(function: &mut IrFunction, block_idx: u32) -> u32 {
  get_live_in_out_value_count(function, block_idx, false).1
}
