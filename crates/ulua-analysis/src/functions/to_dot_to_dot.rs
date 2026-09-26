use alloc::string::String;

use crate::{
  records::{state_dot::StateDot, to_dot_options::ToDotOptions},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

pub fn to_dot(ty: TypeId, opts: &ToDotOptions) -> String {
  let mut state = StateDot::new(*opts);
  state.result = String::from("digraph graphname {\n");
  state.visit_child_type_id(ty, 0, None);
  state.result += "}";
  state.result
}

pub fn to_dot_type_pack_id(tp: TypePackId, opts: &ToDotOptions) -> String {
  let mut state = StateDot::new(*opts);
  state.result = String::from("digraph graphname {\n");
  state.visit_child_type_pack_id(tp, 0, None);
  state.result.push('}');
  state.result
}

pub fn to_dot_default_opts_type_id(ty: TypeId) -> String {
  to_dot(ty, &ToDotOptions::default())
}

pub fn to_dot_default_opts_type_pack_id(tp: TypePackId) -> String {
  to_dot_type_pack_id(tp, &ToDotOptions::default())
}
