use crate::{
  records::{lazy_type::LazyType, type_stringifier::TypeStringifier},
  type_aliases::type_id::TypeId,
};

impl TypeStringifier {
  pub fn operator_call_13(&mut self, _ty: TypeId, ltv: &LazyType) {
    unsafe {
      if !ltv.unwrapped.is_null() {
        let unwrapped = ltv.unwrapped;
        self.stringify_type_id(unwrapped);
      } else {
        (*(*self.state).result).invalid = true;
        (*self.state).emit("lazy?");
      }
    }
  }
}
