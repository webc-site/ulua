use crate::records::{checkpoint::Checkpoint, constraint_generator::ConstraintGenerator};

/// # Safety
/// 调用方须保证 `cg` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn checkpoint(cg: *const ConstraintGenerator) -> Checkpoint {
  // SAFETY: The caller guarantees `cg` is a valid pointer to a `ConstraintGenerator`.
  // The `constraints` field is a `Vec<ConstraintPtr>`, and `size()` returns its length.
  let offset = unsafe { (*cg).constraints.len() };
  Checkpoint { offset }
}
