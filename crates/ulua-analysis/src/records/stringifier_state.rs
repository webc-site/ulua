use alloc::string::String;
use core::ffi::c_void;

use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  records::{set::Set, to_string_options::ToStringOptions, to_string_result::ToStringResult},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
#[derive(Debug, Clone)]
pub struct StringifierState {
  pub opts: *mut ToStringOptions,
  pub result: *mut ToStringResult,
  pub cycle_names: DenseHashMap<TypeId, String>,
  pub cycle_tp_names: DenseHashMap<TypePackId, String>,
  pub seen: Set<*mut c_void>,
  pub used_names: DenseHashSet<String>,
  pub indentation: usize,
  pub exhaustive: bool,
  pub ignore_synthetic_name: bool,
  pub previous_name_index: i32,
}
