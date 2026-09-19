extern crate alloc;

use alloc::string::String;

use crate::{
  records::{state_dot::StateDot, to_dot_options::ToDotOptions},
  type_aliases::type_id::TypeId,
};

pub fn to_dot(ty: TypeId, opts: &ToDotOptions) -> String {
  let mut state = StateDot::new(*opts);
  state.result = String::from("digraph graphname {\n");
  state.visit_child_type_id(ty, 0, None);
  state.result += "}";
  state.result
}
