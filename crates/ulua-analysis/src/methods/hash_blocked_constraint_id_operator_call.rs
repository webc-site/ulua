use crate::{
  records::hash_blocked_constraint_id::HashBlockedConstraintId,
  type_aliases::blocked_constraint_id::BlockedConstraintId,
};

impl HashBlockedConstraintId {
  /// C++ `size_t HashBlockedConstraintId::operator()(const BlockedConstraintId& bci) const`
  /// (ConstraintGraph.cpp:42). `std::hash<T*>` is the identity on the pointer
  /// value, so each alternative hashes to its raw pointer cast to `usize`.
  pub fn operator_call(&self, bci: &BlockedConstraintId) -> usize {
    let mut result: usize = 0;

    if let Some(ty) = bci.get_if_0() {
      result = *ty as usize;
    } else if let Some(tp) = bci.get_if_1() {
      result = *tp as usize;
    } else if let Some(c) = bci.get_if_2() {
      result = *c as usize;
    } else {
      ulua_common::macros::luau_assert::LUAU_ASSERT!(false);
    }

    result
  }
}
