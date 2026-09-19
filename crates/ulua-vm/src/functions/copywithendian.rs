use core::{
  ffi::c_char,
  ptr::copy_nonoverlapping,
  slice::{from_raw_parts, from_raw_parts_mut},
};

/// 端序感知拷贝：与 cpp/VM/src/lstrlib.cpp:1392 一致。
/// 同本机字节序 → 等价 memcpy（ptr::copy_nonoverlapping，调用处 dest/src 不重叠）；
/// 异字节序 → 逆序逐字节拷贝（`dst[j] = src[len-1-j]`）。
///
/// # Safety
/// `dest`/`src` 必须有效且至少可读写 `size` 字节，两区间不得重叠。
pub(crate) unsafe fn copywithendian(
  dest: *mut c_char,
  src: *const c_char,
  size: i32,
  islittle: i32,
) {
  let n = size as usize;
  // 本机端序由目标平台编译期决定（cpp 原版为 nativeendian.little 运行时常量，值相同）
  let native_is_little = cfg!(target_endian = "little");

  if (islittle != 0) == native_is_little {
    unsafe {
      copy_nonoverlapping(src, dest, n);
    }
  } else {
    let src = unsafe { from_raw_parts(src as *const u8, n) };
    let dst = unsafe { from_raw_parts_mut(dest as *mut u8, n) };
    for (d, &s) in dst.iter_mut().zip(src.iter().rev()) {
      *d = s;
    }
  }
}
