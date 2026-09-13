use crate::{
  records::{type_checker::TypeChecker, type_pack::TypePack, type_pack_var::TypePackVar},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
impl TypeChecker {
  /// C++ `TypePackId TypeChecker::addTypePack(const std::vector<TypeId>& ty, std::optional<TypePackId> tail)`
  /// (TypeInfer.cpp:5605): `return addTypePack(TypePackVar(TypePack{ty, tail}));`
  pub fn add_type_pack_vector_type_id_optional_type_pack_id(
    &mut self,
    ty: &[TypeId],
    tail: Option<TypePackId>,
  ) -> TypePackId {
    self.add_type_pack_type_pack_var(TypePackVar::from(TypePack {
      head: ty.to_vec(),
      tail,
    }))
  }
}
