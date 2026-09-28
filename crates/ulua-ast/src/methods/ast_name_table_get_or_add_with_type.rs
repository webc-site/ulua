//! `std::pair<AstName, Lexeme::Type> AstNameTable::get_or_add_with_type(const char* name, size_t length)`
//! — Ast/src/Lexer.cpp:224.
//!
//! cpp 形参的 `const char*`/`size_t` 成对入参折为单一 `&[u8]`：指针与长度不再
//! 可漂移，与只读门面 [`AstNameTable::get_with_type`] 完全对称（同一字节切片
//! 读法/写法两形态），被调侧因此不再是 `unsafe fn`。

use core::slice::from_raw_parts_mut;

use crate::{
  enums::type_lexer::Type,
  records::{ast_name::AstName, ast_name_table::AstNameTable, entry::Entry},
};

impl AstNameTable {
  /// 查表驻留一个名字：命中即返回既有 [`AstName`]，未命中则把字节拷进 arena
  /// 后登记。返回值同时给出词素族判定（名字 or 属性名）。
  pub(crate) fn get_or_add_with_type(&mut self, name: &[u8]) -> (AstName, Type) {
    // 先把分配器句柄取出，避免下面的 in-place 修正与 `insert_mut` 借用冲突。
    let mut allocator = self.allocator;
    let length = name.len();

    // 键透传入参地址值（不构造引用）：hash/eq 只看前 length 个字节
    // （`records::entry.rs`），未命中时下方立即把 entry 重指向 arena 副本。
    let key = Entry {
      value: AstName::from_raw_parts(name.as_ptr(), length as u32),
      length: length as u32,
      r#type: Type::EOF,
    };

    let entry = self.data.insert_mut(key);

    // 名字已在表内：不产生任何 arena 分配（cpp 同款早退）。
    if entry.r#type != Type::EOF {
      return (entry.value, entry.r#type);
    }

    // cpp 以 name[0] == '@' 区分属性名；空名（len == 0，如 intern ""）时 C++
    // 读到结尾 NUL，Rust 侧空切片无首字节，按读得 '\0'（非 '@'）处理。
    let first = name.first().copied().unwrap_or(0);

    // 新条目持有的是借用切片，源缓冲比本表长寿并不成立：把同样的字节拷进
    // arena 独占缓冲并补 NUL（cpp `strdup` 形态），哈希值随内容不变。
    // Safety: `allocator` 由 `new`/`rebind_allocator` 从活的 `&mut Allocator`
    // 接线、比本表长寿（NonNull 证非空）；`allocate(length + 1)` 返回一段长
    // 度≥length+1、起始对齐且地址稳定（bump arena 不移动已分配块）的区域，
    // 故 `from_raw_parts_mut` 的区间可写；`copy_from_slice` 两侧长度相等且不
    // 重叠（源为调用方切片、目标为 arena 新块）；末尾 NUL 落在第 length+1 字节。
    let name_data = unsafe { allocator.as_mut().allocate(length + 1) };
    unsafe {
      let buffer = from_raw_parts_mut(name_data, length + 1);
      buffer[..length].copy_from_slice(name);
      buffer[length] = 0;
    }

    entry.value = AstName::from_raw_parts(name_data.cast_const(), length as u32);
    entry.r#type = if first == b'@' {
      Type::ATTRIBUTE
    } else {
      Type::NAME
    };

    (entry.value, entry.r#type)
  }
}
