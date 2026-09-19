use ulua_common::fflag::LuauCodegenExtraTableOpts;

use crate::records::{
  const_prop_state::ConstPropState, ir_data::K_UNKNOWN_TAG, ir_op::IrOp,
  register_info::RegisterInfo,
};

impl ConstPropState {
  pub fn invalidate_register_info_bool_bool(
    &mut self,
    reg: &mut RegisterInfo,
    invalidate_tag: bool,
    invalidate_value: bool,
  ) {
    if invalidate_tag {
      reg.tag = K_UNKNOWN_TAG;
    }

    if invalidate_value {
      reg.value = IrOp::default();

      if !LuauCodegenExtraTableOpts.get() {
        reg.known_table_array_size = -1;
      }
    }

    reg.version += 1;
  }
}
