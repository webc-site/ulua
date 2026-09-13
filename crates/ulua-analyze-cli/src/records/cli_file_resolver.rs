//! Port of `struct CliFileResolver : Luau::FileResolver` (`CLI/src/Analyze.cpp:160-229`).
//!
//! A concrete `FileResolver` subclass: `#[repr(C)]` with `base: FileResolver`
//! first so a `*mut FileResolver` (the vtable receiver) can be cast back to
//! `*mut CliFileResolver`. The three overridden virtuals (`readSource`,
//! `resolveModule`, `getHumanReadableModuleName`) are wired to thunks that
//! delegate to the inherent methods; `getEnvironmentForModule` is not overridden
//! and uses the base default.

use alloc::string::String;

use ulua_analysis::{
  records::{
    file_resolver::{FileResolver, FileResolverVtable},
    module_info::ModuleInfo,
    source_code::SourceCode,
    type_check_limits::TypeCheckLimits,
  },
  type_aliases::module_name_type::ModuleName,
};
use ulua_ast::records::ast_expr::AstExpr;

use crate::methods::cli_file_resolver_get_human_readable_module_name::cli_file_resolver_get_human_readable_module_name;
#[repr(C)]
pub struct CliFileResolver {
  pub base: FileResolver,
}

/// `std::optional<SourceCode> readSource(const ModuleName&)` thunk.
///
/// # Safety
/// `this` must point at the `base` subobject of a live `CliFileResolver`.
pub(crate) unsafe fn cli_file_resolver_read_source_thunk(
  this: *mut FileResolver,
  name: &ModuleName,
) -> Option<SourceCode> {
  unsafe {
    let this = this as *mut CliFileResolver;
    (*this).read_source(name)
  }
}

/// `std::optional<ModuleInfo> resolveModule(const ModuleInfo*, AstExpr*, const TypeCheckLimits&)` thunk.
///
/// # Safety
/// `this` must point at the `base` subobject of a live `CliFileResolver`.
pub(crate) unsafe fn cli_file_resolver_resolve_module_thunk(
  this: *mut FileResolver,
  context: *const ModuleInfo,
  expr: *mut AstExpr,
  limits: &TypeCheckLimits,
) -> Option<ModuleInfo> {
  unsafe {
    let this = this as *mut CliFileResolver;
    (*this).resolve_module(context, expr, limits)
  }
}

/// `std::string getHumanReadableModuleName(const ModuleName&) const` thunk.
///
/// # Safety
/// `this` must point at the `base` subobject of a live `CliFileResolver`.
pub(crate) unsafe fn cli_file_resolver_get_human_readable_module_name_thunk(
  this: *const FileResolver,
  name: &ModuleName,
) -> String {
  unsafe {
    let this = this as *const CliFileResolver;
    cli_file_resolver_get_human_readable_module_name(this, name)
  }
}

/// `getEnvironmentForModule` is not overridden by `CliFileResolver`; this mirrors
/// the `FileResolver` base default (`return std::nullopt;`).
///
/// # Safety
/// `this` must point at a live `FileResolver` (the base subobject).
unsafe fn cli_file_resolver_get_environment_for_module_default(
  _this: *const FileResolver,
  _name: &ModuleName,
) -> Option<String> {
  None
}

impl CliFileResolver {
  pub fn new() -> Self {
    let vtable = FileResolverVtable {
      read_source: cli_file_resolver_read_source_thunk,
      resolve_module: cli_file_resolver_resolve_module_thunk,
      get_human_readable_module_name: cli_file_resolver_get_human_readable_module_name_thunk,
      get_environment_for_module: cli_file_resolver_get_environment_for_module_default,
    };

    CliFileResolver {
      base: FileResolver {
        vtable,
        require_suggester: None,
      },
    }
  }
}

impl Default for CliFileResolver {
  fn default() -> Self {
    Self::new()
  }
}
