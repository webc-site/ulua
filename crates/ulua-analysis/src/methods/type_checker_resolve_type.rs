use ulua_ast::records::ast_type::AstType;

use crate::{
  functions::arc_as_mut::arc_as_mut,
  records::type_checker::TypeChecker,
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};
impl TypeChecker {
  pub fn resolve_type(&mut self, scope: ScopePtr, annotation: &AstType) -> TypeId {
    let ty = self.resolve_type_worker(scope, annotation);

    unsafe {
      let module = arc_as_mut(self.current_module.as_ref().expect("current_module 由 check_without_recursion_check 入口置入 Some、末尾才 take()，check 调用树内恒为 Some"));
      *(*module)
        .ast_resolved_types
        .get_or_insert(annotation as *const AstType) = ty;
    }

    ty
  }
}
