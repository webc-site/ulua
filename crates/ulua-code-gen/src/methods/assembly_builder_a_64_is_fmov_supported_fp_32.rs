use crate::{
  functions::get_fmov_imm_fp_32::get_fmov_imm_fp_32,
  records::assembly_builder_a_64::AssemblyBuilderA64,
};

impl AssemblyBuilderA64 {
  pub fn is_fmov_supported_fp_32(&mut self, value: f32) -> bool {
    get_fmov_imm_fp_32(value) >= 0
  }
}
