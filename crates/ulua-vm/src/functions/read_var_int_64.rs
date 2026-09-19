use core::ffi::c_char;

/// # Safety
///
/// 指针参数必须有效、对齐且指向存活对象（长度 size 的字节缓冲）。
pub(crate) unsafe fn read_var_int_64(data: *const c_char, size: usize, offset: &mut usize) -> u64 {
  // 单一实现收敛到 ulua-common（cpp 仅 BytecodeWire.cpp 一份）
  let bytes = unsafe { core::slice::from_raw_parts(data as *const u8, size) };
  ulua_common::functions::read_var_int_64::read_var_int_64(bytes, offset)
}
