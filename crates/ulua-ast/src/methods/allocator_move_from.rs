use crate::records::allocator::Allocator;

impl Allocator {
  /// cpp `Allocator.cpp` 的 move 构造：偷走 `rhs` 的整条页链，并把 `rhs` 留在
  /// 「空链但仍可用」态（其后再分配时懒建首页）。
  pub fn move_from(rhs: &mut Allocator) -> Allocator {
    // `take` 一步完成「交出链顶 + 源侧置空」，源侧即 cpp 的 root = nullptr 空态。
    let moved = Allocator {
      root: rhs.root.take(),
      offset: rhs.offset,
    };

    rhs.offset = 0;

    moved
  }
}
