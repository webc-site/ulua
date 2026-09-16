use core::{
  mem::{MaybeUninit, size_of},
  ptr::copy_nonoverlapping,
};

/// 从 `data[*offset..]` 读取一个按内存布局解释的 `T`，并推进 `*offset`。
///
/// 越界或 `*offset + size_of::<T>()` 溢出时失败（对应 C++ `memcpy` 前的
/// `LUAU_ASSERT`）。`checked_add` 保证 release 下偏移回绕不会伪装成合法
/// 边界（否则 `ptr::add` 会解引用出界指针，是 UB）。
pub fn read<T: Copy>(data: &[u8], offset: &mut usize) -> T {
  let size = size_of::<T>();
  let end = offset
    .checked_add(size)
    .filter(|&end| end <= data.len())
    .expect("read out of bounds");

  let mut result = MaybeUninit::<T>::uninit();
  // SAFETY：`offset..end` 已验证落在 `data` 界内，源/目标不重叠。
  unsafe {
    copy_nonoverlapping(
      data.as_ptr().add(*offset),
      result.as_mut_ptr().cast::<u8>(),
      size,
    );
  }
  *offset = end;
  // SAFETY：`T: Copy`，刚写入完整的 `size_of::<T>()` 字节。
  unsafe { result.assume_init() }
}
