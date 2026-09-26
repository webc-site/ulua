use ulua_ast::records::ast_name_table::AstNameTable;

/// C++ `static void fillBuiltinGlobals(LintContext& context, const AstNameTable& names, const ScopePtr& env)`.
use crate::{
  records::{lint_context::LintContext, scope_registry::resolve_scope},
  type_aliases::scope_ptr_type::ScopePtr,
};
pub fn fill_builtin_globals(context: &mut LintContext, names: &AstNameTable, env: &ScopePtr) {
  let mut current = env.as_ref();
  loop {
    for (symbol, binding) in &current.bindings {
      let name = names.get_str(symbol.name());

      if name.is_null() {
        continue;
      }

      let global = context.builtin_globals.get_or_insert(name);
      global.r#type = binding.type_id;

      if binding.deprecated {
        global.deprecated = Some(binding.deprecated_suggestion.clone());
      }
    }

    match current.parent.and_then(resolve_scope) {
      Some(parent) => current = parent,
      None => break,
    }
  }
}
