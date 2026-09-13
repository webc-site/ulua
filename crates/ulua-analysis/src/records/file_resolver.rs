use alloc::{string::String, sync::Arc};
use core::fmt::{Debug, Formatter, Result};

use ulua_ast::records::ast_expr::AstExpr;

use crate::{
  records::{
    module_info::ModuleInfo, require_suggester::RequireSuggester, source_code::SourceCode,
    type_check_limits::TypeCheckLimits,
  },
  type_aliases::module_name_type::ModuleName,
};
#[repr(C)]
pub struct FileResolver {
  pub vtable: FileResolverVtable,
  pub require_suggester: Option<Arc<RequireSuggester>>,
}

#[derive(Clone, Copy)]
pub struct FileResolverVtable {
  pub read_source: unsafe fn(*mut FileResolver, name: &ModuleName) -> Option<SourceCode>,
  pub resolve_module: unsafe fn(
    *mut FileResolver,
    context: *const ModuleInfo,
    expr: *mut AstExpr,
    limits: &TypeCheckLimits,
  ) -> Option<ModuleInfo>,
  pub get_human_readable_module_name: unsafe fn(*const FileResolver, name: &ModuleName) -> String,
  pub get_environment_for_module:
    unsafe fn(*const FileResolver, name: &ModuleName) -> Option<String>,
}

pub(crate) unsafe fn file_resolver_resolve_module_default(
  _this: *mut FileResolver,
  _context: *const ModuleInfo,
  _expr: *mut AstExpr,
  _limits: &TypeCheckLimits,
) -> Option<ModuleInfo> {
  None
}

pub(crate) unsafe fn file_resolver_get_human_readable_module_name_default(
  _this: *const FileResolver,
  name: &ModuleName,
) -> String {
  name.clone()
}

pub(crate) unsafe fn file_resolver_get_environment_for_module_default(
  _this: *const FileResolver,
  _name: &ModuleName,
) -> Option<String> {
  None
}

impl FileResolver {
  /// # Safety
  /// 调用方须保证 `this` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn read_source(this: *mut FileResolver, name: &ModuleName) -> Option<SourceCode> {
    unsafe { ((*this).vtable.read_source)(this, name) }
  }

  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn resolve_module(
    this: *mut FileResolver,
    context: *const ModuleInfo,
    expr: *mut AstExpr,
    limits: &TypeCheckLimits,
  ) -> Option<ModuleInfo> {
    unsafe { ((*this).vtable.resolve_module)(this, context, expr, limits) }
  }

  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn get_human_readable_module_name(
    this: *const FileResolver,
    name: &ModuleName,
  ) -> String {
    unsafe { ((*this).vtable.get_human_readable_module_name)(this, name) }
  }

  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn get_environment_for_module(
    this: *const FileResolver,
    name: &ModuleName,
  ) -> Option<String> {
    unsafe { ((*this).vtable.get_environment_for_module)(this, name) }
  }
}

impl Debug for FileResolver {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("FileResolver")
      .field("require_suggester", &self.require_suggester)
      .finish()
  }
}
