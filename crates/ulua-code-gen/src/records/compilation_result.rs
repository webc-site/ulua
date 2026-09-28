use alloc::{string::String, vec::Vec};

use crate::enums::code_gen_compilation_result::CodeGenCompilationResult;

#[derive(Debug, Clone)]
pub struct CompilationResult {
  pub result: CodeGenCompilationResult,
  pub proto_failures: Vec<ProtoCompilationFailure>,
}

impl Default for CompilationResult {
  fn default() -> Self {
    Self {
      result: CodeGenCompilationResult::Success,
      proto_failures: Vec::new(),
    }
  }
}

impl CompilationResult {
  pub fn has_errors(&self) -> bool {
    self.result != CodeGenCompilationResult::Success || !self.proto_failures.is_empty()
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProtoCompilationFailure {
  pub result: CodeGenCompilationResult,
  pub debugname: String,
  pub line: i32,
}

impl Default for ProtoCompilationFailure {
  fn default() -> Self {
    Self {
      result: CodeGenCompilationResult::Success,
      debugname: String::new(),
      line: -1,
    }
  }
}
