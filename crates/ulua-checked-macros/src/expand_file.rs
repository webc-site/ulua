use proc_macro2::TokenStream;
use quote::quote;

use crate::{file_input::FileInput, paths, report};

pub fn expand(tokens: TokenStream) -> TokenStream {
  let input = match syn::parse2::<FileInput>(tokens) {
    Ok(input) => input,
    Err(err) => return err.to_compile_error(),
  };

  let root_source = match paths::read_manifest_file(&input.root) {
    Ok(source) => source,
    Err(err) => return err.to_compile_error(),
  };
  let defs_source = match &input.defs {
    Some(defs) => match paths::read_manifest_file(defs) {
      Ok(source) => Some(source),
      Err(err) => return err.to_compile_error(),
    },
    None => None,
  };

  let result = if input.modules.is_empty() && input.module.is_none() {
    if let Some(defs) = &defs_source {
      ulua_rt::check_with_definitions(&root_source, defs)
    } else {
      ulua_rt::check(&root_source)
    }
  } else {
    check_file_modules(&input, &root_source, defs_source.as_deref())
  };

  if let Err(diagnostics) = result {
    return report::diagnostics_error(input.root.span(), &diagnostics);
  }

  expand_include_strs(&input)
}

fn check_file_modules(
  input: &FileInput,
  root_source: &str,
  defs: Option<&str>,
) -> Result<(), Vec<ulua_rt::TypeDiagnostic>> {
  let root_module = input
    .module
    .as_ref()
    .map(|module| module.value())
    .unwrap_or_else(|| "main".to_string());
  let mut modules = Vec::with_capacity(input.modules.len() + 1);
  modules.push((root_module.clone(), root_source.to_string()));

  for module in &input.modules {
    let name = module.name.value();
    if name == root_module {
      return Err(vec![ulua_rt::TypeDiagnostic {
        module: Some(root_module),
        line: 1,
        column: 1,
        end_line: 1,
        end_column: 1,
        message: "module map duplicates the root module".to_string(),
        in_definitions: false,
      }]);
    }
    let source = match paths::read_manifest_file(&module.source_or_path) {
      Ok(source) => source,
      Err(err) => {
        return Err(vec![ulua_rt::TypeDiagnostic {
          module: Some(name),
          line: 1,
          column: 1,
          end_line: 1,
          end_column: 1,
          message: err.to_string(),
          in_definitions: false,
        }]);
      }
    };
    modules.push((name, source));
  }

  let borrowed: Vec<(&str, &str)> = modules
    .iter()
    .map(|(name, source)| (name.as_str(), source.as_str()))
    .collect();

  if let Some(defs) = defs {
    ulua_rt::check_modules_with_definitions(&root_module, &borrowed, defs)
  } else {
    ulua_rt::check_modules(&root_module, &borrowed)
  }
}

fn expand_include_strs(input: &FileInput) -> TokenStream {
  let root = paths::include_str_expr(&input.root);
  let mut dependencies = input
    .modules
    .iter()
    .map(|module| paths::include_str_expr(&module.source_or_path))
    .collect::<Vec<_>>();

  if let Some(defs) = &input.defs {
    dependencies.push(paths::include_str_expr(defs));
  }

  quote! {{
      #(const _: &str = #dependencies;)*
      #root
  }}
}
