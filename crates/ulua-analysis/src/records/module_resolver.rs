extern crate alloc;

use alloc::string::String;

use ulua_ast::records::ast_expr::AstExpr;

use crate::{
  records::module_info::ModuleInfo,
  type_aliases::{module_name_type::ModuleName, module_ptr_module::ModulePtr},
};

// C++ pure-virtual interface (ModuleResolver.h:18-49), ported with the
// FileResolver struct+vtable pattern. Every entry is pure virtual in C++, so
// there are no default fns; concrete resolvers fill the vtable.
#[repr(C)]
pub struct ModuleResolver {
  pub vtable: ModuleResolverVtable,
}

#[derive(Clone, Copy)]
pub struct ModuleResolverVtable {
  /// Compute a ModuleInfo from the AST argument to require(). None = unable
  /// to determine validity (type inference silently assumes success).
  pub resolve_module_info: unsafe fn(
    *mut ModuleResolver,
    current_module_name: &ModuleName,
    path_expr: *const AstExpr,
  ) -> Option<ModuleInfo>,
  /// Null when the module is unknown at compile time, or mid-typecheck on a cycle.
  pub get_module: unsafe fn(*const ModuleResolver, module_name: &ModuleName) -> Option<ModulePtr>,
  /// Distinguishes the two get_module-null cases.
  pub module_exists: unsafe fn(*const ModuleResolver, module_name: &ModuleName) -> bool,
  pub get_human_readable_module_name:
    unsafe fn(*const ModuleResolver, module_name: &ModuleName) -> String,
}
