//! `type_pack` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::vec::Vec;

use crate::{records::type_pack::TypePack, type_aliases::type_id::TypeId};

impl TypePack {
  pub fn head(&self) -> &Vec<TypeId> {
    &self.head
  }
}

impl TypePack {
  pub fn head_mut(&mut self) -> &mut Vec<TypeId> {
    &mut self.head
  }
}
