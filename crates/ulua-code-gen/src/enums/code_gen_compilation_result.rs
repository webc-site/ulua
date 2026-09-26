#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[derive(Default)]
pub enum CodeGenCompilationResult {
  #[default]
  Success = 0,
  NothingToCompile = 1,
  NotNativeModule = 2,
  CodeGenNotInitialized = 3,
  CodeGenOverflowInstructionLimit = 4,
  CodeGenOverflowBlockLimit = 5,
  CodeGenOverflowBlockInstructionLimit = 6,
  CodeGenAssemblerFinalizationFailure = 7,
  CodeGenLoweringFailure = 8,
  AllocationFailed = 9,
  Count = 10,
}
