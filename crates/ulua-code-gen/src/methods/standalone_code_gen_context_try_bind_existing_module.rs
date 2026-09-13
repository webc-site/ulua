use ulua_vm::records::proto::Proto;

use crate::{
  records::{
    module_bind_result::ModuleBindResult, standalone_code_gen_context::StandaloneCodeGenContext,
  },
  type_aliases::module_id::ModuleId,
};

impl StandaloneCodeGenContext {
  pub fn try_bind_existing_module(
    &mut self,
    _module_id: &ModuleId,
    _module_protos: &[*mut Proto],
  ) -> Option<ModuleBindResult> {
    // The StandaloneCodeGenContext does not support sharing of native code
    None
  }
}
