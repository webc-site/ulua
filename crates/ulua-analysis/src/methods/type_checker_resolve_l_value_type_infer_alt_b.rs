use crate::{
  records::type_checker::TypeChecker,
  type_aliases::{
    l_value::LValue, refinement_map::RefinementMap, scope_ptr_type::ScopePtr, type_id::TypeId,
  },
};

impl TypeChecker {
  pub fn resolve_l_value_refinement_map_scope_ptr_l_value(
    &mut self,
    refis: &RefinementMap,
    scope: ScopePtr,
    lvalue: &LValue,
  ) -> Option<TypeId> {
    if let Some(ty) = refis.get(lvalue) {
      Some(*ty)
    } else {
      self.resolve_l_value_scope_ptr_l_value(scope, lvalue)
    }
  }
}
