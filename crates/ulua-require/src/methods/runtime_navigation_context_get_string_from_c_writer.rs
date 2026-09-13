use alloc::{ffi::CString, string::String};
use core::ffi::{CStr, c_char, c_void};

use crate::{
  enums::luarequire_write_result::luarequire_WriteResult,
  records::runtime_navigation_context::RuntimeNavigationContext,
};

type PlainWriter = unsafe extern "C-unwind" fn(
  *mut c_void,
  *mut c_void,
  *mut c_char,
  usize,
  *mut usize,
) -> luarequire_WriteResult;

type InputWriter = unsafe extern "C-unwind" fn(
  *mut c_void,
  *mut c_void,
  *const c_char,
  *mut c_char,
  usize,
  *mut usize,
) -> luarequire_WriteResult;

/// C writer 调用形态：无输入，或带一个 C 字符串输入。
enum Writer<'a> {
  Plain(PlainWriter),
  WithInput(InputWriter, &'a CStr),
}

impl Writer<'_> {
  /// 调用 C writer 写入缓冲区，返回写入结果。
  unsafe fn call(
    &self,
    l: *mut c_void,
    ctx: *mut c_void,
    buffer: *mut c_char,
    buffer_size: usize,
    size_out: &mut usize,
  ) -> luarequire_WriteResult {
    match *self {
      Writer::Plain(write) => unsafe { write(l, ctx, buffer, buffer_size, size_out) },
      Writer::WithInput(write, input) => unsafe {
        write(l, ctx, input.as_ptr(), buffer, buffer_size, size_out)
      },
    }
  }
}

impl RuntimeNavigationContext {
  /// 通过 C writer 获取字符串：先按初始缓冲区调用，`BUFFER_TOO_SMALL` 时按
  /// `size_out` 扩容重试一次；成功时截去结尾 NUL 并转 `String`。
  fn write_string(&self, writer: Writer<'_>, initial_buffer_size: usize) -> Option<String> {
    let mut buffer = vec![0; initial_buffer_size];
    let mut size: usize = 0;

    let mut result = unsafe {
      writer.call(
        self.l,
        self.ctx,
        buffer.as_mut_ptr() as *mut c_char,
        buffer.len(),
        &mut size,
      )
    };
    if result == luarequire_WriteResult::WriteBufferTooSmall {
      buffer.resize(size, 0);
      result = unsafe {
        writer.call(
          self.l,
          self.ctx,
          buffer.as_mut_ptr() as *mut c_char,
          buffer.len(),
          &mut size,
        )
      };
    }

    if result != luarequire_WriteResult::WriteSuccess {
      return None;
    }

    // 兼容 size_out 含结尾 NUL 的 writer：截去末尾的 '\0'（end-1 一定在界内）。
    let mut end = size.min(buffer.len());
    if end > 0 && unsafe { *buffer.get_unchecked(end - 1) } == 0 {
      end -= 1;
    }
    buffer.truncate(end);
    String::from_utf8(buffer).ok()
  }

  pub(crate) fn get_string_from_c_writer(
    &self,
    writer: PlainWriter,
    initial_buffer_size: usize,
  ) -> Option<String> {
    self.write_string(Writer::Plain(writer), initial_buffer_size)
  }

  pub(crate) fn get_string_from_c_writer_with_input(
    &self,
    writer: InputWriter,
    input: &str,
    initial_buffer_size: usize,
  ) -> Option<String> {
    let c_input = CString::new(input).ok()?;
    self.write_string(Writer::WithInput(writer, &c_input), initial_buffer_size)
  }
}
