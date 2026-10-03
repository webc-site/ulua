//! `std::pair<AstName, Lexeme::Type> AstNameTable::get_or_add_with_type(const char* name, size_t length)`
//! — Ast/src/Lexer.cpp:224.
//!
//! cpp 形参的 `const char*`/`size_t` 成对入参折为单一 `&[u8]`：指针与长度不再
//! 可漂移，与只读门面 [`AstNameTable::get_with_type`] 完全对称（同一字节切片
//! 读法/写法两形态），被调侧因此不再是 `unsafe fn`。

use crate::{
  enums::type_lexer::Type,
  records::{
    ast_array::AstArrayBuilder, ast_name::AstName, ast_name_table::AstNameTable, entry::Entry,
  },
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
    // 「向 arena 未初始化槽写入」收口在 [`AstArrayBuilder`] 契约边界（`new` 申请
    // length+1 槽、`push_slice`/`push` 逐段写入并界检，尾槽落 NUL），本调用点
    // 只剩 NonNull 宿主槽位的单点解引用（`records/ast_name_table.rs` 字段契约）。
    // Safety: `allocator` 由 `new`/`rebind_allocator` 从活的 `&mut Allocator`
    // 接线、比本表长寿（NonNull 证非空）；造 `&mut` 期间无其它别名指向同一
    // arena。所得 `dup.data` 即 `allocate(length + 1)` 的同一块基址，旧
    // 「裸切片 + copy_from_slice + 尾 NUL」三步与 builder 逐位等价。
    let mut slots = AstArrayBuilder::new(unsafe { allocator.as_mut() }, length + 1);
    slots.push_slice(name);
    slots.push(0);
    let dup = slots.finish();

    entry.value = AstName::from_raw_parts(dup.as_ptr(), length as u32);
    entry.r#type = if first == b'@' {
      Type::ATTRIBUTE
    } else {
      Type::NAME
    };

    (entry.value, entry.r#type)
  }
}
