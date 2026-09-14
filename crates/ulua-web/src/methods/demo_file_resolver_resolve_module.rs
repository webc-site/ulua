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

use ulua_analysis::{
  records::{module_info::ModuleInfo, type_check_limits::TypeCheckLimits},
  type_aliases::module_name_type::ModuleName,
};
use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_node::AstNode},
  rtti::ast_node_as,
};

use crate::{records::demo_file_resolver::DemoFileResolver, util::cstr_cow};

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

    // ModuleInfo{g->name.value}：name.value 是指名全局的驻留 C 字符串，
    // 其内容成为（字符串型）ModuleName。
    let value = unsafe { (*g).name.value };
    // SAFETY: value 指向 VM 驻留字符串，驻留串生命周期覆盖 VM 存活期；
    // null 时 cstr_cow 返回空串，等价 ModuleName::default()。
    let name: ModuleName = unsafe { cstr_cow(value) }.into_owned();

    Some(ModuleInfo {
      name,
      optional: false,
    })
  }
}
