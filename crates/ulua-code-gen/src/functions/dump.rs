extern crate alloc;

use alloc::string::String;

use crate::{
  enums::include_use_info::IncludeUseInfo,
  functions::to_string_ir_dump_alt_g::to_string as to_string_function,
  records::ir_function::IrFunction,
};

pub fn dump(function: &mut IrFunction) -> String {
  let result = to_string_function(function, IncludeUseInfo::Yes);

  std::println!("{}", result);

  result
}
