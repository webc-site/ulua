//! `DemoFileResolver::resolveModule` (`CLI/src/Web.cpp:27-33`).
//!
//! ```cpp
//! std::optional<Luau::ModuleInfo> resolveModule(const Luau::ModuleInfo* context, Luau::AstExpr* expr, const Luau::TypeCheckLimits& limits) override
//! {
//!     if (Luau::AstExprGlobal* g = expr->as<Luau::AstExprGlobal>())
//!         return Luau::ModuleInfo{g->name.value};
//!
//!     return std::nullopt;
//! }
//! ```

use core::ffi::CStr;

use ulua_analysis::{
  records::{module_info::ModuleInfo, type_check_limits::TypeCheckLimits},
  type_aliases::module_name_type::ModuleName,
};
use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_node::AstNode},
  rtti::ast_node_as,
};

use crate::records::demo_file_resolver::DemoFileResolver;

impl DemoFileResolver {
  pub fn resolve_module(
    &self,
    _context: *const ModuleInfo,
    expr: *mut AstExpr,
    _limits: &TypeCheckLimits,
  ) -> Option<ModuleInfo> {
    // expr->as<Luau::AstExprGlobal>()
    let g = unsafe { ast_node_as::<AstExprGlobal>(expr as *mut AstNode) };
    if g.is_null() {
      return None;
    }

    // ModuleInfo{g->name.value} — g->name.value is the interned C string
    // naming the global; its contents become the (string) ModuleName.
    let value = unsafe { (*g).name.value };
    let name: ModuleName = if value.is_null() {
      ModuleName::default()
    } else {
      unsafe { CStr::from_ptr(value) }
        .to_string_lossy()
        .into_owned()
    };

    Some(ModuleInfo {
      name,
      optional: false,
    })
  }
}
