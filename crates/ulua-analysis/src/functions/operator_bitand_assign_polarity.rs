use core::ops::BitAndAssign;

use crate::{enums::polarity::Polarity, functions::operator_bitand_polarity::operator_bitand};
pub fn operator_bitand_assign(lhs: &mut Polarity, rhs: Polarity) -> &mut Polarity {
  *lhs = operator_bitand(*lhs, rhs);
  lhs
}

impl BitAndAssign for Polarity {
  fn bitand_assign(&mut self, rhs: Self) {
    operator_bitand_assign(self, rhs);
  }
}
