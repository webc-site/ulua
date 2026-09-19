extern crate alloc;

use alloc::string::String;

use crate::{
  records::{state_dot::StateDot, to_dot_options::ToDotOptions},
  type_aliases::type_pack_id::TypePackId,
};

pub fn to_dot(tp: TypePackId, opts: &ToDotOptions) -> String {
  let mut state = StateDot::new(*opts);
  state.result = String::from("digraph graphname {\n");
  state.visit_child_type_pack_id(tp, 0, None);
  state.result.push('}');
  state.result
}
