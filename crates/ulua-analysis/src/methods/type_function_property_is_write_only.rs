use crate::records::type_function_property::TypeFunctionProperty;

impl TypeFunctionProperty {
  #[inline]
  pub fn is_write_only(&self) -> bool {
    self.write_ty.is_some() && self.read_ty.is_none()
  }
}
