use ulua_vm::records::proto::Proto;

use crate::{
  enums::code_gen_compilation_result::CodeGenCompilationResult,
  functions::bind_native_protos::bind_native_protos,
  records::{module_bind_result::ModuleBindResult, shared_code_gen_context::SharedCodeGenContext},
  type_aliases::module_id::ModuleId,
};

impl SharedCodeGenContext {
  pub fn try_bind_existing_module(
    &mut self,
    module_id: &ModuleId,
    module_protos: &[*mut Proto],
  ) -> Option<ModuleBindResult> {
    let native_module_ref = self.shared_allocator.try_get_native_module(module_id);

    if native_module_ref.native_module_ref_empty() {
      return None;
    }

    let native_module = unsafe { &*native_module_ref.native_module_ref_get() };
    let mut native_protos = native_module.native_module_get_native_protos().clone();
    let protos_bound = bind_native_protos(module_protos, &mut native_protos, false);
    native_module.native_module_add_refs(protos_bound as usize);

    Some(ModuleBindResult {
      compilation_result: CodeGenCompilationResult::Success,
      functions_bound: protos_bound,
    })
  }
}
