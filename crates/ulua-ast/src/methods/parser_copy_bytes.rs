use core::ptr::copy_nonoverlapping;

use crate::records::{allocator::Allocator, ast_array::AstArray, parser::Parser};

impl Parser {
  pub fn copy_bytes(&mut self, data: &[u8]) -> AstArray<u8> {
    // C++ `copy(data.c_str(), data.size() + 1)` reads size()+1 bytes because
    // std::string::c_str() is NUL-terminated with a readable size()+1 Buffer.
    // Rust's `String` is NOT NUL-terminated and `String::as_ptr()` is a dangling
    // pointer when the string is empty, so copying `len()+1` bytes from the source
    // over-reads it (a guaranteed SIGSEGV on empty content, UB otherwise). Allocate
    // len+1, copy the `len` content bytes, and write the trailing NUL ourselves so
    // the result keeps c_str() semantics (NUL-terminated, logical size = len).
    let len = data.len();

    // Safety: &mut *self.allocator：allocator 是 Parser 构造时给出的非空可变 arena 指针且比 self 长寿；Allocator::allocate 恒非空（失败经 handle_alloc_error 直接中止）且块按 8 对齐，申请 len+1 字节足以容纳 u8（align 1）数组；源 data 为合法 &[u8] 切片，目标为新鲜分配与源必不重叠，copy len 字节后在偏移 len 写 NUL 仍在申请量内；单线程串行无别名。
    let storage = unsafe {
      let storage = Allocator::allocate(&mut *self.allocator, len + 1);

      if len > 0 {
        copy_nonoverlapping(data.as_ptr(), storage, len);
      }
      *storage.add(len) = 0;

      storage
    };

    // 分配失败已在 `allocate` 内中止，故这里直接成对写 data/size，不再先造 null 占位。
    AstArray {
      data: storage,
      size: len,
    }
  }
}
