use core::array;

use ulua_common::functions::read::{BytecodeRead, read};

#[test]
fn reads_little_types_and_advances_offset() {
  let data = [1u8, 2, 3, 4, 5, 6, 7, 8];
  let mut offset = 0;

  let byte = read::<u8>(&data, &mut offset);
  assert_eq!(byte, 1);
  assert_eq!(offset, 1);

  let half = read::<u16>(&data, &mut offset);
  assert_eq!(half, u16::from_ne_bytes([2, 3]));
  assert_eq!(offset, 3);

  let word = read::<u32>(&data, &mut offset);
  assert_eq!(word, u32::from_ne_bytes([4, 5, 6, 7]));
  assert_eq!(offset, 7);

  let byte2 = read::<u8>(&data, &mut offset);
  assert_eq!(byte2, 8);
  assert_eq!(offset, 8);
}

#[test]
fn read_f64_roundtrips_through_ne_bytes() {
  let value = f64::from_bits(0x4009_21FB_5444_2D18);
  let mut bytes = [0u8; 8];
  bytes.copy_from_slice(&value.to_ne_bytes());
  let mut offset = 0;
  assert_eq!(read::<f64>(&bytes, &mut offset), value);
  assert_eq!(offset, 8);
}

#[test]
#[should_panic(expected = "read out of bounds")]
fn read_past_end_panics_like_cpp_assert() {
  let data = [1u8, 2, 3];
  let mut offset = 0;
  read::<u32>(&data, &mut offset);
}

#[test]
#[should_panic(expected = "read out of bounds")]
fn read_offset_overflow_panics_instead_of_wrapping() {
  let data = [1u8, 2, 3, 4];
  let mut offset = usize::MAX;
  read::<u32>(&data, &mut offset);
}

#[test]
fn read_exact_size_ok() {
  let data: [u8; 8] = array::from_fn(|i| i as u8);
  let mut offset = 4;
  assert_eq!(
    read::<u32>(&data, &mut offset),
    u32::from_ne_bytes([4, 5, 6, 7])
  );
  assert_eq!(offset, 8);
}

#[test]
fn trait_covers_std_scalars_used_by_bytecode_loaders() {
  fn assert_impls<T: BytecodeRead>() {}
  assert_impls::<u8>();
  assert_impls::<i8>();
  assert_impls::<u16>();
  assert_impls::<i16>();
  assert_impls::<u32>();
  assert_impls::<i32>();
  assert_impls::<u64>();
  assert_impls::<i64>();
  assert_impls::<u128>();
  assert_impls::<i128>();
  assert_impls::<usize>();
  assert_impls::<isize>();
  assert_impls::<f32>();
  assert_impls::<f64>();
}
