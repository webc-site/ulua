extern crate alloc;

use alloc::string::String;

use crate::{functions::to_dot::to_dot, records::ir_function::IrFunction};

pub fn dump_dot(function: &IrFunction, include_inst: bool) -> String {
  let result = to_dot(function, include_inst);

  std::println!("{}", result);

  result
}
