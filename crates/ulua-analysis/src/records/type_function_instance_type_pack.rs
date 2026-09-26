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
    // Safety: drop(&mut self) 提供对两字段的独占可写访问且它们均为完好初始化的
    // Vec；ptr::write 直接覆盖存储、不读也不 drop 旧值——元素是 Copy 的 arena
    // 句柄（*const Type/*const TypePackVar），无自有堆资源，不产生第二所有者或
    // 双 drop，旧 Vec 缓冲被刻意弃置；后续 drop 只会见到新写入的空 Vec。
    unsafe {
      write(&mut self.type_arguments, Vec::new());
      write(&mut self.pack_arguments, Vec::new());
    }
  }
}
