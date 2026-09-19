//! Source: `Bytecode/src/BytecodeBuilder.cpp:676`
//!
//! Faithful port of `BytecodeBuilder::finalize`: assemble the final bytecode
//! blob — version byte, type-encoding version, string table, userdata type-name
//! mapping, then every function's pre-serialized `data` blob, then the main
//! function index. `bytecode` is taken out of `self` while writing so the
//! `&self` helpers (`write_string_table`) and `&mut self` field reads don't
//! alias the Buffer being filled.

use core::mem::take;

use ulua_common::{
  enums::luau_bytecode_tag::{
    LBC_TYPE_VERSION_MAX, LBC_TYPE_VERSION_MIN, LBC_VERSION_CLASSES, LBC_VERSION_MAX,
    LBC_VERSION_MIN,
  },
  fflag,
  macros::luau_assert::LUAU_ASSERT,
};

use crate::{
  functions::{write_byte::write_byte, write_var_int::write_var_int},
  records::{bytecode_builder::BytecodeBuilder, string_ref::StringRef},
};

impl BytecodeBuilder {
  pub fn finalize(&mut self) {
    LUAU_ASSERT!(self.bytecode.is_empty());

    // 已使用的 userdata 类型名先注册进字符串表。
    // 索引遍历为编译器所迫：add_string_table_entry 需要 &mut self，
    // 无法在 iter_mut 借用存活期间调用（StringRef 存裸指针，借用即止）。
    for i in 0..self.userdata_types.len() {
      if self.userdata_types[i].used {
        let sref = StringRef::from_slice(self.userdata_types[i].name.as_bytes());
        let entry = self.add_string_table_entry(sref);
        self.userdata_types[i].name_ref = entry;
      }
    }

    // preallocate space for bytecode blob
    let mut capacity: usize = 16;

    for (string_ref, _index) in self.string_table.iter() {
      capacity += string_ref.length + 2;
    }

    for func in &self.functions {
      capacity += func.data.len();
    }

    // assemble final bytecode blob — taken out of `self` so the Buffer is
    // not aliased by the `&self`/field reads below.
    let mut bytecode = take(&mut self.bytecode);
    bytecode.reserve(capacity);

    let version = self.get_version();
    // cpp 769: `(version >= LBC_VERSION_MIN && version <= LBC_VERSION_MAX) || version == LBC_VERSION_CLASSES`
    LUAU_ASSERT!(
      (version >= LBC_VERSION_MIN.0 as u8 && version <= LBC_VERSION_MAX.0 as u8)
        || version == LBC_VERSION_CLASSES.0 as u8
    );

    // 版本字节直接写入字节缓冲（Vec<u8>，无 UTF-8 不变量约束）。
    bytecode.push(version);

    let typesversion = self.get_type_encoding_version();
    LUAU_ASSERT!(
      typesversion >= LBC_TYPE_VERSION_MIN.0 as u8 && typesversion <= LBC_TYPE_VERSION_MAX.0 as u8
    );
    write_byte(&mut bytecode, typesversion);

    self.write_string_table(&mut bytecode);

    // Write the mapping between used type name indices and their name
    for (i, userdata_type) in self.userdata_types.iter().enumerate() {
      if userdata_type.used {
        write_byte(&mut bytecode, (i + 1) as u8);
        write_var_int(&mut bytecode, userdata_type.name_ref as u64);
      }
    }

    // 0 marks the end of the mapping
    write_byte(&mut bytecode, 0);

    write_var_int(&mut bytecode, self.functions.len() as u64);

    // cpp 797-802：四个 flag 任一开启时，每个函数体前写 VarInt 长度前缀，
    // 供读取方在应用该 flag 的增量（cost model / double 常量 / fastcall /
    // class shape）前定位函数边界。
    let size_prefix = fflag::LuauBytecodeCostModel.get()
      || fflag::LuauCompileEmitVectorDouble.get()
      || fflag::LuauCompileFastpcall.get()
      || fflag::DebugLuauUserDefinedClasses.get();

    for func in &self.functions {
      if size_prefix {
        write_var_int(&mut bytecode, func.data.len() as u64);
      }
      // func.data 的字节流原样写入字节缓冲。
      bytecode.extend_from_slice(&func.data);
    }

    LUAU_ASSERT!((self.main_function as usize) < self.functions.len());
    write_var_int(&mut bytecode, self.main_function as u64);

    self.bytecode = bytecode;
  }
}
