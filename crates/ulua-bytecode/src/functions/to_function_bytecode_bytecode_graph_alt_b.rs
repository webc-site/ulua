use alloc::string::String;

use crate::{
  functions::to_function_bytecode_bytecode_graph::to_function_bytecode_bytecode_builder_comp_time_bc_function,
  records::bytecode_builder::BytecodeBuilder,
  type_aliases::comp_time_bc_function::CompTimeBcFunction,
};

pub fn to_function_bytecode_comp_time_bc_function(_fn_: &mut CompTimeBcFunction) -> String {
  let mut bcb = BytecodeBuilder::new(None);
  to_function_bytecode_bytecode_builder_comp_time_bc_function(&mut bcb, _fn_)
}
