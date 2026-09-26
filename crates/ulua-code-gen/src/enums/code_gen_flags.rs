//! Source: `CodeGen/include/Luau/CodeGenOptions.h:17`

crate::flag_enum! {
  pub enum CodeGenFlags: u32 {
    CodeGenOnlyNativeModules = 1 << 0,
    CodeGenColdFunctions = 1 << 1,
  }
  aliases {
    CODE_GEN_COLD_FUNCTIONS = CodeGenColdFunctions,
  }
}
