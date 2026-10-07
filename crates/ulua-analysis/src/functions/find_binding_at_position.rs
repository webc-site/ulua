use ulua_ast::{
  records::{
    ast_expr_global::AstExprGlobal, ast_expr_local::AstExprLocal, node_handle::OptNode,
    position::Position,
  },
  rtti::ast_node_try_as,
};

use crate::{
  functions::{
    find_binding_local_statement::find_binding_local_statement,
    find_expr_or_local_at_position::find_expr_or_local_at_position,
    find_scope_at_position::find_scope_at_position,
  },
  records::{
    arena_handle::alias_ref, binding::Binding, module::Module, scope::Scope,
    scope_registry::resolve_scope, source_module::SourceModule, symbol::Symbol,
  },
};

pub fn find_binding_at_position(
  module: &Module,
  source: &SourceModule,
  pos: Position,
) -> Option<Binding> {
  let expr_or_local = find_expr_or_local_at_position(source, pos);

  let name = if !expr_or_local.expr.is_null() {
    // `expr` 槽位仍是裸指针：经句柄门面 `OptNode::from_ptr` 折叠可空性，
    // 判型下转走生命周期正确的 [`ast_node_try_as`]（未命中折叠为 None 且
    // 从不解引用），借用半径由本分支局部句柄供给，不再锻造假 'static。
    let expr_node = OptNode::from_ptr(expr_or_local.expr);
    if let Some(global) = expr_node
      .get()
      .and_then(|e| ast_node_try_as::<AstExprGlobal>(e))
    {
      Symbol::from_global(global.name)
    } else {
      let local = expr_node
        .get()
        .and_then(|e| ast_node_try_as::<AstExprLocal>(e))?;
      Symbol::from_local(local.local.as_ptr())
    }
  } else if !expr_or_local.local.is_null() {
    Symbol::from_local(expr_or_local.local)
  } else {
    return None;
  };

  let start_scope = find_scope_at_position(module, pos);
  // 句柄化上溯：起点为模块记录的 ScopePtr，父链经 resolve_scope 只读还原。
  let mut current_scope: Option<&Scope> = start_scope.as_deref();

  while let Some(scope) = current_scope {
    if let Some(binding) = scope.bindings.get(&name)
      && binding.location.begin <= pos
    {
      // Ignore this binding if we're inside its definition. e.g. local abc = abc -- Will take the definition of abc from outer scope
      if let Some(binding_statement) = find_binding_local_statement(source, binding) {
        let stmt_location = alias_ref(binding_statement).base.base.location;
        if stmt_location.contains(pos) {
          current_scope = scope.parent.and_then(resolve_scope);
          continue;
        }
      }
      return Some(binding.clone());
    }

    current_scope = scope.parent.and_then(resolve_scope);
  }

  None
}
