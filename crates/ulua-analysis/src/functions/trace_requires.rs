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

/// C++ `Luau::traceRequires`：`FileResolver*` 参数以 trait object 引用承载；
/// C++ 侧的 `AstStatBlock* root` 是非 const 指针（`AstNode::visit` 需要非 const
/// `this`），这里收窄为 `&mut` 借用，解引用责任留在调用方（AST arena 边界），
/// 因此本函数不再 `unsafe`。
pub fn trace_requires(
  file_resolver: &mut dyn FileResolver,
  root: &mut AstStatBlock,
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

  root.visit(&mut tracer);

  tracer.process(limits);

  result
}
