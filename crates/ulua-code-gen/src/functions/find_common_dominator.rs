use crate::records::block_ordering::BlockOrdering;

pub fn find_common_dominator(idoms: &[u32], data: &[BlockOrdering], mut a: u32, mut b: u32) -> u32 {
  while a != b {
    while data[a as usize].post_order < data[b as usize].post_order {
      a = idoms[a as usize];
      if a == !0u32 {
        // 保留 native-only 行为断言，不依赖 CODEGEN_ASSERT（cpp findCommonDominator 的
        // LUAU_ASSERT(a != ~0u)；支配链上溯到哨兵根即 idom 表被破坏，继续循环只会越界）。
        panic!("CODEGEN_ASSERT 保留：支配树上溯未到根（a 不得为哨兵 !0u32）");
      }
    }

    while data[b as usize].post_order < data[a as usize].post_order {
      b = idoms[b as usize];
      if b == !0u32 {
        // 保留 native-only 行为断言，不依赖 CODEGEN_ASSERT（与上支对称：支配树上溯未到根即
        // idom 表被破坏，cpp 侧同一 LUAU_ASSERT）。
        panic!("CODEGEN_ASSERT 保留：支配树上溯未到根（b 不得为哨兵 !0u32）");
      }
    }
  }

  a
}
