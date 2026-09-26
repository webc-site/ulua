use crate::type_aliases::{type_id::TypeId, type_pack_id::TypePackId};
#[derive(Debug, Clone)]
pub struct WorkItem {
  pub(crate) t: *const (),
  pub(crate) is_type: bool,
  pub(crate) parent: i32,
}

impl WorkItem {
  pub fn work_item_type_id_i32(ty: TypeId, parent: i32) -> Self {
    Self {
      t: ty as *const (),
      is_type: true,
      parent,
    }
  }
  pub fn work_item_type_pack_id_i32(tp: TypePackId, parent: i32) -> Self {
    Self {
      t: tp as *const (),
      is_type: false,
      parent,
    }
  }

  pub fn type_id(&self) -> Option<TypeId> {
    if self.is_type {
      Some(self.t as TypeId)
    } else {
      None
    }
  }

  pub fn type_pack_id(&self) -> Option<TypePackId> {
    if self.is_type {
      None
    } else {
      Some(self.t as TypePackId)
    }
  }
}
