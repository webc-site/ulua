use crate::{
  functions::get_fmov_imm_fp_64::get_fmov_imm_fp_64,
  records::assembly_builder_a_64::AssemblyBuilderA64,
};

impl AssemblyBuilderA64 {
  pub fn is_fmov_supported_fp_64(&mut self, value: f64) -> bool {
    get_fmov_imm_fp_64(value) >= 0
  }
}

#[unsafe(export_name = "ulua_assembly_builder_a_64_is_fmov_supported_fp_64")]
pub extern "C-unwind" fn assembly_builder_a_64_is_fmov_supported_fp_64() {}
