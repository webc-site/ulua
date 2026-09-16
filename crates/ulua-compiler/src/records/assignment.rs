use crate::records::l_value::LValue;

#[derive(Debug, Clone)]
pub struct Assignment {
  pub(crate) lvalue: LValue,
  pub(crate) conflict_reg: u8,
  pub(crate) value_reg: u8,
}

impl Assignment {
  pub(crate) const K_INVALID_REG: u8 = 255;
}

impl Default for Assignment {
  fn default() -> Self {
    Self {
      lvalue: LValue::default(),
      conflict_reg: Self::K_INVALID_REG,
      value_reg: Self::K_INVALID_REG,
    }
  }
}
