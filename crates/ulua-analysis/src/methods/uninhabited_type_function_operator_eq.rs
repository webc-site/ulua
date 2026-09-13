use crate::records::uninhabited_type_function::UninhabitedTypeFunction;

impl UninhabitedTypeFunction {
  #[inline]
  pub fn operator_eq(&self, rhs: &UninhabitedTypeFunction) -> bool {
    self.ty == rhs.ty
  }
}
