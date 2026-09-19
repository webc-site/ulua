use ulua_cli_lib::records::global_options::{get_debug_level, get_optimization_level};
use ulua_compiler::records::compile_options::CompileOptions;

pub fn copts() -> CompileOptions {
  CompileOptions {
    optimization_level: get_optimization_level(),
    debug_level: get_debug_level(),
    type_info_level: 1,
    ..Default::default()
  }
}
