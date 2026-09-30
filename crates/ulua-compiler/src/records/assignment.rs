use crate::records::{compiler::K_INVALID_REG, l_value::LValue};

#[derive(Debug, Clone)]
pub(crate) struct Assignment {
  pub(crate) lvalue: LValue,
  pub(crate) conflict_reg: u8,
  pub(crate) value_reg: u8,
}

impl Default for Assignment {
  fn default() -> Self {
    Self {
      lvalue: LValue::default(),
      conflict_reg: K_INVALID_REG,
      value_reg: K_INVALID_REG,
    }
  }
}
