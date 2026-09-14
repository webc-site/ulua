use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind, ir_value_kind::IrValueKind},
  functions::get_cmd_value_kind::get_cmd_value_kind,
  records::{
    ir_op::IrOp, remove_dead_store_state::RemoveDeadStoreState,
    store_location_hint::StoreLocationHint,
  },
};

impl RemoveDeadStoreState {
  pub fn record_hint_before_kill(&mut self, store_inst_idx: u32) {
    let function = unsafe { &mut *self.function };

    let store_cmd = function.instructions[store_inst_idx as usize].cmd;

    let dest: IrOp = function.instructions[store_inst_idx as usize].ops[0];

    if dest.kind() != IrOpKind::VmReg {
      return;
    }

    let value: IrOp;
    let mut kind = IrValueKind::Unknown;

    match store_cmd {
      IrCmd::StoreDouble => {
        value = function.instructions[store_inst_idx as usize].ops[1];
        kind = IrValueKind::Double;
      }
      IrCmd::StoreInt => {
        value = function.instructions[store_inst_idx as usize].ops[1];
        kind = IrValueKind::Int;
      }
      IrCmd::StoreInt64 => {
        value = function.instructions[store_inst_idx as usize].ops[1];
        kind = IrValueKind::Int64;
      }
      IrCmd::StorePointer => {
        value = function.instructions[store_inst_idx as usize].ops[1];
        kind = IrValueKind::Pointer;
      }
      IrCmd::StoreTvalue => {
        value = function.instructions[store_inst_idx as usize].ops[1];
        kind = IrValueKind::Tvalue;
      }
      IrCmd::StoreSplitTvalue => {
        value = function.instructions[store_inst_idx as usize].ops[2];

        if value.kind() == IrOpKind::Inst {
          let inst_cmd = function.inst_op(value).cmd;
          kind = get_cmd_value_kind(inst_cmd);
        }

        if kind == IrValueKind::Unknown {
          return;
        }
      }
      IrCmd::StoreVector => {
        // multi-component, not useful as a single-value restore hint
        return;
      }
      _ => return,
    }

    if value.kind() != IrOpKind::Inst {
      return;
    }

    function.record_store_location_hint(
      store_inst_idx,
      StoreLocationHint {
        op: dest,
        inst_idx: value.index(),
        kind,
      },
    );
  }
}
