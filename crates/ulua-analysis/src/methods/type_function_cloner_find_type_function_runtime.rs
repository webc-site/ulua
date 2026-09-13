use crate::{
  records::type_function_cloner::TypeFunctionCloner,
  type_aliases::type_function_type_id::TypeFunctionTypeId,
};

impl TypeFunctionCloner {
  pub fn find_type_function_type_id(&self, ty: TypeFunctionTypeId) -> Option<TypeFunctionTypeId> {
    self.types.find(&ty).copied()
  }
}
