use ulua_compiler::records::compile_options::CompileOptions as LuauCompileOptions;

use crate::{
  functions::coverage_active::coverage_active,
  records::global_options::{get_debug_level, get_optimization_level},
};

pub fn copts() -> LuauCompileOptions {
  LuauCompileOptions {
    optimization_level: get_optimization_level(),
    debug_level: get_debug_level(),
    type_info_level: 1,
    coverage_level: if coverage_active() { 2 } else { 0 },
    ..LuauCompileOptions::default()
  }
}
