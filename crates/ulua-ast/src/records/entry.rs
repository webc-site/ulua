use ulua_common::{functions::c_slice::c_slice, records::dense_hash_table::DenseDefault};

use crate::{enums::type_lexer::Type, records::ast_name::AstName};

#[derive(Debug, Clone, Copy)]
pub struct Entry {
  pub value: AstName,
  pub length: u32,
  pub r#type: Type,
}

impl Entry {
  pub const fn new(value: AstName, length: u32, r#type: Type) -> Self {
    Self {
      value,
      length,
      r#type,
    }
  }
}

impl Default for Entry {
  fn default() -> Self {
    Entry {
      // 空名一律走 `AstName::new()` 单源，不在此重写 null 字面量。
      value: AstName::new(),
      length: 0,
      r#type: Type::EOF,
    }
  }
}

// 作为 `AstNameTable::data`（`DenseHashSet<Entry, EntryHash>`）键时的空槽占位值：
// 与 `Default` 同一构造（null 名 / 长度 0 / Type::EOF），对应调用点旧形
// `DenseHashSet::new(Entry { value: AstName{null}, 0, EOF })` 逐位等价。契约
// （见 ulua-common `dense_hash_table` 模块文档）：占用与否由位图判定、哨兵可存取；
// Entry 的哈希（EntryHash，FNV over 名字字节）与相等（PartialEq，长度+内容）
// 都只看名字不看待 `type`，即便表内驻留了内容等同占位值的零长名字条目，命中语义
// 也不受影响，故 `DenseHashSet::default()` 门面可安全使用。
impl DenseDefault for Entry {
  fn dense_default() -> Self {
    Entry::default()
  }
}

impl PartialEq for Entry {
  fn eq(&self, other: &Self) -> bool {
    if self.length != other.length {
      return false;
    }

    if self.value.value == other.value.value {
      return true;
    }

    // Safety: value/length 由 AstNameTable 插入时成对写入，指向合法字节区域；
    // 长度相等已在上方判定。
    unsafe {
      c_slice(self.value.value, self.length as usize)
        == c_slice(other.value.value, other.length as usize)
    }
  }
}

impl Eq for Entry {}
