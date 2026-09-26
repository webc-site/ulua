use crate::{
  functions::arc_as_mut::arc_as_mut,
  records::{type_checker::TypeChecker, type_pack::TypePack, type_pack_var::TypePackVar},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl TypeChecker {
  /// C++ `TypePackId TypeChecker::addTypePack(TypePackVar&& tv)` (TypeInfer.cpp:5590):
  /// `return currentModule->internalTypes.addTypePack(std::move(tv));`
  pub fn add_type_pack_type_pack_var(&mut self, tp: TypePackVar) -> TypePackId {
    unsafe {
      (*(arc_as_mut(self.expect_current_module())))
        .internal_types
        .add_type_pack_type_pack_var(tp)
    }
  }

  /// C++ `TypePackId TypeChecker::addTypePack(TypePack&& tp)` (TypeInfer.cpp:5595):
  /// `return addTypePack(TypePackVar(std::move(tp)));`
  pub fn add_type_pack_type_pack(&mut self, tp: TypePack) -> TypePackId {
    self.add_type_pack_type_pack_var(TypePackVar::from(tp))
  }

  /// C++ `TypePackId TypeChecker::addTypePack(const std::vector<TypeId>& ty, std::optional<TypePackId> tail)`
  /// (TypeInfer.cpp:5605): `return addTypePack(TypePackVar(TypePack{ty, tail}));`
  pub fn add_type_pack_vector_type_id_optional_type_pack_id(
    &mut self,
    ty: &[TypeId],
    tail: Option<TypePackId>,
  ) -> TypePackId {
    self.add_type_pack_type_pack_var(TypePackVar::from(TypePack::new(ty.to_vec(), tail)))
  }

  /// C++ `TypePackId TypeChecker::addTypePack(std::initializer_list<TypeId>&& ty)` (TypeInfer.cpp:5610):
  /// `return addTypePack(TypePackVar(TypePack{std::vector<TypeId>(begin(ty), end(ty)), std::nullopt}));`
  pub fn add_type_pack_initializer_list_type_id(&mut self, ty: &[TypeId]) -> TypePackId {
    self.add_type_pack_type_pack_var(TypePackVar::from(TypePack::from_vec(ty.to_vec())))
  }
}
