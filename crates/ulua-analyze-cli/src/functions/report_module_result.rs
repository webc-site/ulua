use alloc::sync::Arc;

use ulua_analysis::{
  functions::attach_type_data::attach_type_data,
  records::{file_resolver::FileResolver, frontend::Frontend, module::Module},
  type_aliases::module_name_type::ModuleName,
};
use ulua_ast::functions::pretty_print_with_types_pretty_printer_alt_b::pretty_print_with_types_ast_stat_block;

use crate::{
  enums::report_format::ReportFormat,
  functions::{report_error::report_error, report_warning::report_warning},
};

/// C++ `static bool reportModuleResult(Frontend& frontend, const ModuleName& name, ReportFormat format, bool annotate)`
/// (`CLI/src/Analyze.cpp:91-129`).
pub fn report_module_result(
  frontend: &mut Frontend,
  name: &ModuleName,
  format: ReportFormat,
  annotate: bool,
) -> bool {
  // std::optional<CheckResult> cr = frontend.getCheckResult(name, false);
  let cr = frontend.get_check_result(name, false, false);

  let cr = match cr {
    None => {
      eprintln!("Failed to find result for {}", name);
      return false;
    }
    Some(cr) => cr,
  };

  // if (!frontend.getSourceModule(name))
  if frontend.get_source_module(name).is_null() {
    eprintln!("Error opening {}", name);
    return false;
  }

  // for (auto& error : cr->errors) reportError(frontend, format, error);
  for error in &cr.errors {
    report_error(frontend, format, error);
  }

  // std::string humanReadableName = frontend.fileResolver->getHumanReadableModuleName(name);
  let human_readable_name =
    unsafe { FileResolver::get_human_readable_module_name(frontend.file_resolver, name) };

  // for (auto& error : cr->lintResult.errors) reportWarning(format, humanReadableName.c_str(), error);
  for error in &cr.lint_result.errors {
    report_warning(format, &human_readable_name, error);
  }
  // for (auto& warning : cr->lintResult.warnings) reportWarning(format, humanReadableName.c_str(), warning);
  for warning in &cr.lint_result.warnings {
    report_warning(format, &human_readable_name, warning);
  }

  if annotate {
    // SourceModule* sm = frontend.getSourceModule(name);
    // ModulePtr m = frontend.moduleResolver.getModule(name);
    let sm = frontend.get_source_module_mut(name);
    let module = frontend.module_resolver.get_module(name);

    // attachTypeData(*sm, *m);
    // ModulePtr is Arc<Module>; attachTypeData mutates the Module, mirroring the
    // C++ `*m` dereference of the shared module pointer.
    let module_ptr = Arc::as_ptr(&module) as *mut Module;
    unsafe {
      attach_type_data(&mut *sm, &mut *module_ptr);

      // std::string annotated = prettyPrintWithTypes(*sm->root);
      let annotated = pretty_print_with_types_ast_stat_block(&mut *(*sm).root);

      // printf("%s", annotated.c_str());
      print!("{}", annotated);
    }
  }

  // return cr->errors.empty() && cr->lintResult.errors.empty();
  cr.errors.is_empty() && cr.lint_result.errors.is_empty()
}
