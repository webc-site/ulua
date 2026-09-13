use alloc::vec::Vec;
use core::ptr::null_mut;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::{
    file_resolver::FileResolver, require_trace_result::RequireTraceResult,
    require_tracer::RequireTracer,
  },
  type_aliases::module_name_type::ModuleName,
};
impl RequireTracer {
  pub fn new(
    result: *mut RequireTraceResult,
    file_resolver: *mut FileResolver,
    current_module_name: ModuleName,
  ) -> Self {
    RequireTracer {
      result,
      file_resolver,
      current_module_name,
      locals: DenseHashMap::new(null_mut()),
      work: Vec::new(),
      require_calls: Vec::new(),
    }
  }
}
