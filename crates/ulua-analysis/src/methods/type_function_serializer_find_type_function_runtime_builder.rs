use crate::{
  functions::{follow_type, follow_type_pack},
  records::type_function_serializer::TypeFunctionSerializer,
  type_aliases::{
    type_function_kind::TypeFunctionKind, type_function_type_id::TypeFunctionTypeId,
    type_function_type_pack_id::TypeFunctionTypePackId, type_id::TypeId, type_or_pack::TypeOrPack,
    type_pack_id::TypePackId,
  },
};

impl TypeFunctionSerializer {
  pub fn find_type_id(&self, ty: TypeId) -> Option<TypeFunctionTypeId> {
    let ty = follow_type::follow(ty);
    self.types.get(&ty).copied()
  }

  pub(crate) fn find_type_pack_id(&self, tp: TypePackId) -> Option<TypeFunctionTypePackId> {
    let tp = follow_type_pack::follow(tp);
    self.packs.get(&tp).copied()
  }

  pub fn find_type_or_pack(&self, kind: TypeOrPack) -> Option<TypeFunctionKind> {
    match kind {
      TypeOrPack::V0(ty) => self.find_type_id(ty).map(TypeFunctionKind::V0),
      TypeOrPack::V1(tp) => self.find_type_pack_id(tp).map(TypeFunctionKind::V1),
    }
  }
}
