use alloc::sync::Arc;

use ulua_ast::records::ast_node::AstNode;

use crate::{
  functions::arc_as_mut::arc_as_mut,
  records::{
    arena_handle::alias, constraint_generator::ConstraintGenerator, scope::Scope,
    scope_registry::register_scope,
  },
  type_aliases::scope_ptr_type::ScopePtr,
};
impl ConstraintGenerator {
  /// C++ `ScopePtr ConstraintGenerator::childScope(AstNode* node, const ScopePtr& parent)`
  /// (`cpp/Analysis/src/ConstraintGenerator.cpp:522`)。
  ///
  /// `node` 为遍历方交出的存活 arena 节点共享借用（cpp 裸指针形参的 Rust 对应），
  /// 本函数只读取其 `location` 并以其地址作 `ast_scopes` 映射键。
  pub fn child_scope(&mut self, node: &AstNode, parent: &ScopePtr) -> ScopePtr {
    let scope: ScopePtr = Arc::new(Scope::new(parent, 0));
    // 注册发放句柄：父 children 存句柄；`scope_raw` 仍供 `ast_scopes` 裸指针映射。
    let scope_id = register_scope(&scope);
    let scope_raw = arc_as_mut(&scope);
    self.scopes.push((node.location, scope.clone()));

    let scope_mut = alias(scope_raw);
    scope_mut.location = node.location;
    scope_mut.return_type = parent.return_type;
    scope_mut.vararg_pack = parent.vararg_pack;

    alias(arc_as_mut(parent)).children.push(scope_id);

    if let Some(module) = &self.module {
      let module_ptr = arc_as_mut(module);
      *alias(module_ptr).ast_scopes.get_or_insert(&raw const *node) = scope_raw;
    }

    scope
  }
}
