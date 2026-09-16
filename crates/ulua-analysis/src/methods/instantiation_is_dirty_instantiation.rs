use crate::{
  records::{function_type::FunctionType, instantiation::Instantiation},
  type_aliases::type_id::TypeId,
};

impl Instantiation {
  pub fn is_dirty_type_id(&self, ty: TypeId) -> bool {
    let log = self.base.base.log;
    let ftv = unsafe { (*log).txn_log_get_mutable::<FunctionType, TypeId>(ty) };
    if !ftv.is_null() {
      if unsafe { (*ftv).has_no_free_or_generic_types } {
        return false;
      }
      return true;
    }
    false
  }
}
