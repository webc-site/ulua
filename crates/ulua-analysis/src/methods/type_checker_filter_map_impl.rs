use crate::{
  functions::filter_map::filter_map as filter_map_type_id,
  records::{type_checker::TypeChecker, union_type::UnionType},
  type_aliases::{type_id::TypeId, type_id_predicate::TypeIdPredicate},
};

impl TypeChecker {
  pub fn filter_map_impl(&mut self, r#type: TypeId, predicate: TypeIdPredicate) -> Option<TypeId> {
    let types = filter_map_type_id(r#type, predicate);
    if types.is_empty() {
      return None;
    }

    Some(if types.len() == 1 {
      types[0]
    } else {
      self.add_type(&UnionType { options: types })
    })
  }
}
