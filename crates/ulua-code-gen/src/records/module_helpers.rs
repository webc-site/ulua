use crate::records::label::Label;

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
