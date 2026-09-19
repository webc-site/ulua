use alloc::vec::Vec;
use core::ptr::write;

use crate::{
  records::type_pack_function::TypePackFunction,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
#[derive(Debug, Clone)]
pub struct TypeFunctionInstanceTypePack {
  pub(crate) function: *const TypePackFunction,
  pub(crate) type_arguments: Vec<TypeId>,
  pub(crate) pack_arguments: Vec<TypePackId>,
}

impl Drop for TypeFunctionInstanceTypePack {
  fn drop(&mut self) {
    unsafe {
      write(&mut self.type_arguments, Vec::new());
      write(&mut self.pack_arguments, Vec::new());
    }
  }
}
