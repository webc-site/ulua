use core::ptr::copy_nonoverlapping;

/// 端序感知拷贝：与 cpp/VM/src/lstrlib.cpp:1471 一致。
/// 同本机端序 → 等价 memcpy（`ptr::copy_nonoverlapping`）；异端序 → 逆序逐字节
/// 拷贝（`dst[j] = src[len-1-j]`）。
///
/// 切片化签名（review §2/§3：裸指针入参 → 切片）：拷贝长度即 `dest.len()`，
/// `src` 须不短于 `dest`；两切片不得重叠（现有调用方传各自的独立栈缓冲）。
pub(crate) fn copywithendian(dest: &mut [u8], src: &[u8], islittle: i32) {
  // 越界即 panic 而非 UB：合法调用方（str_pack 固定缓冲 / str_unpack 窗口钳位）
  // 恒满足 `src.len() >= dest.len()`，此片检不落入其热路径失败分支
  let src = &src[..dest.len()];
  let n = dest.len();
  // 本机端序由目标平台编译期决定（cpp 原版为 nativeendian.little 运行时常量，值相同）
  let native_is_little = cfg!(target_endian = "little");

  if (islittle != 0) == native_is_little {
    // SAFETY: `src`/`dest` 均为定长切片且 `src` 恰为 `n` 字节可读、`dest` 可写，
    // 两区间按契约不重叠，正是 copy_nonoverlapping 入约
    unsafe {
      copy_nonoverlapping(src.as_ptr(), dest.as_mut_ptr(), n);
    }
  } else {
    for (d, &s) in dest.iter_mut().zip(src.iter().rev()) {
      *d = s;
    }
  }
}
