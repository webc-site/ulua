//! cpp `ReplRequirer.cpp` 的 `static write(std::optional<std::string> contents,
//! char* buffer, size_t bufferSize, size_t* sizeOut)`：require 配置回调用以
//! 把字符串写入 C 侧缓冲区（含结尾 NUL）。

use core::ffi::c_char;

use ulua_common::functions::c_slice::c_slice_mut;
use ulua_require::enums::luarequire_write_result::LuarequireWriteResult;

/// # Safety
///
/// `buffer` 在 `buffer_size` 内可写；`size_out` 可写一个 `usize`；
/// 二者由 `ulua-require` 的缓冲区协议保证。
pub(crate) unsafe fn write(
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
    // Safety: 上方已确认 size_out 非空
    unsafe { *size_out = null_terminated_size };
    return LuarequireWriteResult::WRITE_BUFFER_TOO_SMALL;
  }

  // Safety: 上方已确认 buffer 非空，且 buffer_size >= null_terminated_size，
  // ulua-require 缓冲区协议保证 buffer 有 buffer_size 字节可写空间
  let dst = unsafe { c_slice_mut(buffer as *mut u8, null_terminated_size) };
  // cpp `memcpy(buffer, contents->c_str(), nullTerminatedSize)`：连同结尾 NUL
  dst[..contents.len()].copy_from_slice(contents.as_bytes());
  dst[contents.len()] = 0;
  // Require.h:57-59：成功时 size_out 是写入的字节数；NUL 只是给 C 风格消费者的
  // 哨兵，不计入长度（cpp Navigation.cpp:155-158 据此 resize，不剥尾零）。
  // Safety: 上方已确认 size_out 非空
  unsafe { *size_out = contents.len() };

  LuarequireWriteResult::WRITE_SUCCESS
}
