//! `BytecodeBuilder` 之 字符串表与 userdata 类型名的 intern/查询。

use std::{vec, vec::Vec};

use ulua_common::{enums::luau_bytecode_type::LuauBytecodeType, macros::luau_assert::LUAU_ASSERT};

use super::BytecodeBuilder;
use crate::{
  enums::dump_flags::DumpFlags,
  records::{string_ref::StringRef, userdata_type::UserdataType},
  type_aliases::string_table::StringTable,
};

// ── abs-r139：并自 `methods/bytecode_builder_add_string_table_entry.rs` ──
/// cpp `BytecodeBuilder::addStringTableEntry`（`Bytecode/src/BytecodeBuilder.cpp:319`）的实体。
///
/// 拆成自由函数是因为 `finalize` 需要在遍历 `userdataTypes` 的同时写表：把 `string_table` /
/// `debug_strings` / `dump_flags` 作为分离字段借出后即可避开 `&mut self` 与字段借用的冲突，
/// 不必为绕开借用检查而退回裸指针或索引循环。
pub(super) fn intern_string<'a>(
  string_table: &mut StringTable<'a>,
  debug_strings: &mut Vec<StringRef<'a>>,
  dump_flags: u32,
  value: StringRef<'a>,
) -> u32 {
  if let Some(idx) = string_table.find(&value) {
    return *idx;
  }

  // Bytecode serialization uses 1-based string-table indices (0 is reserved
  // to mean "no string"). C++ computes the index as `stringTable.size()`
  // *after* inserting the new entry via `operator[]`, i.e. pre-insert size+1.
  // Computing it pre-insert made the first string index 0, so
  // `debugStrings[valueString - 1]` underflowed in `dumpConstant`.
  let new_index = string_table.size() as u32 + 1;
  string_table.try_insert(value.clone(), new_index);

  if DumpFlags::Code.is_set(dump_flags) {
    debug_strings.push(value);
  }

  new_index
}

impl<'a> BytecodeBuilder<'a> {
  pub(crate) fn add_string_table_entry(&mut self, value: StringRef<'a>) -> u32 {
    intern_string(
      &mut self.string_table,
      &mut self.debug_strings,
      self.dump_flags,
      value,
    )
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_add_userdata_type.rs` ──
impl<'a> BytecodeBuilder<'a> {
  /// cpp `addUserDataTypes`：只登记名字**视图**（不拷贝），名字缓冲必须活得比 builder 久，
  /// 这条不变量由 `BytecodeBuilder<'a>`/`StringRef<'a>` 静态保证。
  pub fn add_userdata_type(&mut self, name: StringRef<'a>) -> u32 {
    let ty = UserdataType {
      name,
      ..Default::default()
    };

    self.userdata_types.push(ty);
    (self.userdata_types.len() - 1) as u32
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_clear_strings.rs` ──
impl<'a> BytecodeBuilder<'a> {
  /// cpp: `BytecodeBuilder::clearStrings`
  /// (`cpp/Bytecode/include/Luau/BytecodeBuilder.h:182`)
  pub fn clear_strings(&mut self) {
    self.debug_strings.clear();
    self.string_table.clear();
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_get_string_table.rs` ──
impl<'a> BytecodeBuilder<'a> {
  /// cpp `getStringTable()`（`Bytecode/src/BytecodeBuilder.cpp:3351`）：返回按 1 基索引
  /// 铺平的字符串表**原始字节**。Lua/Luau 字符串常量允许任意字节，这里不做任何
  /// UTF-8 判定，非法字节原样保留；需要显示时才由调用方 `from_utf8_lossy`。
  /// 未落位的槽（理论上不可达，`LUAU_ASSERT` 兜底）留 `&[]` 哨兵，
  /// 与 cpp `stringTable` 的空串语义一致，避免第二种非法串→空串改写路径扩散到调用方。
  pub fn get_string_table(&self) -> Vec<&[u8]> {
    let mut strings: Vec<&[u8]> = vec![b"".as_slice(); self.string_table.size()];

    for (string_ref, &index) in self.string_table.iter() {
      LUAU_ASSERT!(index > 0 && (index as usize) <= strings.len());
      if index > 0 && (index as usize) <= strings.len() {
        strings[index as usize - 1] = string_ref.as_bytes();
      }
    }
    strings
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_try_get_userdata_type_name.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub(crate) fn try_get_userdata_type_name(&self, type_: LuauBytecodeType) -> Option<&str> {
    // C++ `unsigned((type & ~LBC_TYPE_OPTIONAL_BIT) - LBC_TYPE_TAGGED_USERDATA_BASE)`: the
    // subtraction is done in (signed) int and cast to unsigned, so a non-userdata type wraps
    // to a huge index that fails the bounds check. The u16 subtraction here underflow-panicked.
    let index = ((type_.0 & !(LuauBytecodeType::LBC_TYPE_OPTIONAL_BIT.0)) as i32
      - LuauBytecodeType::LBC_TYPE_TAGGED_USERDATA_BASE.0 as i32) as u32;

    // C++ 返回 `userdataTypes[index].name` 这个 NUL 结尾的 `const char*`；这里按长度给出
    // `&str`（历史上返回裸指针后被调用方按 C 串语义读取，会越过未终止的字节读到
    // 相邻内存，多带一个杂散字节 —— compiler_debug_types 失败的根因）。
    self
      .userdata_types
      .get(index as usize)
      .and_then(|ty| ty.name.as_str().ok())
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_use_userdata_type.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn use_userdata_type(&mut self, index: u32) {
    LUAU_ASSERT!((index as usize) < self.userdata_types.len());
    self.userdata_types[index as usize].used = true;
  }
}
