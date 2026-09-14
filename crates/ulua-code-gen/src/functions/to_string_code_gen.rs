use alloc::string::String;

use crate::enums::code_gen_compilation_result::CodeGenCompilationResult;

/// cpp CodeGen.h:42 `toString(const CodeGenCompilationResult&)` 的对应移植（公开 API，当前 workspace 暂无调用点）。
pub fn to_string_code_gen_compilation_result(result: CodeGenCompilationResult) -> String {
  match result {
    CodeGenCompilationResult::Success => String::from("Success"),
    CodeGenCompilationResult::NothingToCompile => String::from("NothingToCompile"),
    CodeGenCompilationResult::NotNativeModule => String::from("NotNativeModule"),
    CodeGenCompilationResult::CodeGenNotInitialized => String::from("CodeGenNotInitialized"),
    CodeGenCompilationResult::CodeGenOverflowInstructionLimit => {
      String::from("CodeGenOverflowInstructionLimit")
    }
    CodeGenCompilationResult::CodeGenOverflowBlockLimit => {
      String::from("CodeGenOverflowBlockLimit")
    }
    CodeGenCompilationResult::CodeGenOverflowBlockInstructionLimit => {
      String::from("CodeGenOverflowBlockInstructionLimit")
    }
    CodeGenCompilationResult::CodeGenAssemblerFinalizationFailure => {
      String::from("CodeGenAssemblerFinalizationFailure")
    }
    CodeGenCompilationResult::CodeGenLoweringFailure => String::from("CodeGenLoweringFailure"),
    CodeGenCompilationResult::AllocationFailed => String::from("AllocationFailed"),
    CodeGenCompilationResult::Count => String::from("Count"),
  }
}
