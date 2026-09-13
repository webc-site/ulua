use crate::records::{
  type_function_cloner::TypeFunctionCloner,
  type_function_generic_type_pack::TypeFunctionGenericTypePack,
};

impl TypeFunctionCloner {
  pub fn clone_children_type_function_generic_type_pack_type_function_generic_type_pack(
    &mut self,
    _g1: *mut TypeFunctionGenericTypePack,
    _g2: *mut TypeFunctionGenericTypePack,
  ) {
    // noop.
  }
}
