use ulua_ast::{
  records::{
    ast_expr_global::AstExprGlobal, ast_expr_local::AstExprLocal, ast_node::AstNode,
    position::Position,
  },
  rtti::ast_node_as,
};

use crate::{
  functions::{
    find_binding_local_statement::find_binding_local_statement,
    find_expr_or_local_at_position::find_expr_or_local_at_position,
    find_scope_at_position::find_scope_at_position,
  },
  records::{binding::Binding, module::Module, source_module::SourceModule, symbol::Symbol},
};

pub fn find_binding_at_position(
  module: &Module,
  source: &SourceModule,
  pos: Position,
) -> Option<Binding> {
  let expr_or_local = find_expr_or_local_at_position(source, pos);

  let name = if !expr_or_local.expr.is_null() {
    unsafe {
      let global = ast_node_as::<AstExprGlobal>(expr_or_local.expr as *mut AstNode);
      if !global.is_null() {
        Symbol::symbol_ast_name((*global).name)
      } else {
        let local = ast_node_as::<AstExprLocal>(expr_or_local.expr as *mut AstNode);
        if !local.is_null() {
          Symbol::symbol_ast_local((*local).local)
        } else {
          return None;
        }
      }
    }
  } else if !expr_or_local.local.is_null() {
    Symbol::symbol_ast_local(expr_or_local.local)
  } else {
    return None;
  };

  let mut current_scope = find_scope_at_position(module, pos);

  while let Some(ref scope) = current_scope {
    if let Some(binding) = scope.bindings.get(&name)
      && binding.location.begin <= pos
    {
      // Ignore this binding if we're inside its definition. e.g. local abc = abc -- Will take the definition of abc from outer scope
      if let Some(binding_statement) = find_binding_local_statement(source, binding) {
        let stmt_location = unsafe { &*binding_statement }.base.base.location;
        if stmt_location.contains(pos) {
          current_scope = scope.parent.clone();
          continue;
        }
      }
      return Some(binding.clone());
    }

    current_scope = scope.parent.clone();
  }

  None
}
