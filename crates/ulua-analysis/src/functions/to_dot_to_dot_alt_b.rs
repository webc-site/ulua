use core::ptr::null;
extern crate alloc;

use alloc::string::String;

use crate::{
  records::{state_dot::StateDot, to_dot_options::ToDotOptions},
  type_aliases::type_pack_id::TypePackId,
};

pub fn to_dot(tp: TypePackId, opts: &ToDotOptions) -> String {
  to_dot_impl(tp, *opts)
}

// 内部实现：裸指针解引用由 unsafe 块承担（私有可见性，不触发签名契约告警）。
fn to_dot_impl(tp: TypePackId, opts: ToDotOptions) -> String {
  let mut state = StateDot::new(opts);

  state.result = String::from("digraph graphname {\n");
  unsafe { state.visit_child_type_pack_id_i32_c_char(tp, 0, null()) };
  state.result.push('}');

  state.result
}

pub use crate::functions::to_dot_to_dot_alt_b::to_dot as to_dot_type_pack_id_to_dot_options;
