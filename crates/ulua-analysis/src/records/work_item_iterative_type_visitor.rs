//! @interface-stub
use core::{ffi::c_void, ptr::null};

use crate::type_aliases::{type_id::TypeId, type_pack_id::TypePackId};
#[derive(Debug, Clone)]
pub struct WorkItem {
  pub(crate) t: *const c_void,
  pub(crate) is_type: bool,
  pub(crate) parent: i32,
}

impl WorkItem {
  pub fn work_item_type_id_i32(ty: TypeId, parent: i32) -> Self {
    Self {
      t: ty as *const c_void,
      is_type: true,
      parent,
    }
  }
  pub fn work_item_type_pack_id_i32(tp: TypePackId, parent: i32) -> Self {
    Self {
      t: tp as *const c_void,
      is_type: false,
      parent,
    }
  }
  pub fn as_type(&self) -> *const TypeId {
    if self.is_type {
      &self.t as *const *const c_void as *const TypeId
    } else {
      null()
    }
  }
  pub fn as_type_pack(&self) -> *const TypePackId {
    if self.is_type {
      null()
    } else {
      &self.t as *const *const c_void as *const TypePackId
    }
  }
  pub fn operator_eq_type_id(&self, ty: TypeId) -> bool {
    self.type_id() == Some(ty)
  }
  pub fn operator_eq_type_pack_id(&self, tp: TypePackId) -> bool {
    self.type_pack_id() == Some(tp)
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
