use ulua_compiler::records::compile_options::CompileOptions as LuauCompileOptions;

use crate::functions::{coverage_active::coverage_active, repl_main::GLOBAL_OPTIONS};

pub fn copts() -> LuauCompileOptions {
  let mut result = LuauCompileOptions::default();

  unsafe {
    result.optimization_level = GLOBAL_OPTIONS.optimization_level;
    result.debug_level = GLOBAL_OPTIONS.debug_level;
  }

  result.type_info_level = 1;
  result.coverage_level = if coverage_active() { 2 } else { 0 };

  result
}
