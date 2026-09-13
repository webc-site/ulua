use core::ptr::null_mut;

use crate::records::allocator::Allocator;

impl Allocator {
  /// C++ move constructor `Allocator::Allocator(Allocator&& rhs)`
  /// (Ast/src/Allocator.cpp:15): steals `rhs`'s pages and leaves `rhs` in the
  /// null/empty state (still usable — `allocate` lazily allocates a fresh page
  /// when `root` is null). Returns the newly-constructed owning allocator.
  pub fn allocator_allocator(rhs: &mut Allocator) -> Allocator {
    let moved = Allocator {
      root: rhs.root,
      offset: rhs.offset,
    };

    rhs.root = null_mut();
    rhs.offset = 0;

    moved
  }
}
