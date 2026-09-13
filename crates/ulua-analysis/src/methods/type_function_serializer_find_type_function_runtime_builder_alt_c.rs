use crate::{
  records::type_function_serializer::TypeFunctionSerializer,
  type_aliases::{type_function_kind::TypeFunctionKind, type_or_pack::TypeOrPack},
};

impl TypeFunctionSerializer {
  pub fn find_type_or_pack(&self, kind: TypeOrPack) -> Option<TypeFunctionKind> {
    match kind {
      TypeOrPack::V0(ty) => self.find_type_id(ty).map(TypeFunctionKind::V0),
      TypeOrPack::V1(tp) => self.find_type_pack_id(tp).map(TypeFunctionKind::V1),
    }
  }
}
