use alloc::{string::String, vec::Vec};
use core::ptr::null_mut;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{enums::abix_64::ABIX64, records::assembly_builder_x_64::AssemblyBuilderX64};

impl AssemblyBuilderX64 {
  pub fn assembly_builder_x_64_bool_abix_64_i32(
    log_text: bool,
    abi: ABIX64,
    features: u32,
  ) -> Self {
    let mut builder = Self {
      data: Vec::new(),
      code: Vec::new(),
      text: String::new(),
      log_text,
      abi,
      features,
      next_label: 1,
      pending_labels: Vec::new(),
      label_locations: Vec::new(),
      const_cache_32: DenseHashMap::new(!0u32),
      const_cache_64: DenseHashMap::new(!0u64),
      finalized: false,
      data_pos: 0,
      code_pos: null_mut(),
      code_end: null_mut(),
      instruction_count: 0,
    };

    builder.data.resize(4096, 0);
    builder.data_pos = builder.data.len();

    builder.code.resize(4096, 0);
    builder.code_pos = builder.code.as_mut_ptr();
    builder.code_end = unsafe { builder.code_pos.add(builder.code.len()) };

    builder
  }
}
