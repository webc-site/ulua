use core::ops::BitAnd;

use crate::enums::control_flow::ControlFlow;
pub fn operator_bitand(a: ControlFlow, b: ControlFlow) -> ControlFlow {
  ControlFlow::from_bits(a as u32 & b as u32)
}

impl BitAnd for ControlFlow {
  type Output = Self;

  fn bitand(self, rhs: Self) -> Self::Output {
    operator_bitand(self, rhs)
  }
}
