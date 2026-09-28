use crate::records::{
  ast_array::{AstArray, AstArrayBuilder},
  parser::Parser,
};

impl Parser {
  pub fn copy_bytes(&mut self, data: &[u8]) -> AstArray<u8> {
    // C++ `copy(data.c_str(), data.size() + 1)` reads size()+1 bytes because
    // std::string::c_str() is NUL-terminated with a readable size()+1 Buffer.
    // Rust's `String` is NOT NUL-terminated and `String::as_ptr()` is a dangling
    // pointer when the string is empty, so copying `len()+1` bytes from the source
    // over-reads it (a guaranteed SIGSEGV on empty content, UB otherwise). Allocate
    // len+1, copy the `len` content bytes, and write the trailing NUL ourselves so
    // the result keeps c_str() semantics (NUL-terminated, logical size = len).
    //
    // 槽位申请与写入收口在 [`AstArrayBuilder`]（容量 `len+1`、内容 `len` 字节 +
    // 1 个 NUL 尾槽，`finish_with(len)` 把 NUL 留在逻辑区间外）——分配失败已在
    // `allocate` 内中止，逐槽写入的界检/初始化契约见该类型的 `push`/`push_slice`。
    let len = data.len();
    let mut slots = AstArrayBuilder::new(self.arena(), len + 1);
    slots.push_slice(data);
    slots.push(0);
    slots.finish_with(len)
  }
}
