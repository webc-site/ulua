use crate::{
  functions::{
    emit_clear_native_flag_code_gen_a_64::emit_clear_native_flag_assembly_builder_a_64,
    emit_continue_call::emit_continue_call,
    emit_exit_code_gen_a_64::emit_exit_assembly_builder_a_64_bool,
    emit_interrupt_code_gen_a_64::emit_interrupt,
    emit_return_code_gen_a_64::emit_return_assembly_builder_a_64_module_helpers,
    emit_update_pc_for_exit_code_gen_a_64::emit_update_pc_for_exit_assembly_builder_a_64,
  },
  records::{assembly_builder_a_64::AssemblyBuilderA64, module_helpers::ModuleHelpers},
};

pub fn assemble_helpers(build: &mut AssemblyBuilderA64, helpers: &mut ModuleHelpers) {
  if build.log_text {
    build.log_append(format_args!("; updatePcAndContinueInVm\n"));
  }
  build.set_label_label(&mut helpers.update_pc_and_continue_in_vm);
  emit_update_pc_for_exit_assembly_builder_a_64(build);

  if build.log_text {
    build.log_append(format_args!("; exitContinueVmClearNativeFlag\n"));
  }
  build.set_label_label(&mut helpers.exit_continue_vm_clear_native_flag);
  emit_clear_native_flag_assembly_builder_a_64(build);

  if build.log_text {
    build.log_append(format_args!("; exitContinueVm\n"));
  }
  build.set_label_label(&mut helpers.exit_continue_vm);
  emit_exit_assembly_builder_a_64_bool(build, true);

  if build.log_text {
    build.log_append(format_args!("; exitNoContinueVm\n"));
  }
  build.set_label_label(&mut helpers.exit_no_continue_vm);
  emit_exit_assembly_builder_a_64_bool(build, false);

  if build.log_text {
    build.log_append(format_args!("; interrupt\n"));
  }
  build.set_label_label(&mut helpers.interrupt);
  emit_interrupt(build);

  if build.log_text {
    build.log_append(format_args!("; return\n"));
  }
  build.set_label_label(&mut helpers.return_);
  emit_return_assembly_builder_a_64_module_helpers(build, helpers);

  if build.log_text {
    build.log_append(format_args!("; continueCall\n"));
  }
  build.set_label_label(&mut helpers.continue_call);
  emit_continue_call(build, helpers);
}
