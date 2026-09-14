use crate::{records::union_type::UnionType, type_aliases::union_type_iterator::UnionTypeIterator};

pub fn end_union_type(_utv: &UnionType) -> UnionTypeIterator {
  UnionTypeIterator::type_iterator_default()
}
