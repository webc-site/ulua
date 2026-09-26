//! `std::pair<AstName, Lexeme::Type> AstNameTable::get_or_add_with_type(const char* name, size_t length)`
//! — Ast/src/Lexer.cpp:224.

use core::ptr::copy_nonoverlapping;

use crate::{
  enums::type_lexer::Type,
  records::{allocator::Allocator, ast_name::AstName, ast_name_table::AstNameTable, entry::Entry},
};

impl AstNameTable {
  /// Looks up or adds a name with the given length into the table.
  ///
  /// 批 2 存储面改型：cpp 形参 `const char* name`（Lexer.h:136）→ `*const u8`，
  /// 同一字节域，判定点（首字节 `'@'`、拷贝、补 NUL）逐位不变。
  ///
  /// # Safety
  /// `name` 必须指向至少 `length` 字节的已初始化内存。
  pub(crate) unsafe fn get_or_add_with_type(
    &mut self,
    name: *const u8,
    length: usize,
  ) -> (AstName, Type) {
    // Read the allocator pointer out before borrowing `self.data`, so the
    // in-place fixup below doesn't conflict with the `insert_mut` borrow.
    let allocator: *mut Allocator = self.allocator;

    let key = Entry {
      value: AstName {
        value: name,
        len: length as u32,
      },
      length: length as u32,
      r#type: Type::EOF,
    };

    let entry = self.data.insert_mut(key);

    // entry already was inserted
    if entry.r#type != Type::EOF {
      return (entry.value, entry.r#type);
    }

    // We just inserted an non-owned pointer into the map; we
    // need to correct it, *but* must not disturb the hash value, so we copy
    // the same bytes into an allocator-owned Buffer and repoint at it.
    // (C++ does this via `const_cast<Entry&>`; `insert_mut` is the faithful
    // Rust spelling of that mutation.)
    // Safety: `allocator` 是构造时从活的 `&mut Allocator` 接线、比本表长寿的 arena 裸指针（非空、对齐）；
    // `allocate(length + 1)` 返回一段至少 length+1 字节、起始对齐且地址稳定（arena 不移动已分配块）的内存。
    let name_data = unsafe { (*allocator).allocate(length + 1) };
    unsafe {
      // Safety: `name_data` 由上一步保证有 length+1 可写字节；`name` 依本函数 `/// # Safety` 契约指向至少
      // `length` 个已初始化字节。copy_nonoverlapping 两侧不重叠（源为调用方缓冲、目标为 arena 新块）、按 u8 对齐、
      // 拷贝 length 字节均在界内；随后 `*name_data.add(length)=0` 写入末尾 NUL 亦落在预留的第 length+1 字节。
      copy_nonoverlapping(name, name_data, length);
      *name_data.add(length) = 0;
    }

    // C++ 以 name[0] == '@' 区分属性名；空名（len==0，如 intern ""）时 C++
    // 读到结尾 NUL，Rust 侧空切片读首字节越界，按读得 '\0'（非 '@'）处理。
    // Safety: 仅在 `length != 0` 分支解引用 `name`，而本函数契约保证 `name` 指向至少 `length`(≥1) 个
    // 已初始化字节，故首字节可读。u8 与 cpp char 同字节域，`b'@'` 比较逐位同值。
    let first: u8 = if length != 0 { unsafe { *name } } else { 0 };

    entry.value = AstName {
      value: name_data.cast_const(),
      len: length as u32,
    };
    entry.r#type = if first == b'@' {
      Type::ATTRIBUTE
    } else {
      Type::NAME
    };

    (entry.value, entry.r#type)
  }
}
