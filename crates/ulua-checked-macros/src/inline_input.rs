use syn::{
  Error, Ident, LitStr, Result, Token,
  parse::{Parse, ParseStream},
};

use crate::fields::{self, CommonFields};

/// `ulua!` 宏的输入：裸字面量、`source = ...` 命名字段形式。
pub struct InlineInput {
  pub common: CommonFields,
}

impl Parse for InlineInput {
  fn parse(input: ParseStream<'_>) -> Result<Self> {
    if input.peek(LitStr) {
      return parse_literal_form(input);
    }

    Ok(Self {
      common: fields::parse_kv(input, "source")?,
    })
  }
}

/// `ulua!("...")` 及其 `defs` 附加形式（无 module/modules）。
fn parse_literal_form(input: ParseStream<'_>) -> Result<InlineInput> {
  let source = input.parse()?;
  let mut defs = None;

  if input.peek(Token![,]) {
    input.parse::<Token![,]>()?;
    let key: Ident = input.parse()?;
    if key != "defs" {
      return Err(Error::new(key.span(), "expected `defs`"));
    }
    input.parse::<Token![=]>()?;
    defs = Some(input.parse()?);
    if input.peek(Token![,]) {
      input.parse::<Token![,]>()?;
    }
  }

  if !input.is_empty() {
    return Err(input.error("unexpected tokens after inline Luau source"));
  }

  Ok(InlineInput {
    common: CommonFields {
      source,
      module: None,
      defs,
      modules: Vec::new(),
    },
  })
}
