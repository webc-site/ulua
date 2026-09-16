pub use ulua_ast::rtti::AstNodeClass;

use crate::records::nth::Nth;

/// Returns the nth element (1-indexed) from the iterator, or `None` if there are fewer than `n` elements.
pub fn nth<T, I: Iterator<Item = T>>(iter: &mut I, n: i32) -> Option<T> {
  if n > 0 {
    for (i, item) in iter.by_ref().enumerate() {
      if (i as i32) + 1 == n {
        return Some(item);
      }
    }
  }
  None
}

/// Template function equivalent: returns an `Nth` query object configured to select the nth element (1-indexed)
/// of the given type `T` (a subtype of `AstNode`).
pub fn nth_t<T: AstNodeClass>(nth: i32) -> Nth {
  Nth {
    class_index: T::CLASS_INDEX,
    nth,
  }
}
