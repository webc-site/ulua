use crate::records::block_ordering::BlockOrdering;

pub fn find_common_dominator(idoms: &[u32], data: &[BlockOrdering], mut a: u32, mut b: u32) -> u32 {
  while a != b {
    while data[a as usize].post_order < data[b as usize].post_order {
      a = idoms[a as usize];
      if a == !0u32 {
        // Keep behavior native-only assertion without relying on CODEGEN_ASSERT.
        panic!("CODEGEN_ASSERT failed: a != !0u32");
      }
    }

    while data[b as usize].post_order < data[a as usize].post_order {
      b = idoms[b as usize];
      if b == !0u32 {
        // Keep behavior native-only assertion without relying on CODEGEN_ASSERT.
        panic!("CODEGEN_ASSERT failed: b != !0u32");
      }
    }
  }

  a
}
