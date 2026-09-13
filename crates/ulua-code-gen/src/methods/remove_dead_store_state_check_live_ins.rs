use ulua_common::FFlag::LuauCodegenVmExitSync;
use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{add_use::add_use, visit_arguments::visit_arguments, vm_exit_op::vm_exit_op},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    ir_function::IrFunction, ir_inst::IrInst, ir_op::IrOp,
    remove_dead_store_state::RemoveDeadStoreState, vm_exit_store_info::VmExitStoreInfo,
    vm_exit_store_record::VmExitStoreRecord, vm_exit_sync_info::VmExitSyncInfo,
  },
};

// IrData.h: handler.pcpos sentinel for the entry guard
const K_VM_EXIT_ENTRY_GUARD_PC: u32 = (1u32 << 28) - 1;

// IrVisitUseDef-style helper: capture a store instruction into the exit sync record and add uses for its operands
fn record_store(function: *mut IrFunction, store_info: &mut VmExitStoreInfo, inst_idx: u32) {
  let backup = unsafe { (&(*function).instructions)[inst_idx as usize].clone() };
  store_info
    .stores
    .push_back(VmExitStoreRecord { inst_idx, backup });

  let inst_ptr: *mut IrInst = unsafe { &mut (&mut (*function).instructions)[inst_idx as usize] };
  visit_arguments(unsafe { &mut *inst_ptr }, |op| {
    add_use(unsafe { &mut *function }, op);
  });
}

impl RemoveDeadStoreState {
  // When checking control flow, such as exit to fallback blocks
  pub fn check_live_ins(&mut self, op: IrOp, inst_idx: u32, record_vm_exit_sync: bool) {
    let function: *mut IrFunction = self.function;

    if op.kind() == IrOpKind::VmExit {
      if LuauCodegenVmExitSync.get()
        && record_vm_exit_sync
        && vm_exit_op(op) != K_VM_EXIT_ENTRY_GUARD_PC
      {
        let sync_info: *mut VmExitSyncInfo =
          unsafe { (*function).vm_exit_info.get_or_insert(inst_idx) };
        CODEGEN_ASSERT!(unsafe { (*sync_info).reg_stores.is_empty() });

        unsafe {
          (*sync_info).vm_exit = op;
        }

        self.recorded_vm_exit_syncs.push(inst_idx);

        // Reverse order so that we capture lexically close VM registers first
        let max_reg = self.max_reg;
        let mut i = max_reg;
        while i >= 0 {
          let tag_idx = self.info[i as usize].tag_inst_idx;
          let value_idx = self.info[i as usize].value_inst_idx;
          let tvalue_idx = self.info[i as usize].tvalue_inst_idx;
          let ignore_at_exit = self.info[i as usize].ignore_at_exit;
          let maybe_gco = self.info[i as usize].maybe_gco;

          // If value cannot be propagated into the exit, store must remain as used by the exit
          if (tag_idx != !0u32 && self.non_propagating_store.contains(&tag_idx))
            || (value_idx != !0u32 && self.non_propagating_store.contains(&value_idx))
            || (tvalue_idx != !0u32 && self.non_propagating_store.contains(&tvalue_idx))
          {
            self.use_reg(i as u8);
            i -= 1;
            continue;
          }

          if ignore_at_exit && !maybe_gco {
            i -= 1;
            continue;
          }

          if unsafe { (*sync_info).reg_stores.len() } >= 16 {
            self.use_reg(i as u8);
            i -= 1;
            continue;
          }

          let has_partial_overlap = (tag_idx != !0u32 || value_idx != !0u32) && tvalue_idx != !0u32;

          if has_partial_overlap {
            self.use_reg(i as u8);
            i -= 1;
            continue;
          }

          let mut store_info = VmExitStoreInfo {
            reg: i as u8,
            ..Default::default()
          };

          if tag_idx != !0u32 {
            CODEGEN_ASSERT!(tvalue_idx == !0u32);
            record_store(function, &mut store_info, tag_idx);
          }

          if value_idx != !0u32 {
            CODEGEN_ASSERT!(tvalue_idx == !0u32);
            record_store(function, &mut store_info, value_idx);
          }

          if tvalue_idx != !0u32 {
            CODEGEN_ASSERT!(tag_idx == !0u32 && value_idx == !0u32);

            let store_cmd = unsafe { (&(*function).instructions)[tvalue_idx as usize].cmd };
            let nil_tag_store = store_cmd == IrCmd::StoreTag && {
              let tag_op = unsafe { (&(*function).instructions)[tvalue_idx as usize].ops[1] };
              let t = unsafe { (*function).tag_op(tag_op) };
              t == LuaType::Nil as u8
            };
            CODEGEN_ASSERT!(
              store_cmd == IrCmd::StoreSplitTvalue
                || store_cmd == IrCmd::StoreTvalue
                || store_cmd == IrCmd::StoreVector
                || nil_tag_store
            );

            record_store(function, &mut store_info, tvalue_idx);
          }

          if !store_info.stores.empty() {
            unsafe {
              (*sync_info).reg_stores.push(store_info);
            }
          }

          i -= 1;
        }
      } else {
        let max_reg = self.max_reg;
        for i in 0..=max_reg {
          let ignore_at_exit = self.info[i as usize].ignore_at_exit;
          let maybe_gco = self.info[i as usize].maybe_gco;

          if ignore_at_exit && !maybe_gco {
            continue;
          }

          self.use_reg(i as u8);
        }

        self.has_gco_to_clear = false;
      }
    } else if op.kind() == IrOpKind::Block {
      if (op.index() as usize) < unsafe { (*function).cfg.r#in.len() } {
        let max_reg = self.max_reg;
        for i in 0..=max_reg {
          let in_ref = unsafe { &(&(*function).cfg.r#in)[op.index() as usize] };
          let is_in = (in_ref.regs[i as usize / 64] & (1u64 << (i as usize % 64))) != 0
            || (in_ref.vararg_seq && i >= in_ref.vararg_start as i32);

          if is_in {
            self.use_reg(i as u8);
          }
        }
      } else {
        self.read_all_regs();
      }
    } else if op.kind() == IrOpKind::Undef {
      // Nothing to do for a debug abort
    } else {
      CODEGEN_ASSERT!(false); // unexpected jump target type
    }
  }
}
