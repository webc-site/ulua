use crate::records::type_function_property::TypeFunctionProperty;

impl TypeFunctionProperty {
  #[inline]
  pub fn is_shared(&self) -> bool {
    if let (Some(read_ty), Some(write_ty)) = (self.read_ty, self.write_ty) {
      read_ty == write_ty
    } else {
      false
    }
  }
}
