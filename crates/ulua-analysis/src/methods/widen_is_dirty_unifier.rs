use crate::{
  records::{singleton_type::SingletonType, widen::Widen},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Widen {
  pub fn is_dirty_type_id(&mut self, ty: TypeId) -> bool {
    unsafe { (*self.base.base.log).txn_log_is::<SingletonType, _>(ty) }
  }

  pub fn is_dirty_type_pack_id(&mut self, _tp: TypePackId) -> bool {
    false
  }
}
