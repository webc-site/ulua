use ulua_common::FFlag;

use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind, ir_value_kind::IrValueKind},
  functions::vm_reg_op::vm_reg_op,
  macros::{op_a::op_a, op_b::op_b, op_c::op_c},
  records::{
    ir_data::K_INVALID_INST_IDX, ir_function::IrFunction, ir_inst::IrInst,
    ir_value_location_tracking::IrValueLocationTracking,
    value_restore_location::ValueRestoreLocation,
  },
};

impl IrValueLocationTracking {
  pub fn after_inst_lowering(&mut self, inst: &mut IrInst, inst_idx: u32) {
    match inst.cmd {
      IrCmd::LoadTag
      | IrCmd::LoadPointer
      | IrCmd::LoadDouble
      | IrCmd::LoadInt
      | IrCmd::LoadInt64
      | IrCmd::LoadTvalue => {
        if op_a(inst).kind() == IrOpKind::VmReg {
          self.invalidate_restore_op(op_a(inst), false);
        }

        self.record_restore_op(inst_idx, op_a(inst));
      }

      IrCmd::StorePointer
      | IrCmd::StoreDouble
      | IrCmd::StoreInt
      | IrCmd::StoreInt64
      | IrCmd::StoreTvalue => {
        let source_op = op_b(inst.clone());

        if source_op.kind() == IrOpKind::Inst {
          let function = unsafe { &mut *self.function };
          let source = function.instructions[source_op.index() as usize].clone();
          let can_remat_args = can_rematerialize_arguments_at(function, source_op.index());

          if source.last_use != inst_idx || can_remat_args {
            self.record_restore_op(source_op.index(), op_a(inst));
          }
        }
      }

      IrCmd::StoreSplitTvalue => {
        let source_op = op_c(inst.clone());

        if source_op.kind() == IrOpKind::Inst {
          let function = unsafe { &mut *self.function };
          let source = function.instructions[source_op.index() as usize].clone();
          let can_remat_args = can_rematerialize_arguments_at(function, source_op.index());

          if source.last_use != inst_idx || can_remat_args {
            self.record_restore_op(source_op.index(), op_a(inst));
          }
        }
      }

      IrCmd::NumToUint | IrCmd::NumToInt => {
        let arg = op_a(inst);

        if FFlag::LuauCodegenForwardRematerialize.get() && arg.kind() == IrOpKind::Inst {
          let function = unsafe { &mut *self.function };
          let owner_loc = function.find_restore_location_u32_bool(arg.index(), true);

          if owner_loc.op.kind() == IrOpKind::VmReg
            && owner_loc.kind == IrValueKind::Double
            && owner_loc.conversion_cmd == IrCmd::NOP
            && !owner_loc.lazy
          {
            let reg = vm_reg_op(owner_loc.op) as usize;
            let captured = (function.cfg.captured.regs[reg / 64] & (1u64 << (reg % 64))) != 0;

            if !captured && self.vm_reg_dependent[reg] == K_INVALID_INST_IDX {
              let forward_cmd = if inst.cmd == IrCmd::NumToUint {
                IrCmd::UintToNum
              } else {
                IrCmd::IntToNum
              };

              function.record_restore_location(
                inst_idx,
                ValueRestoreLocation {
                  op: owner_loc.op,
                  kind: IrValueKind::Double,
                  conversion_cmd: forward_cmd,
                  lazy: false,
                },
              );

              self.vm_reg_dependent[reg] = inst_idx;
            }
          }
        }
      }

      _ => {}
    }
  }
}

fn can_rematerialize_arguments_at(function: &mut IrFunction, inst_idx: u32) -> bool {
  let mut inst = function.instructions[inst_idx as usize].clone();

  if (inst.cmd == IrCmd::UintToNum || inst.cmd == IrCmd::IntToNum)
    && op_a(&mut inst).kind() == IrOpKind::Inst
  {
    let dep_inst_idx = op_a(&mut inst).index();

    if function.instructions[dep_inst_idx as usize].last_use != inst_idx {
      return true;
    }
  }

  false
}
