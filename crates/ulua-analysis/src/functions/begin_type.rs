use crate::{
  records::{intersection_type::IntersectionType, union_type::UnionType},
  type_aliases::{
    intersection_type_iterator::IntersectionTypeIterator, union_type_iterator::UnionTypeIterator,
  },
};

pub fn begin_union_type(utv: &UnionType) -> UnionTypeIterator {
  UnionTypeIterator::type_iterator_type(utv)
}

pub fn begin_intersection_type(itv: &IntersectionType) -> IntersectionTypeIterator {
  IntersectionTypeIterator::type_iterator_type(itv)
}
