mod _inner {
  use crate::enums::luau_opcode::LuauOpcode;

  pub fn is_skip_c(op: LuauOpcode) -> bool {
    matches!(op, LuauOpcode::LOP_LOADB)
  }
}

pub use _inner::{is_skip_c, is_skip_c as isSkipC};
