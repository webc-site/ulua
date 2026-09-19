use alloc::vec::Vec;
use core::ffi::{c_char, c_void};

use crate::{
  enums::luarequire_write_result::luarequire_WriteResult, functions::c_str_prefix::with_c_str,
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

/// C writer 调用形态：无输入，或带一个字节串输入（仅在真 FFI 边界补 NUL）。
enum Writer<'a> {
  Plain(PlainWriter),
  WithInput(InputWriter, &'a [u8]),
}

impl Writer<'_> {
  /// 调用 C writer 写入缓冲区，返回写入结果。
  ///
  /// # Safety
  /// `l`/`ctx`/`buffer` 须为调用 C writer 所需的合法指针；
  /// writer 回调须按 `buffer_size` 约束写入并在 `size_out` 返回长度。
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
      Writer::WithInput(write, input) => with_c_str(input, |input| unsafe {
        write(l, ctx, input, buffer, buffer_size, size_out)
      }),
    }
  }
}

impl RuntimeNavigationContext<'_> {
  /// 通过 C writer 获取字节串：先按初始缓冲区调用，`BUFFER_TOO_SMALL` 时按
  /// `size_out` 扩容重试一次；成功时截去结尾 NUL 后原样返回字节。
  ///
  /// 与 cpp `getStringFromCWriter` 一致：结果按 `std::string` 字节返回，
  /// 不做 UTF-8 校验（chunkname/loadname/cache_key 可能是任意字节）。
  fn write_bytes(&self, writer: Writer<'_>, initial_buffer_size: usize) -> Option<Vec<u8>> {
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

    // cpp/Navigation.cpp:155-158 仅 `buffer.resize(size)`：size_out 即内容字节数，
    // 不剥任何结尾零。
    let end = size.min(buffer.len());
    buffer.truncate(end);
    Some(buffer)
  }

  pub(crate) fn get_string_from_c_writer(
    &self,
    writer: PlainWriter,
    initial_buffer_size: usize,
  ) -> Option<Vec<u8>> {
    self.write_bytes(Writer::Plain(writer), initial_buffer_size)
  }

  pub(crate) fn get_string_from_c_writer_with_input(
    &self,
    writer: InputWriter,
    input: &[u8],
    initial_buffer_size: usize,
  ) -> Option<Vec<u8>> {
    self.write_bytes(Writer::WithInput(writer, input), initial_buffer_size)
  }
}
