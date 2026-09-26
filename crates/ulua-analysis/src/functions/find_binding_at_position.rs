use ulua_ast::{
  records::{ast_expr_global::AstExprGlobal, ast_expr_local::AstExprLocal, position::Position},
  rtti::ast_node_try_as_ptr,
};

use crate::{
  functions::{
    find_binding_local_statement::find_binding_local_statement,
    find_expr_or_local_at_position::find_expr_or_local_at_position,
    find_scope_at_position::find_scope_at_position,
  },
  records::{
    binding::Binding, module::Module, scope::Scope, scope_registry::resolve_scope,
    source_module::SourceModule, symbol::Symbol,
  },
};

pub fn find_binding_at_position(
  module: &Module,
  source: &SourceModule,
  pos: Position,
) -> Option<Binding> {
  let expr_or_local = find_expr_or_local_at_position(source, pos);

  let name = if !expr_or_local.expr.is_null() {
    // Safety: expr 已在上方判空非 null，指向 parse arena 存活的 repr(C) 表达式节点
    // （bump 分配、地址不移动，于本函数借用期内全程存活）；try_as_ptr 先判空再按
    // class_index 甄别，未命中返回 None 且从不解引用，命中即返回完整存活节点的
    // 只读借用。全程只读、单线程，无别名冲突。
    unsafe {
      if let Some(global) = ast_node_try_as_ptr::<AstExprGlobal>(expr_or_local.expr) {
        Symbol::from_global(global.name)
      } else {
        let local = ast_node_try_as_ptr::<AstExprLocal>(expr_or_local.expr)?;
        Symbol::from_local(local.local.as_ptr())
      }
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
        let stmt_location = unsafe { &*binding_statement }.base.base.location;
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
