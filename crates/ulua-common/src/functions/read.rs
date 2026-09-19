use core::mem::size_of;

pub trait BytecodeRead: Copy {
  fn from_bytes(bytes: &[u8]) -> Self;
}

macro_rules! impl_bytecode_read {
  ($($ty:ty),* $(,)?) => {
    $(impl BytecodeRead for $ty {
      fn from_bytes(bytes: &[u8]) -> Self {
        Self::from_ne_bytes(bytes.try_into().expect("invalid byte width"))
      }
    })*
  };
}

impl_bytecode_read!(
  u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64
);

/// 从 `data[*offset..]` 读取一个按内存布局解释的 `T`，并推进 `*offset`。
///
/// 越界或 `*offset + size_of::<T>()` 溢出时失败（对应 C++ `memcpy` 前的
/// `LUAU_ASSERT`）。`checked_add` 保证 release 下偏移回绕不会伪装成合法
/// 边界（否则 `ptr::add` 会解引用出界指针，是 UB）。
pub fn read<T: BytecodeRead>(data: &[u8], offset: &mut usize) -> T {
  let size = size_of::<T>();
  let end = offset
    .checked_add(size)
    .filter(|&end| end <= data.len())
    .expect("read out of bounds");

  let result = T::from_bytes(&data[*offset..end]);
  *offset = end;
  result
}
