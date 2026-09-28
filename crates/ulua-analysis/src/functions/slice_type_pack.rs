use alloc::vec::Vec;

use crate::{
  functions::{begin_type_pack::begin, end_type_pack::end_type_pack_id},
  records::{builtin_types::BuiltinTypes, type_arena::TypeArena},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
pub fn slice_type_pack(
  slice_index: usize,
  to_be_sliced: TypePackId,
  head: &[TypeId],
  tail: Option<TypePackId>,
  builtin_types: &BuiltinTypes,
  arena: &mut TypeArena,
) -> TypePackId {
  if slice_index == 0 {
    to_be_sliced
  } else if slice_index == head.len() {
    tail.unwrap_or(builtin_types.empty_type_pack)
  } else {
    let mut head_slice = Vec::new();
    let mut iter = begin(to_be_sliced);
    let end_iter = end_type_pack_id(to_be_sliced);

    // 前置游标按 slice_index 逐个 advance（自定义 TypePackIterator 游标推进，非容器下标）；
    // 迭代器 next() 语义与手动 current()/advance() 终点判定不完全一致，保留原推进写法。
    for _ in 0..slice_index {
      if iter != end_iter {
        iter.advance();
      }
    }

    while iter != end_iter {
      head_slice.push(*iter.current());
      iter.advance();
    }

    arena.add_type_pack_vector_type_id_optional_type_pack_id(head_slice, tail)
  }
}
