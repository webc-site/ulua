//! cpp `ReplRequirer.cpp` 的 `static write(std::optional<std::string> contents,
//! char* buffer, size_t bufferSize, size_t* sizeOut)`：require 配置回调用以
//! 把字符串写入 C 侧缓冲区（含结尾 NUL）。

use core::{ffi::c_char, slice::from_raw_parts_mut};

use ulua_require::enums::luarequire_write_result::LuarequireWriteResult;

/// # Safety
///
/// `buffer` 在 `buffer_size` 内可写；`size_out` 可写一个 `usize`；
/// 二者由 `ulua-require` 的缓冲区协议保证。
pub unsafe fn write(
  contents: Option<&str>,
  buffer: *mut c_char,
  buffer_size: usize,
  size_out: *mut usize,
) -> LuarequireWriteResult {
  let Some(contents) = contents else {
    return LuarequireWriteResult::WRITE_FAILURE;
  };

  // cpp 无条件 `*sizeOut = ...` 并 memcpy，指针为空即调用方违约（在 C++ 里是
  // UB）。这里不把它伪装成 WRITE_SUCCESS——那会让 require 侧以为读到了内容。
  if buffer.is_null() || size_out.is_null() {
    return LuarequireWriteResult::WRITE_FAILURE;
  }

  let null_terminated_size = contents.len() + 1;

  if buffer_size < null_terminated_size {
    // SAFETY: 上方已确认 size_out 非空
    unsafe { *size_out = null_terminated_size };
    return LuarequireWriteResult::WRITE_BUFFER_TOO_SMALL;
  }

  // SAFETY: 上方已确认 buffer 非空且 buffer_size >= null_terminated_size
  unsafe {
    let dst = from_raw_parts_mut(buffer as *mut u8, null_terminated_size);
    // cpp `memcpy(buffer, contents->c_str(), nullTerminatedSize)`：连同结尾 NUL
    dst[..contents.len()].copy_from_slice(contents.as_bytes());
    *size_out = null_terminated_size;
  }

  LuarequireWriteResult::WRITE_SUCCESS
}
