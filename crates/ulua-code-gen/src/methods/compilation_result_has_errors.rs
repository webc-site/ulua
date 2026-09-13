use crate::{
  enums::code_gen_compilation_result::CodeGenCompilationResult,
  records::compilation_result::CompilationResult,
};
impl CompilationResult {
  pub fn has_errors(&self) -> bool {
    self.result != CodeGenCompilationResult::Success || !self.proto_failures.is_empty()
  }
}
