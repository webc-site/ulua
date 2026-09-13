use ulua_common::FFlag::LuauCodegenExtraTableOpts;

use crate::records::{const_prop_state::ConstPropState, ir_op::IrOp, register_info::RegisterInfo};

impl ConstPropState {
  pub fn invalidate_register_info_bool_bool(
    &mut self,
    reg: &mut RegisterInfo,
    invalidate_tag: bool,
    invalidate_value: bool,
  ) {
    if invalidate_tag {
      reg.tag = 0xff;
    }

    if invalidate_value {
      reg.value = IrOp::default();

      if !LuauCodegenExtraTableOpts.get() {
        reg.known_not_readonly_deprecated = false;
        reg.known_no_metatable_deprecated = false;
        reg.known_table_array_size_deprecated = -1;
      }
    }

    reg.version += 1;
  }
}
