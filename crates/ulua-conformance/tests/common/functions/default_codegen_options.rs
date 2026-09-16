use core::{mem::zeroed, ptr::null};

use ulua_code_gen::{
  enums::code_gen_flags::CodeGenFlags, records::compilation_options::CompilationOptions,
};
pub fn default_codegen_options() -> CompilationOptions {
  let mut opts = CompilationOptions {
    flags: 0,
    hooks: unsafe { zeroed() },
    userdata_types: null(),
    record_counters: false,
    nop_padding: false,
  };

  opts.flags = CodeGenFlags::CODE_GEN_COLD_FUNCTIONS as u32;
  opts
}
