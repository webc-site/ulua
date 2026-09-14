use crate::enums::control_flow::ControlFlow;

pub fn matches(a: ControlFlow, b: ControlFlow) -> bool {
  (a as u32 & b as u32) != 0
}
