use crate::{
  records::{
    compilation_options::CompilationOptions, compilation_result::CompilationResult,
    compilation_stats::CompilationStats,
  },
  type_aliases::{lua_state::lua_State, module_id::ModuleId},
};

/// **Out of scope** — see `compile_code_gen_context.rs`. `Luau::CodeGen::compileInternal`
/// is the native-codegen entry point; ulua executes via the bytecode interpreter
/// (docs/CONFORMANCE.md). It was a phantom `extern` to the C++ symbol (unresolved
/// on Windows), so it is stubbed explicitly.
pub fn compile_lua_state_i32_compilation_options_compilation_stats(
  _l: *mut lua_State,
  _idx: i32,
  _options: &CompilationOptions,
  _stats: *mut CompilationStats,
) -> CompilationResult {
  let _ = ModuleId::default();
  unimplemented!(
    "ulua does not execute JIT-compiled native code (out of scope; see docs/CONFORMANCE.md)"
  )
}
