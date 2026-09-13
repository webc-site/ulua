use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::substitute::substitute,
  macros::{codegen_assert::CODEGEN_ASSERT, op_a::op_a},
  records::{const_prop_state::ConstPropState, ir_builder::IrBuilder, ir_inst::IrInst},
};

impl ConstPropState {
  pub fn substitute_tag_load_with_t_value_data(
    &mut self,
    build: &mut IrBuilder,
    load_inst: &mut IrInst,
  ) -> bool {
    CODEGEN_ASSERT!(op_a(load_inst).kind() == IrOpKind::VmReg);

    if let Some(prev_idx) =
      self.get_previous_versioned_load_index(IrCmd::LoadTvalue, op_a(load_inst))
      && let Some(tag) = self.inst_tag.find(&unsafe { *prev_idx })
      && *tag != 0xff
    {
      let replacement = build.const_tag(*tag);
      unsafe { substitute(&mut *self.function, load_inst, replacement) };
      return true;
    }

    false
  }
}
