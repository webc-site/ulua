use alloc::{collections::BTreeMap, string::String, sync::Arc};

use crate::{
  enums::solver_mode::SolverMode,
  functions::accumulate_errors::accumulate_errors,
  records::{
    check_result::CheckResult, frontend::Frontend, internal_compiler_error::InternalCompilerError,
    source_node::SourceNode,
  },
  type_aliases::{module_name_type::ModuleName, module_ptr_module::ModulePtr},
};
impl Frontend {
  pub fn get_check_result(
    &self,
    name: &ModuleName,
    accumulate_nested: bool,
    for_autocomplete: bool,
  ) -> Option<CheckResult> {
    // C++: if (getLuauSolverMode() == SolverMode::New) forAutocomplete = false;
    let mut for_autocomplete = for_autocomplete;
    if self.get_luau_solver_mode() == SolverMode::New {
      for_autocomplete = false;
    }

    // C++: auto it = sourceNodes.find(name);
    //      if (it == sourceNodes.end() || it->second->hasDirtyModule(forAutocomplete)) return std::nullopt;
    {
      let node = self.source_nodes.get(name)?;
      if node.has_dirty_module(for_autocomplete) {
        return None;
      }
    }

    let resolver = if for_autocomplete {
      &self.module_resolver_for_autocomplete
    } else {
      &self.module_resolver
    };

    // C++: ModulePtr module = resolver.getModule(name);
    //      if (module == nullptr) throw InternalCompilerError(...)
    // In this port, ModulePtr = Arc<Module> (a non-nullable shared pointer),
    // so getModule never yields a null module; the nullptr branch is therefore
    // statically unreachable and the throw cannot fire. We still surface the
    // intended diagnostic shape by routing through getModule.
    let module: ModulePtr = resolver.get_module(name);
    let _ = || {
      // preserve the C++ diagnostic construction for reference/parity
      InternalCompilerError::internal_compiler_error_string_string(
        String::from("Frontend does not have module: "),
        name.clone(),
      )
    };

    let mut check_result = CheckResult::default();

    // C++: if (module->timeout) checkResult.timeoutHits.push_back(name);
    if module.timeout {
      check_result.timeout_hits.push(name.clone());
    }

    if accumulate_nested {
      // C++: checkResult.errors = accumulateErrors(sourceNodes, resolver, name);
      // `sourceNodes` is stored as a HashMap here but accumulateErrors takes a
      // std::unordered_map (modeled as BTreeMap); build that view.
      let source_nodes_map: BTreeMap<ModuleName, Arc<SourceNode>> = self
        .source_nodes
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
      check_result.errors = accumulate_errors(&source_nodes_map, resolver, name);
    } else {
      // C++: checkResult.errors.insert(end, module->errors.begin(), module->errors.end());
      check_result.errors.extend(module.errors.iter().cloned());
    }

    // C++: checkResult.lintResult = module->lintResult;
    check_result.lint_result = module.lint_result.clone();

    Some(check_result)
  }
}
