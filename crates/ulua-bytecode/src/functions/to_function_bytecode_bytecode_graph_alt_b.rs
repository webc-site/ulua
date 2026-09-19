use std::vec::Vec;

use crate::{
  functions::to_function_bytecode_bytecode_graph::to_function_bytecode_bytecode_builder_comp_time_bc_function,
  records::{bc_function::BcFunction, bytecode_builder::BytecodeBuilder},
};

pub fn to_function_bytecode_comp_time_bc_function(_fn_: &mut BcFunction) -> Vec<u8> {
  let mut bcb = BytecodeBuilder::new(None);
  to_function_bytecode_bytecode_builder_comp_time_bc_function(&mut bcb, _fn_)
}
