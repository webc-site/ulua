use crate::records::uninhabited_type_pack_function::UninhabitedTypePackFunction;

impl UninhabitedTypePackFunction {
  #[inline]
  pub fn operator_eq(&self, rhs: &UninhabitedTypePackFunction) -> bool {
    self.tp == rhs.tp
  }
}
