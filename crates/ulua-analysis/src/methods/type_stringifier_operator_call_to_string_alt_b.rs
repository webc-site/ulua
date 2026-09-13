use crate::{
  records::type_stringifier::TypeStringifier,
  type_aliases::{bound_type::BoundType, type_id::TypeId},
};

impl TypeStringifier {
  pub fn operator_call_10(&mut self, _ty: TypeId, btv: &BoundType) {
    unsafe { self.stringify_type_id(btv.bound_to) };
  }
}
