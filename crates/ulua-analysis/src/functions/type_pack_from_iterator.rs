//! C++ `typePackFromIterator`（TypePack.cpp:367-379）。
use alloc::vec::Vec;

use crate::{
  records::{type_arena::TypeArena, type_pack_iterator::TypePackIterator},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

/// 从 `[start, end)` 构造新类型包：头逐个收集，尾部取迭代终止处的 tail。
pub fn type_pack_from_iterator(
  arena: &mut TypeArena,
  start: &mut TypePackIterator,
  end: &TypePackIterator,
) -> TypePackId {
  // 起点正好是某个包的开头：直接复用，不拷贝。
  if let Some(head_pack) = start.try_get_head() {
    return head_pack;
  }

  let mut head: Vec<TypeId> = Vec::new();
  while start != end {
    head.push(*start.current());
    start.advance();
  }

  let tail = start.tail();
  arena.add_type_pack_vector_type_id_optional_type_pack_id(head, tail)
}
