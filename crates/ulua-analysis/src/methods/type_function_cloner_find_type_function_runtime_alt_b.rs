use crate::{
  records::type_function_cloner::TypeFunctionCloner,
  type_aliases::type_function_type_pack_id::TypeFunctionTypePackId,
};

impl TypeFunctionCloner {
  pub fn find_type_function_type_pack_id(
    &self,
    tp: TypeFunctionTypePackId,
  ) -> Option<TypeFunctionTypePackId> {
    self.packs.find(&tp).copied()
  }
}
