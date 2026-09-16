use crate::records::type_function_property::TypeFunctionProperty;

impl TypeFunctionProperty {
  #[inline]
  pub fn is_read_only(&self) -> bool {
    self.read_ty.is_some() && !self.write_ty.is_some()
  }
}
