use core::{ffi::CStr, ptr::null};

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::records::{
  allocator::Allocator, ast_name::AstName, ast_name_table::AstNameTable, entry::Entry, lexeme::Type,
};

const K_RESERVED: [&CStr; 21] = [
  c"and",
  c"break",
  c"do",
  c"else",
  c"elseif",
  c"end",
  c"false",
  c"for",
  c"function",
  c"if",
  c"in",
  c"local",
  c"nil",
  c"not",
  c"or",
  c"repeat",
  c"return",
  c"then",
  c"true",
  c"until",
  c"while",
];

impl AstNameTable {
  pub fn new(allocator: &mut Allocator) -> Self {
    let mut table = Self {
      // C++ pre-sizes the set to 128 buckets; DenseHashSet::new grows on
      // demand from the empty sentinel (a non-observable difference).
      data: DenseHashSet::new(Entry {
        value: AstName { value: null() },
        length: 0,
        r#type: Type::EOF,
      }),
      allocator: allocator as *mut Allocator,
    };

    for i in (Type::RESERVED_BEGIN.0)..(Type::RESERVED_END_TOKEN.0) {
      let index = (i - Type::RESERVED_BEGIN.0) as usize;
      table.add_static(K_RESERVED[index], Type(i));
    }

    table
  }
}

pub fn ast_name_table_ast_name_table(allocator: &mut Allocator) -> AstNameTable {
  AstNameTable::new(allocator)
}
