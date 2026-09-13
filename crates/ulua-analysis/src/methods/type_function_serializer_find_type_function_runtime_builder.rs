use crate::{
  functions::follow_type::follow_type_id,
  records::type_function_serializer::TypeFunctionSerializer,
  type_aliases::{type_function_type_id::TypeFunctionTypeId, type_id::TypeId},
};

impl TypeFunctionSerializer {
  pub fn find_type_id(&self, ty: TypeId) -> Option<TypeFunctionTypeId> {
    let ty = follow_type_id(ty);
    self.types.get(&ty).copied()
  }
}
