use crate::{
  functions::{
    emit_clear_native_flag_emit_common_x_64::emit_clear_native_flag,
    emit_exit_emit_common_x_64::emit_exit, emit_interrupt_emit_common_x_64::emit_interrupt,
    emit_return_emit_common_x_64::emit_return,
    emit_update_pc_for_exit_emit_common_x_64::emit_update_pc_for_exit,
  },
  records::{assembly_builder_x_64::AssemblyBuilderX64, module_helpers::ModuleHelpers},
};

pub fn assemble_helpers(build: &mut AssemblyBuilderX64, helpers: &mut ModuleHelpers) {
  if build.log_text {
    build.log_append(format_args!("; updatePcAndContinueInVm\n"));
  }
  build.set_label_label(&mut helpers.update_pc_and_continue_in_vm);
  emit_update_pc_for_exit(build);

  if build.log_text {
    build.log_append(format_args!("; exitContinueVmClearNativeFlag\n"));
  }
  build.set_label_label(&mut helpers.exit_continue_vm_clear_native_flag);
  emit_clear_native_flag(build);

  if build.log_text {
    build.log_append(format_args!("; exitContinueVm\n"));
  }
  build.set_label_label(&mut helpers.exit_continue_vm);
  emit_exit(build, true);

  if build.log_text {
    build.log_append(format_args!("; exitNoContinueVm\n"));
  }
  build.set_label_label(&mut helpers.exit_no_continue_vm);
  emit_exit(build, false);

  if build.log_text {
    build.log_append(format_args!("; interrupt\n"));
  }
  build.set_label_label(&mut helpers.interrupt);
  emit_interrupt(build);

  if build.log_text {
    build.log_append(format_args!("; return\n"));
  }
  build.set_label_label(&mut helpers.return_);
  emit_return(build, helpers);
}
