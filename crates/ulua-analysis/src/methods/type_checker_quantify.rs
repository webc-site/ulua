use ulua_ast::records::location::Location;

use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id, quantify::quantify},
  records::{function_type::FunctionType, type_checker::TypeChecker},
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};

impl TypeChecker {
  pub fn quantify(&mut self, scope: &ScopePtr, ty: TypeId, _location: Location) -> TypeId {
    let ty = follow_type_id(ty);

    let ftv = get_type_id::<FunctionType>(ty);

    if !ftv.is_none() {
      quantify(ty, scope.level);
    }

    ty
  }
}
