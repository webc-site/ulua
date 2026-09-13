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
