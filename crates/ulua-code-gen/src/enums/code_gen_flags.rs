#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum CodeGenFlags {
  CodeGenOnlyNativeModules = 1 << 0,
  CodeGenColdFunctions = 1 << 1,
}

impl CodeGenFlags {
  pub const CODE_GEN_ONLY_NATIVE_MODULES: CodeGenFlags = CodeGenFlags::CodeGenOnlyNativeModules;
  pub const CODE_GEN_COLD_FUNCTIONS: CodeGenFlags = CodeGenFlags::CodeGenColdFunctions;
}
