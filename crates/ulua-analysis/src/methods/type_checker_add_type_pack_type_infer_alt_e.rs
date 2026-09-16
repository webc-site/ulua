use crate::{
  records::{type_checker::TypeChecker, type_pack::TypePack, type_pack_var::TypePackVar},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl TypeChecker {
  /// C++ `TypePackId TypeChecker::addTypePack(std::initializer_list<TypeId>&& ty)` (TypeInfer.cpp:5610):
  /// `return addTypePack(TypePackVar(TypePack{std::vector<TypeId>(begin(ty), end(ty)), std::nullopt}));`
  pub fn add_type_pack_initializer_list_type_id(&mut self, ty: &[TypeId]) -> TypePackId {
    self.add_type_pack_type_pack_var(TypePackVar::from(TypePack {
      head: ty.to_vec(),
      tail: None,
    }))
  }
}
