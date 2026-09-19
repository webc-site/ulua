use core::mem::zeroed;

use ulua_code_gen::records::assembly_options::AssemblyOptions;
use ulua_compiler::records::compile_options::CompileOptions;
#[derive(Debug, Clone)]
#[repr(C)]
pub struct LoweringFixture {
  pub compilation_options: CompileOptions,
  pub compilation_options_c: CompileOptions,
  pub assembly_options: AssemblyOptions,
}

impl Default for LoweringFixture {
  fn default() -> Self {
    let compilation_options = CompileOptions {
      optimization_level: 2,
      debug_level: 1,
      type_info_level: 1,
      ..Default::default()
    };
    let compilation_options_c = CompileOptions {
      optimization_level: 2,
      debug_level: 1,
      type_info_level: 1,
      ..Default::default()
    };
    Self {
      compilation_options,
      compilation_options_c,
      assembly_options: unsafe { zeroed() },
    }
  }
}
