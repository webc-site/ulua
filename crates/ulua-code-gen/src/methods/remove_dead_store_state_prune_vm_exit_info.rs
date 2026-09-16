use crate::{
  enums::ir_cmd::IrCmd,
  functions::{remove_use::remove_use, visit_arguments::visit_arguments},
  records::{
    ir_function::IrFunction, ir_inst::IrInst, remove_dead_store_state::RemoveDeadStoreState,
    vm_exit_sync_info::VmExitSyncInfo,
  },
};

impl RemoveDeadStoreState {
  // VmExit information contains data that needs a sync if the stores are removed as unused
  pub fn prune_vm_exit_info(&mut self) {
    let function: *mut IrFunction = self.function;

    let n = self.recorded_vm_exit_syncs.len();
    for item in self.recorded_vm_exit_syncs.iter().take(n) {
      let inst_idx = *item;

      let sync_info: *mut VmExitSyncInfo =
        unsafe { (*function).vm_exit_info.get_or_insert(inst_idx) };

      let mut i = 0usize;
      while i < unsafe { (*sync_info).reg_stores.len() } {
        let mut j = 0usize;
        while j < unsafe { (&(*sync_info).reg_stores)[i].stores.size() as usize } {
          let store_inst_idx =
            unsafe { (&(*sync_info).reg_stores)[i].stores.as_slice()[j].inst_idx };

          if unsafe { (&(*function).instructions)[store_inst_idx as usize].cmd } != IrCmd::NOP {
            let inst_ptr: *mut IrInst =
              unsafe { &mut (&mut (*function).instructions)[store_inst_idx as usize] };
            visit_arguments(unsafe { &mut *inst_ptr }, |op| {
              remove_use(unsafe { &mut *function }, op);
            });

            let back = unsafe { (&(*sync_info).reg_stores)[i].stores.back().clone() };
            unsafe {
              (&mut (*sync_info).reg_stores)[i].stores.as_mut_slice()[j] = back;
              (&mut (*sync_info).reg_stores)[i].stores.pop_back();
            }
          } else {
            j += 1;
          }
        }

        if unsafe { (&(*sync_info).reg_stores)[i].stores.empty() } {
          let back = unsafe { (*sync_info).reg_stores.last().unwrap().clone() };
          unsafe {
            (&mut (*sync_info).reg_stores)[i] = back;
            (*sync_info).reg_stores.pop();
          }
        } else {
          i += 1;
        }
      }
    }
  }
}
