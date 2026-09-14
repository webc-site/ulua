use crate::enums::code_gen_compilation_result::CodeGenCompilationResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct ModuleBindResult {
  pub compilation_result: CodeGenCompilationResult,
  pub functions_bound: u32,
}

impl Default for ModuleBindResult {
  fn default() -> Self {
    Self {
      compilation_result: CodeGenCompilationResult::Success,
      functions_bound: 0,
    }
  }
}

impl ModuleBindResult {
  pub const COMPILATION_RESULT: CodeGenCompilationResult = CodeGenCompilationResult::Success;
  pub const FUNCTIONS_BOUND: u32 = 0;
}
