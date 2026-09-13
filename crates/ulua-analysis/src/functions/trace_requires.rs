use alloc::vec::Vec;
use core::ptr::null_mut;

use ulua_ast::{records::ast_stat_block::AstStatBlock, visit::AstVisitable};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::{
    file_resolver::FileResolver, require_trace_result::RequireTraceResult,
    require_tracer::RequireTracer, type_check_limits::TypeCheckLimits,
  },
  type_aliases::module_name_type::ModuleName,
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn trace_requires(
  file_resolver: *mut FileResolver,
  root: *mut AstStatBlock,
  current_module_name: ModuleName,
  limits: &TypeCheckLimits,
) -> RequireTraceResult {
  let mut result = RequireTraceResult {
    exprs: DenseHashMap::new(null_mut()),
    require_list: Vec::new(),
  };

  let mut tracer = RequireTracer::new(
    &mut result as *mut RequireTraceResult,
    file_resolver,
    current_module_name,
  );

  unsafe {
    if let Some(root_ref) = root.as_mut() {
      root_ref.visit(&mut tracer);
    }
  }

  tracer.process(limits);

  result
}
