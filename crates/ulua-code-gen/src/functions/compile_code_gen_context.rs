use crate::{
  records::{
    compilation_options::CompilationOptions, compilation_result::CompilationResult,
    compilation_stats::CompilationStats,
  },
  type_aliases::{lua_state::lua_State, module_id::ModuleId},
};

/// **Out of scope.** `Luau::CodeGen::compileInternal` generates and installs
/// native machine code; ulua's execution oracle is the bytecode interpreter
/// (see docs/CONFORMANCE.md), so this was never ported. It survived only as an
/// `extern` declaration of the original C++ mangled symbol, which has no
/// implementation to link against (lld DCE'd the reference on Linux/macOS, MSVC
/// kept it → unresolved external symbol on Windows). Stub it explicitly.
pub fn compile_module_id_lua_state_i32_compilation_options_compilation_stats(
  _module_id: &ModuleId,
  _l: *mut lua_State,
  _idx: i32,
  _options: &CompilationOptions,
  _stats: *mut CompilationStats,
) -> CompilationResult {
  unimplemented!(
    "ulua does not execute JIT-compiled native code (out of scope; see docs/CONFORMANCE.md)"
  )
}
