#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockOrdering {
  pub depth: u32,
  pub pre_order: u32,
  pub post_order: u32,
  pub visited: bool,
}

impl Default for BlockOrdering {
  fn default() -> Self {
    Self {
      depth: 0,
      pre_order: !0u32,
      post_order: !0u32,
      visited: false,
    }
  }
}

impl BlockOrdering {
  pub const PRE_ORDER: u32 = !0u32;
  pub const POST_ORDER: u32 = !0u32;
}
