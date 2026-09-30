use alloc::sync::Arc;

use ulua_analysis::{
  functions::attach_type_data::attach_type_data,
  records::{frontend::Frontend, module::Module},
  type_aliases::module_name_type::ModuleName,
};
use ulua_ast::functions::pretty_print_with_types_pretty_printer::pretty_print_with_types_ast_stat_block;
use ulua_cli_lib::functions::report_open_error::report_open_error;

use crate::{
  enums::report_format::ReportFormat,
  functions::{report_error::report_error, report_warning::report_warning},
};

/// C++ `static bool reportModuleResult(Frontend& frontend, const ModuleName& name, ReportFormat format, bool annotate)`
/// (`CLI/src/Analyze.cpp:91-129`).
pub(crate) fn report_module_result(
  frontend: &mut Frontend,
  name: &ModuleName,
  format: ReportFormat,
  annotate: bool,
) -> bool {
  // std::optional<CheckResult> cr = frontend.getCheckResult(name, false);
  // let-else 与参数解构合一
  let Some(cr) = frontend.get_check_result(name, false, false) else {
    eprintln!("Failed to find result for {name}");
    return false;
  };

  // if (!frontend.getSourceModule(name))
  if frontend.get_source_module(name).is_none() {
    report_open_error(name);
    return false;
  }

  // for (auto& error : cr->errors) reportError(frontend, format, error);
  for error in &cr.errors {
    report_error(frontend, format, error);
  }

  // std::string humanReadableName = frontend.fileResolver->getHumanReadableModuleName(name);
  let human_readable_name = frontend
    .file_resolver_ref()
    .get_human_readable_module_name(name);

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
    //
    // ulua-analysis 现以 `Option<Handle<SourceModule>>` 交付可变形态（步⑤），
    // `None` 即 cpp `getSourceModule` 返回 nullptr 的分支；`Module` 仍按 cpp
    // 约定以 Arc 共享指针交付（analysis 句柄化是独立既定工程，见 review 保留项），
    // 故 `attach_type_data` 的第二个实参仍需就地取 Arc 堆块地址。
    let Some(source_module) = frontend.get_source_module_mut(name) else {
      report_open_error(name);
      return false;
    };
    let module = frontend.module_resolver.get_module(name);

    // Safety: `source_module` 句柄指向 `frontend.source_modules` 表内存活的
    // SourceModule（本次 `&mut frontend` 借用期内交付）；`Arc::as_ptr(&module)`
    // 为 resolver 返回的 Arc<Module> 堆块地址，强引用存活至语句末。
    // `attachTypeData(*sm, *m)` 与 cpp 一致地经共享 Arc 原地写 Module（此刻该
    // Arc 为 resolver 表内的唯一交付实例，调用结束前不再有别的使用者）。
    unsafe {
      let source_module = source_module.get_mut();
      attach_type_data(source_module, &mut *(Arc::as_ptr(&module) as *mut Module));

      // std::string annotated = prettyPrintWithTypes(*sm->root);
      // Safety: `sm.root` 由 frontend 解析该模块时写入，源码模块存在即非空。
      let annotated = pretty_print_with_types_ast_stat_block(&mut *source_module.root);
      print!("{}", annotated);
    }
  }

  // return cr->errors.empty() && cr->lintResult.errors.empty();
  cr.errors.is_empty() && cr.lint_result.errors.is_empty()
}
