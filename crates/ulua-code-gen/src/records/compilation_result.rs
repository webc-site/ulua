use alloc::vec::Vec;

use crate::{
  enums::code_gen_compilation_result::CodeGenCompilationResult,
  records::proto_compilation_failure::ProtoCompilationFailure,
};

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
