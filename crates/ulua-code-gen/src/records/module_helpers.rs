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

impl ModuleHelpers {
  pub const EXIT_CONTINUE_VM: Label = Label {
    id: 0,
    location: !0u32,
  };
  pub const EXIT_NO_CONTINUE_VM: Label = Label {
    id: 0,
    location: !0u32,
  };
  pub const EXIT_CONTINUE_VM_CLEAR_NATIVE_FLAG: Label = Label {
    id: 0,
    location: !0u32,
  };
  pub const UPDATE_PC_AND_CONTINUE_IN_VM: Label = Label {
    id: 0,
    location: !0u32,
  };
  pub const RETURN: Label = Label {
    id: 0,
    location: !0u32,
  };
  pub const INTERRUPT: Label = Label {
    id: 0,
    location: !0u32,
  };
  pub const CONTINUE_CALL: Label = Label {
    id: 0,
    location: !0u32,
  };
}
