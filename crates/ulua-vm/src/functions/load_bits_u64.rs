//! buffer_readbits/buffer_writebits 共享的"字节区间装入 u64"辅助函数

use core::{ffi::c_char, ptr::copy_nonoverlapping};

use ulua_common::macros::luau_big_endian::LUAU_BIG_ENDIAN;

/// 将 `buf[startbyte..endbyte]` 装入 u64：
/// 大端 → 从高字节向低字节逐个左移累加；小端 → 直接整块拷贝。
/// 与 cpp/VM/src/lstrlib.cpp 中 bit 操作的装载逻辑一致。
///
/// # Safety
/// `buf.add(i)` 对 `startbyte..endbyte` 内所有 i 必须有效可读，
/// 调用方已做 `bitoffset + bitcount <= len * 8` 校验。
#[inline]
pub(crate) unsafe fn load_bits_u64(buf: *mut c_char, startbyte: usize, endbyte: usize) -> u64 {
  let mut data: u64 = 0;

  if LUAU_BIG_ENDIAN {
    for i in (startbyte..endbyte).rev() {
      data = (data << 8) + unsafe { *buf.add(i) } as u8 as u64;
    }
  } else {
    unsafe {
      copy_nonoverlapping(
        buf.add(startbyte),
        &mut data as *mut u64 as *mut c_char,
        endbyte - startbyte,
      );
    }
  }

  data
}
