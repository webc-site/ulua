//! cpp `SubtypingResult& with_sub_component(Component)`——同形核心见
//! [`crate::functions::prepend_reasoning_component`]（与
//! `with_super_component` 仅路径侧不同）。
use crate::{
  functions::prepend_reasoning_component::{ReasoningSide, prepend_component},
  records::subtyping_result::SubtypingResult,
  type_aliases::component::Component,
};

impl SubtypingResult {
  pub fn with_sub_component(&mut self, component: Component) -> &mut Self {
    prepend_component(&mut self.reasoning, component, ReasoningSide::Sub);
    self
  }
}
