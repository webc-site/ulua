//! @interface-stub
use core::{ffi::c_void, ptr::null};

use crate::type_aliases::{
  type_function_type_id::TypeFunctionTypeId, type_function_type_pack_id::TypeFunctionTypePackId,
};
#[derive(Debug, Clone)]
pub struct WorkItem {
  pub(crate) t: *const c_void,
  pub(crate) is_type: bool,
  pub(crate) parent: i32,
}

impl WorkItem {
  pub fn work_item_type_function_type_id_i32(ty: TypeFunctionTypeId, parent: i32) -> Self {
    Self {
      t: ty as *const c_void,
      is_type: true,
      parent,
    }
  }
  pub fn work_item_type_function_type_pack_id_i32(tp: TypeFunctionTypePackId, parent: i32) -> Self {
    Self {
      t: tp as *const c_void,
      is_type: false,
      parent,
    }
  }
  pub fn as_type(&self) -> *const TypeFunctionTypeId {
    if self.is_type {
      &self.t as *const *const c_void as *const TypeFunctionTypeId
    } else {
      null()
    }
  }
  pub fn as_type_pack(&self) -> *const TypeFunctionTypePackId {
    if self.is_type {
      null()
    } else {
      &self.t as *const *const c_void as *const TypeFunctionTypePackId
    }
  }
  pub fn operator_eq_type_function_type_id(&self, ty: TypeFunctionTypeId) -> bool {
    self.type_function_type_id() == Some(ty)
  }
  pub fn operator_eq_type_function_type_pack_id(&self, tp: TypeFunctionTypePackId) -> bool {
    self.type_function_type_pack_id() == Some(tp)
  }

  pub fn type_function_type_id(&self) -> Option<TypeFunctionTypeId> {
    if self.is_type {
      Some(self.t as TypeFunctionTypeId)
    } else {
      None
    }
  }

  pub fn type_function_type_pack_id(&self) -> Option<TypeFunctionTypePackId> {
    if self.is_type {
      None
    } else {
      Some(self.t as TypeFunctionTypePackId)
    }
  }
}
