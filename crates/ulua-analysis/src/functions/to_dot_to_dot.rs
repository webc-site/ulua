use core::ptr::null;
extern crate alloc;

use alloc::string::String;

use crate::{
  records::{state_dot::StateDot, to_dot_options::ToDotOptions},
  type_aliases::type_id::TypeId,
};

pub fn to_dot(ty: TypeId, opts: &ToDotOptions) -> String {
  to_dot_impl(ty, *opts)
}

// 内部实现：裸指针解引用由 unsafe 块承担（私有可见性，不触发签名契约告警）。
fn to_dot_impl(ty: TypeId, opts: ToDotOptions) -> String {
  let mut state = StateDot::new(opts);

  state.result = String::from("digraph graphname {\n");
  unsafe { state.visit_child_type_id_i32_c_char(ty, 0, null()) };
  state.result += "}";

  state.result
}

pub use crate::functions::to_dot_to_dot::to_dot as to_dot_type_id_to_dot_options;
