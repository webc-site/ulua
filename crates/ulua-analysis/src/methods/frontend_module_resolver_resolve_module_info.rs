use core::ptr::from_ref;

use ulua_ast::{records::ast_expr::AstExpr, rtti::AstNodePtr};

use crate::{
  records::{frontend_module_resolver::FrontendModuleResolver, module_info::ModuleInfo},
  type_aliases::module_name_type::ModuleName,
};

impl FrontendModuleResolver {
  pub fn resolve_module_info(
    &self,
    current_module_name: &ModuleName,
    path_expr: &AstExpr,
  ) -> Option<ModuleInfo> {
    // C++ `if (!frontend) return nullopt;` 的 None 分支；解引用集中于
    // `frontend_ref` chokepoint。
    let frontend = self.frontend_ref()?;
    let trace = frontend.require_trace.get(current_module_name)?;
    let key = from_ref(path_expr).cast_mut().as_ast_node();

    trace.exprs.find(&key).cloned()
  }
}
