use ulua_common::fflag;

use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind, ir_value_kind::IrValueKind},
  functions::vm_reg_op::vm_reg_op,
  macros::{op_a::op_a, op_b_ref::op_b_ref, op_c_ref::op_c_ref},
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
        let source_op = op_b_ref(inst);

        if source_op.kind() == IrOpKind::Inst {
          let function = unsafe { &mut *self.function };
          let can_remat_args = can_rematerialize_arguments_at(function, source_op.index());
          let source = &function.instructions[source_op.index() as usize];

          if source.last_use != inst_idx || can_remat_args {
            self.record_restore_op(source_op.index(), op_a(inst));
          }
        }
      }

      IrCmd::StoreSplitTvalue => {
        let source_op = op_c_ref(inst);

        if source_op.kind() == IrOpKind::Inst {
          let function = unsafe { &mut *self.function };
          let can_remat_args = can_rematerialize_arguments_at(function, source_op.index());
          let source = &function.instructions[source_op.index() as usize];

          if source.last_use != inst_idx || can_remat_args {
            self.record_restore_op(source_op.index(), op_a(inst));
          }
        }
      }

      IrCmd::NumToUint | IrCmd::NumToInt => {
        let arg = op_a(inst);

        if fflag::LuauCodegenForwardRematerialize.get() && arg.kind() == IrOpKind::Inst {
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

/// cpp IrValueLocationTracking.cpp:29-32 `canBeRematerialized`。
/// 以索引而非指针访问 instructions：借用约束下无法把 `&mut self.function`
/// 内的引用同时传入需要再次可变借用的方法（`IrInst` 指针身份被
/// `get_inst_index` 的断言依赖，不能传 clone）。
fn can_be_rematerialized(cmd: IrCmd) -> bool {
  cmd == IrCmd::UintToNum || cmd == IrCmd::IntToNum
}

/// cpp IrValueLocationTracking.cpp:34-47 `canRematerializeArguments`。
fn can_rematerialize_arguments_at(function: &mut IrFunction, inst_idx: u32) -> bool {
  let inst = &mut function.instructions[inst_idx as usize];

  if can_be_rematerialized(inst.cmd) && op_a(inst).kind() == IrOpKind::Inst {
    let dep_inst_idx = op_a(inst).index();

    if function.instructions[dep_inst_idx as usize].last_use != inst_idx {
      return true;
    }
  }

  false
}
