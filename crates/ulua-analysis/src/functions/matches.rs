use crate::enums::control_flow::ControlFlow;

pub fn matches(a: ControlFlow, b: ControlFlow) -> bool {
  a.intersects(b)
}
