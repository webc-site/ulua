use crate::{
  records::type_checker::TypeChecker,
  type_aliases::{type_id::TypeId, type_id_predicate::TypeIdPredicate},
};

impl TypeChecker {
  pub fn filter_map(
    &mut self,
    r#type: TypeId,
    predicate: TypeIdPredicate,
  ) -> (Option<TypeId>, bool) {
    let ty_opt = self.filter_map_impl(r#type, predicate);
    let ty = ty_opt.unwrap_or(self.never_type);
    let ty_is_never = ty == self.never_type;
    (Some(ty), !ty_is_never)
  }
}
