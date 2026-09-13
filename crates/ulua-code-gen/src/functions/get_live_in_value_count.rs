use crate::{
  functions::get_live_in_out_value_count::get_live_in_out_value_count,
  records::{ir_block::IrBlock, ir_function::IrFunction},
};

pub fn get_live_in_value_count(function: &mut IrFunction, block: &mut IrBlock) -> u32 {
  get_live_in_out_value_count(function, block, false).0
}
