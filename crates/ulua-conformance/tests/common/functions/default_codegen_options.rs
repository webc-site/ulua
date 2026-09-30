use ulua_code_gen::{
  enums::code_gen_flags::CodeGenFlags, records::compilation_options::CompilationOptions,
};

/// cpp `Conformance.test.cpp:118-123`：`CompilationOptions opts = {}` 后只把
/// `flags` 置为 `CodeGen_ColdFunctions`，其余字段取零值。
pub fn default_codegen_options() -> CompilationOptions {
  CompilationOptions {
    flags: CodeGenFlags::CODE_GEN_COLD_FUNCTIONS as u32,
    ..Default::default()
  }
}
