use crate::{
  functions::filter_map::filter_map as filter_map_type_id,
  records::{type_checker::TypeChecker, union_type::UnionType},
  type_aliases::{type_id::TypeId, type_id_predicate::TypeIdPredicate},
};

impl TypeChecker {
  /// C++ `std::optional<TypeId> TypeChecker::filterMapImpl(TypeId, TypeIdPredicate)`
  /// (`Analysis/src/TypeInfer.cpp:5559-5565`)。predicate 以泛型传入，
  /// 上游 `TypeIdPredicate` 是 `std::function`，这里无需 dyn/Box。
  pub fn filter_map_impl<P: TypeIdPredicate>(
    &mut self,
    r#type: TypeId,
    predicate: &mut P,
  ) -> Option<TypeId> {
    let types = filter_map_type_id(r#type, self, predicate);
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
