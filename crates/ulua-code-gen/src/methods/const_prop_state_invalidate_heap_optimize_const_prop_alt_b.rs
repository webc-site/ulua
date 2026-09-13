use ulua_common::FFlag::LuauCodegenExtraTableOpts;

use crate::{
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{const_prop_state::ConstPropState, register_info::RegisterInfo},
};

impl ConstPropState {
  pub fn invalidate_heap_register_info(&mut self, reg: &mut RegisterInfo) {
    CODEGEN_ASSERT!(!LuauCodegenExtraTableOpts.get());

    reg.known_not_readonly_deprecated = false;
    reg.known_no_metatable_deprecated = false;
    reg.known_table_array_size_deprecated = -1;
  }
}
