use core::{ffi::c_char, ptr::null_mut};

use crate::{
  functions::read_var_int::read_var_int,
  records::{t_string::tstring, temp_buffer::TempBuffer},
};

/// cpp `lvmload.cpp:57-61` `readString` 对应：id 为 0 表示「无字符串」。
///
/// 与 cpp 的差异：cpp 只靠 `TempBuffer::operator[]` 里的 `LUAU_ASSERT`
/// （release 直接编译掉，损坏字节码就是越界读），Rust 侧必须硬校验，故
/// 「id 的 varint 跑出 blob」与「id 越出字符串表」两类失败都返回 `None`，
/// 由 `loadsafe` 转成损坏字节码错误。
///
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn read_string(
  strings: &TempBuffer<*mut tstring>,
  data: *const c_char,
  size: usize,
  offset: &mut usize,
) -> Option<*mut tstring> {
  let id = unsafe { read_var_int(data, size, offset) }?;

  if id == 0 {
    return Some(null_mut());
  }

  let index = (id - 1) as usize;

  if index >= strings.count {
    return None;
  }

  // 上面已硬校验过边界，Index 内的 LUAU_ASSERT 恒真
  Some(strings[index])
}
