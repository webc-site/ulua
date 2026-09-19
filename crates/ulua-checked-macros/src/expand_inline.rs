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
  let result = modules_check::check_dispatch(
    module.as_ref(),
    &source_value,
    &modules,
    defs_value.as_deref(),
    |lit| Ok(lit.value()),
  );

  if let Err(diagnostics) = result {
    return report::diagnostics_error(source.span(), &diagnostics);
  }

  quote! { #source }
}
