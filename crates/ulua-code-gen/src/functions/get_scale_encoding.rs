// See also: 本 crate `macros/codegen_assert.rs` 的 `CODEGEN_ASSERT!`——形似义异：
// 该宏走 ulua_common `assert_fail` C-ABI 上报通道，此处是同名局部替身（标准
// `assert!` panic 语义），刻意不统一，勿合并。
macro_rules! CODEGEN_ASSERT {
  ($expr:expr) => {
    assert!($expr);
  };
}

pub fn get_scale_encoding(scale: u8) -> u8 {
  const SCALES: [u8; 9] = [0xff, 0, 1, 0xff, 2, 0xff, 0xff, 0xff, 3];

  CODEGEN_ASSERT!(scale < 9 && SCALES[scale as usize] != 0xff);
  SCALES[scale as usize]
}
