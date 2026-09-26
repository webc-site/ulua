use crate::{
  enums::type_lexer::Type,
  records::{ast_name::AstName, ast_name_table::AstNameTable, entry::Entry},
};

impl AstNameTable {
  /// 登记静态字节串：静态数组地址恒定且比表长寿，直接以指针入表、不拷入 arena。
  pub(crate) fn add_static(&mut self, name: &'static [u8], r#type: Type) -> AstName {
    let entry = Entry {
      value: AstName {
        value: name.as_ptr(),
        len: name.len() as u32,
      },
      length: name.len() as u32,
      r#type,
    };

    ulua_common::LUAU_ASSERT!(!self.data.contains(&entry));
    self.data.insert(entry);

    entry.value
  }
}
