use crate::enums::control_flow::ControlFlow;

pub fn operator_bitor(a: ControlFlow, b: ControlFlow) -> ControlFlow {
  ControlFlow::from_bits(a as u32 | b as u32)
}
