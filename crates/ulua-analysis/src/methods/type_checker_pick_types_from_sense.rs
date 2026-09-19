use crate::{records::type_checker::TypeChecker, type_aliases::type_id::TypeId};

impl TypeChecker {
  pub fn pick_types_from_sense(
    &mut self,
    _type: TypeId,
    _sense: bool,
    _empty_set_ty: TypeId,
  ) -> (Option<TypeId>, bool) {
    let predicate = self.mk_truthy_predicate(_sense, _empty_set_ty);
    self.filter_map(_type, predicate)
  }
}
