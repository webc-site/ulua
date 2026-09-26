use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::type_lexer::Type,
  records::{allocator::Allocator, ast_name_table::AstNameTable},
};

/// 21 个保留字：纯 Rust 静态切片，直接入表零分配，不再需要任何 C 风格 \0 终止符。
const K_RESERVED: [&[u8]; 21] = [
  b"and",
  b"break",
  b"do",
  b"else",
  b"elseif",
  b"end",
  b"false",
  b"for",
  b"function",
  b"if",
  b"in",
  b"local",
  b"nil",
  b"not",
  b"or",
  b"repeat",
  b"return",
  b"then",
  b"true",
  b"until",
  b"while",
];

impl AstNameTable {
  pub fn new(allocator: &mut Allocator) -> Self {
    let mut table = Self {
      // C++ pre-sizes the set to 128 buckets; DenseHashSet::new grows on
      // demand from the empty sentinel (a non-observable difference).
      // 占位键走 `Entry::dense_default()`（null 名/0 长/EOF），与旧形内联字面量
      // 逐位等价；占用判定由位图负责，哨兵可存取。
      data: DenseHashSet::default(),
      allocator: allocator as *mut Allocator,
    };

    // 保留字是 `[RESERVED_BEGIN, RESERVED_END_TOKEN)` 的连续枚举值区间
    //（21 个，与 K_RESERVED 长度一致），zip 逐项登记，免索引回算。
    for (index, name) in K_RESERVED.into_iter().enumerate() {
      table.add_static(name, Type(Type::RESERVED_BEGIN.0 + index as i32));
    }

    table
  }
}
