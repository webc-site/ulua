use proc_macro2::TokenStream;
use quote::quote;

use crate::{fields::CommonFields, inline_input::InlineInput, modules_check, report};

pub fn expand(tokens: TokenStream) -> TokenStream {
  let InlineInput {
    common: CommonFields {
      source,
      module,
      defs,
      modules,
    },
  } = match syn::parse2::<InlineInput>(tokens) {
    Ok(input) => input,
    Err(err) => return err.to_compile_error(),
  };

  let source_value = source.value();
  let defs_value = defs.as_ref().map(|defs| defs.value());
  // inline 宏的模块源即字面量本身，不涉及 IO，故不会有加载失败。
  let loaded: Vec<(String, String)> = modules
    .iter()
    .map(|entry| (entry.name.value(), entry.source_or_path.value()))
    .collect();
  let result = modules_check::check_dispatch(
    module.as_ref(),
    &source_value,
    &loaded,
    defs_value.as_deref(),
  );

  if let Err(diagnostics) = result {
    return report::diagnostics_error(source.span(), &diagnostics);
  }

  quote! { #source }
}
