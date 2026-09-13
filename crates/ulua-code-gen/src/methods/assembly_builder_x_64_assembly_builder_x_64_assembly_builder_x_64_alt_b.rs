use crate::{
  functions::get_current_x_64_abi::get_current_x_64_abi,
  records::assembly_builder_x_64::AssemblyBuilderX64,
};

impl AssemblyBuilderX64 {
  pub fn assembly_builder_x_64_bool_i32(log_text: bool, features: u32) -> Self {
    let abi = get_current_x_64_abi();
    Self::assembly_builder_x_64_bool_abix_64_i32(log_text, abi, features)
  }
}
