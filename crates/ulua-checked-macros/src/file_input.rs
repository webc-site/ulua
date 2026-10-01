use syn::{
  LitStr, Result,
  parse::{Parse, ParseStream},
};

use crate::fields::{self, CommonFields};

/// `ulua_file!` 宏的输入：裸路径字面量或 `root = ...` 命名字段形式。
pub(crate) struct FileInput {
  pub common: CommonFields,
}

impl Parse for FileInput {
  fn parse(input: ParseStream<'_>) -> Result<Self> {
    if input.peek(LitStr) {
      let root = input.parse()?;
      if !input.is_empty() {
        return Err(input.error("unexpected tokens after Luau file path"));
      }
      return Ok(Self {
        common: CommonFields {
          source: root,
          module: None,
          defs: None,
          modules: Vec::new(),
        },
      });
    }

    Ok(Self {
      common: fields::parse_kv(input, "root")?,
    })
  }
}
