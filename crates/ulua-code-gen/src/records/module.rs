use crate::{enums::code_gen_compilation_result::CodeGenCompilationResult, records::label::Label};

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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct ModuleHelpers {
  pub exit_continue_vm: Label,
  pub exit_no_continue_vm: Label,
  pub exit_continue_vm_clear_native_flag: Label,
  pub update_pc_and_continue_in_vm: Label,
  pub return_: Label,
  pub interrupt: Label,
  pub continue_call: Label,
}
