use core::ptr::NonNull;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::type_lexer::Type,
  records::{allocator::Allocator, ast_name_table::AstNameTable, lexeme::K_RESERVED},
};

impl AstNameTable {
  pub fn new(allocator: &mut Allocator) -> Self {
    let mut table = Self {
      // C++ pre-sizes the set to 128 buckets; DenseHashSet::new grows on
      // demand from the empty sentinel (a non-observable difference).
      // 占位键走 `Entry::dense_default()`（null 名/0 长/EOF），与旧形内联字面量
      // 逐位等价；占用判定由位图负责，哨兵可存取。
      data: DenseHashSet::default(),
      // 入参引用即非空 + 存活证明（cpp 引用成员等价形态），safe 建槽。
      allocator: NonNull::from(allocator),
    };

    // 保留字是 `[RESERVED_BEGIN, RESERVED_END_TOKEN)` 的连续枚举值区间
    //（长度与 K_RESERVED 一致，由 `records/lexeme.rs` 的 const assert 钉死），
    // zip 逐项登记，免 index 回算。
    for (name, type_value) in K_RESERVED
      .into_iter()
      .zip(Type::RESERVED_BEGIN.0..Type::RESERVED_END_TOKEN.0)
    {
      table.add_static(name.as_bytes(), Type(type_value));
    }

    table
  }
}
