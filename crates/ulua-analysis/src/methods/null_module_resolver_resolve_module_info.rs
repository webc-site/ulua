use ulua_ast::records::ast_expr::AstExpr;

use crate::{
  records::{module_info::ModuleInfo, null_module_resolver::NullModuleResolver},
  type_aliases::module_name_type::ModuleName,
};

impl NullModuleResolver {
  pub fn resolve_module_info(
    &mut self,
    _current_module_name: &ModuleName,
    _path_expr: &AstExpr,
  ) -> Option<ModuleInfo> {
    None
  }
}
