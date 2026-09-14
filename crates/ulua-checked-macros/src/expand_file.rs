use proc_macro2::TokenStream;

use crate::{fields::CommonFields, file_input::FileInput, modules_check, paths, report};

pub fn expand(tokens: TokenStream) -> TokenStream {
  let FileInput {
    common: CommonFields {
      source: root,
      module,
      defs,
      modules,
    },
  } = match syn::parse2::<FileInput>(tokens) {
    Ok(input) => input,
    Err(err) => return err.to_compile_error(),
  };

  let root_source = match paths::read_manifest_file(&root) {
    Ok(source) => source,
    Err(err) => return err.to_compile_error(),
  };
  let defs_source = match defs.as_ref().map(paths::read_manifest_file).transpose() {
    Ok(defs_source) => defs_source,
    Err(err) => return err.to_compile_error(),
  };

  let result = if modules.is_empty() && module.is_none() {
    match &defs_source {
      Some(defs) => ulua_rt::check_with_definitions(&root_source, defs),
      None => ulua_rt::check(&root_source),
    }
  } else {
    let root_module = module
      .as_ref()
      .map(|module| module.value())
      .unwrap_or_else(modules_check::default_module);
    modules_check::check(
      root_module,
      &root_source,
      &modules,
      defs_source.as_deref(),
      |lit| paths::read_manifest_file(lit).map_err(|err| err.to_string()),
    )
  };

  if let Err(diagnostics) = result {
    return report::diagnostics_error(root.span(), &diagnostics);
  }

  modules_check::expand_include_strs(&root, defs.as_ref(), &modules)
}
