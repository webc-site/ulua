//! 回归对拍：vm `lua_s_hash` 收口至 `lua51_hash_tail` 单点后，
//! 与内联参考尾循环逐位一致（含 <32 短串与 ≥32 长串走 12B 分块后的尾段）。

use ulua_common::functions::lua51_hash_tail::lua51_hash_tail;
use ulua_vm::functions::lua_s_hash::lua_s_hash;

/// 独立复刻 cpp lstring.cpp 原式（分块 + 尾段），不经任何共享单点。
fn reference_lua_s_hash(str_: &[u8]) -> u32 {
  let mut str = str_;
  let mut a: u32 = 0;
  let mut b: u32 = 0;
  let mut h: u32 = str.len() as u32;
  let mut len = str.len();
  while len >= 32 {
    let block = [
      u32::from_ne_bytes(str[0..4].try_into().unwrap()),
      u32::from_ne_bytes(str[4..8].try_into().unwrap()),
      u32::from_ne_bytes(str[8..12].try_into().unwrap()),
    ];
    str = &str[12..];
    a = a.wrapping_add(block[0]);
    b = b.wrapping_add(block[1]);
    h = h.wrapping_add(block[2]);
    // mix(14, 11, 25) 内联，避免依赖被测侧宏；cpp rol 在本仓为 rotate_right
    #[inline(always)]
    fn rol(x: u32, n: u32) -> u32 {
      x.rotate_right(n)
    }
    a ^= h;
    a = a.wrapping_sub(rol(h, 14));
    b ^= a;
    b = b.wrapping_sub(rol(a, 11));
    h ^= b;
    h = h.wrapping_sub(rol(b, 25));
    len -= 12;
  }
  // 原 Lua 5.1 尾循环，逐字节倒序（独立副本，不调用共享单点）
  for i in (0..len).rev() {
    h ^= h
      .wrapping_shl(5)
      .wrapping_add(h.wrapping_shr(2))
      .wrapping_add(str[i] as u32);
  }
  h
}

#[test]
fn tail_single_point_matches_reference() {
  // lua51_hash_tail 单点自身对空段/全段的恒等性
  assert_eq!(lua51_hash_tail(b"", 0xdeadbeef), 0xdeadbeef);
  assert_eq!(lua51_hash_tail(b"lua", 3), reference_tail(b"lua", 3));
}

fn reference_tail(bytes: &[u8], mut h: u32) -> u32 {
  for i in (0..bytes.len()).rev() {
    h ^= h
      .wrapping_shl(5)
      .wrapping_add(h.wrapping_shr(2))
      .wrapping_add(bytes[i] as u32);
  }
  h
}

#[test]
fn vm_hash_bitwise_identical_to_reference_across_lengths() {
  let mut buf: Vec<u8> = Vec::new();
  for n in 0..=200 {
    if n > 0 {
      buf.push((n as u8).wrapping_mul(37) ^ 0x5a);
    }
    assert_eq!(lua_s_hash(&buf), reference_lua_s_hash(&buf), "len={n}");
    // 全零窗口（对齐 conformance 的错位窗口断言形态）
    let zeros = [0u8; 256];
    assert_eq!(
      lua_s_hash(&zeros[..n]),
      reference_lua_s_hash(&zeros[..n]),
      "zeros len={n}"
    );
  }
  // 随机字节流多切片长度扫描
  let mut x: u64 = 0x12345678;
  let mut data = vec![0u8; 1024];
  for d in data.iter_mut() {
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *d = (x >> 24) as u8;
  }
  for len in [1usize, 7, 15, 31, 32, 33, 44, 96, 127, 128, 129, 512, 1024] {
    assert_eq!(
      lua_s_hash(&data[..len]),
      reference_lua_s_hash(&data[..len]),
      "pseudo-random len={len}"
    );
  }
}
