use alloc::{string::String, vec::Vec};

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{enums::abix_64::ABIX64, records::label::Label};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct AssemblyBuilderX64 {
  pub data: Vec<u8>,
  pub code: Vec<u8>,
  pub text: String,
  pub log_text: bool,
  pub abi: ABIX64,
  pub features: u32,
  pub(crate) next_label: u32,
  pub(crate) pending_labels: Vec<Label>,
  pub(crate) label_locations: Vec<u32>,
  pub(crate) const_cache_32: DenseHashMap<u32, i32>,
  pub(crate) const_cache_64: DenseHashMap<u64, i32>,
  pub(crate) finalized: bool,
  pub(crate) data_pos: usize,
  pub(crate) code_pos: *mut u8,
  pub(crate) code_end: *mut u8,
  pub(crate) instruction_count: u32,
}

impl AssemblyBuilderX64 {
  pub fn get_label_offset(&self, label: &Label) -> u32 {
    ulua_common::LUAU_ASSERT!(label.location != !0u32);
    label.location
  }
}
