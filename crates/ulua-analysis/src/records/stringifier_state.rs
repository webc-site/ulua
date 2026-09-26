use alloc::string::String;

use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  records::{
    arena_handle::alias, set::Set, to_string_options::ToStringOptions,
    to_string_result::ToStringResult,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
#[derive(Debug, Clone)]
pub struct StringifierState {
  pub opts: *mut ToStringOptions,
  pub result: *mut ToStringResult,
  pub cycle_names: DenseHashMap<TypeId, String>,
  pub cycle_tp_names: DenseHashMap<TypePackId, String>,
  pub seen: Set<*mut ()>,
  pub used_names: DenseHashSet<String>,
  pub indentation: usize,
  pub exhaustive: bool,
  pub ignore_synthetic_name: bool,
  pub previous_name_index: i32,
}

impl StringifierState {
  /// §2 裸指针收口单点：`opts` 由入口 `to_string*` 构造期以入参 `&mut
  /// ToStringOptions` 裸化注入，入参借用覆盖整个会话；解引用只发生在
  /// [`alias`] 一处。
  pub(crate) fn opts_mut(&mut self) -> &'static mut ToStringOptions {
    alias(self.opts)
  }

  /// §2 裸指针收口单点：`result` 由入口 `to_string*` 构造期以自身局部
  /// `ToStringResult` 的 `&mut` 裸化注入，局部存活至函数返回；解引用只发生在
  /// [`alias`] 一处。
  pub(crate) fn result_mut(&mut self) -> &'static mut ToStringResult {
    alias(self.result)
  }
}
