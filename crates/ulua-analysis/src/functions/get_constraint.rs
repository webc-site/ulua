//! Source: `Analysis/include/Luau/Constraint.h:374-382` (hand-ported)
/// C++ `template<typename T> const T* get(const Constraint& c)` 的 Rust 惯用形式：
/// 命中变体返回引用，否则 `None`——判空与下转合并为一次安全操作。
use crate::{records::constraint::Constraint, type_aliases::constraint_v::ConstraintVMember};

pub fn get_constraint<T: ConstraintVMember>(c: &Constraint) -> Option<&T> {
  T::get_if(&c.c)
}
