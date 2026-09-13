use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  macros::{codegen_assert::CODEGEN_ASSERT, op_b::op_b, op_c::op_c, op_d::op_d},
  records::{const_prop_state::ConstPropState, ir_builder::IrBuilder, ir_op::IrOp},
};

impl ConstPropState {
  pub fn find_substitute_component_load_from_store_vector(
    &mut self,
    _build: &mut IrBuilder,
    vm_reg: IrOp,
    offset: i32,
  ) -> Option<IrOp> {
    let versioned_load = self.versioned_vm_reg_load_ir_cmd_ir_op(IrCmd::LoadFloat, vm_reg);

    if let Some(prev_idx) = self.get_previous_inst_index(&versioned_load) {
      let function = unsafe { &mut *self.function };
      let store = &function.instructions[unsafe { *prev_idx } as usize];

      CODEGEN_ASSERT!(store.cmd == IrCmd::StoreVector);

      let arg_op = if offset == 0 {
        op_b(store.clone())
      } else if offset == 4 {
        op_c(store.clone())
      } else if offset == 8 {
        op_d(store.clone())
      } else {
        return None;
      };

      let function = unsafe { &mut *self.function };
      let arg = function.as_inst_op(arg_op);

      if !arg.is_null() {
        let arg_cmd = unsafe { &*arg }.cmd;
        if arg_cmd == IrCmd::LoadFloat
          || arg_cmd == IrCmd::BufferReadf32
          || arg_cmd == IrCmd::NumToFloat
          || arg_cmd == IrCmd::UintToFloat
        {
          return Some(arg_op);
        }
      } else if arg_op.kind() == IrOpKind::Constant {
        let double_val = function.double_op(arg_op);
        return Some(_build.const_double(double_val));
      }
    }

    None
  }
}
