use crate::{
  records::{intersection_type::IntersectionType, union_type::UnionType},
  type_aliases::{
    intersection_type_iterator::IntersectionTypeIterator, union_type_iterator::UnionTypeIterator,
  },
};

pub fn begin_union_type(utv: &UnionType) -> UnionTypeIterator {
  // Safety: `utv` 是合法引用，天然满足被调方 `TypeIterator::type_iterator_type(t: *const T)`
  // 的全部前提——非空、正确对齐、指向存活的 `UnionType`（其内部只 `LUAU_ASSERT!(!t.is_null())`、
  // 读 `get_types()` 并把 `(t, 0)` 压入栈）。`utv as *const UnionType` 是同一对象的类型视图，
  // 不改变地址。注意与 C++ `TypeIterator(const T*)` 一致的纪律：返回的迭代器按裸指针保存该
  // 结点，调用方必须在 union 所属 arena 存活期内使用它，不得让其越过类型的生存期。
  unsafe { UnionTypeIterator::type_iterator_type(utv as *const UnionType) }
}

pub fn begin_intersection_type(itv: &IntersectionType) -> IntersectionTypeIterator {
  // Safety: 同上——引用即「非空 + 对齐 + 存活」的类型系统证明，满足
  // `TypeIterator<IntersectionType>::type_iterator_type` 对 `*const IntersectionType` 的
  // 要求；转换只是指针类型视图变化，返回的迭代器沿用 C++ 的裸指针语义，使用期不得超出
  // 该 intersection 结点在 arena 中的存活期（arena 为 bump 块，结点地址不移动）。
  unsafe { IntersectionTypeIterator::type_iterator_type(itv as *const IntersectionType) }
}
