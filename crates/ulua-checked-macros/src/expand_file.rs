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

  let result = modules_check::check_dispatch(
    module.as_ref(),
    &root_source,
    &modules,
    defs_source.as_deref(),
    paths::read_manifest_file,
  );

  if let Err(failure) = result {
    return report::failure_error(root.span(), failure);
  }

  modules_check::expand_include_strs(&root, defs.as_ref(), &modules)
}
