use alloc::string::String;

use crate::enums::code_gen_compilation_result::CodeGenCompilationResult;

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
