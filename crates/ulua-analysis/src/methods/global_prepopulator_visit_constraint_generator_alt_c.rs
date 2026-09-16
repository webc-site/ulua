use alloc::string::String;

use ulua_ast::{
  records::{
    ast_expr_global::AstExprGlobal, ast_node::AstNode, ast_stat_function::AstStatFunction,
  },
  rtti::ast_node_as,
};

use crate::{
  records::{
    binding::Binding, blocked_type::BlockedType, global_prepopulator::GlobalPrepopulator,
    symbol::Symbol,
  },
  type_aliases::type_id::TypeId,
};
impl GlobalPrepopulator {
  /// # Safety
  /// 调用方须保证 `function` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_function(&mut self, function: *mut AstStatFunction) -> bool {
    let function_ref = unsafe { &*function };

    let name_expr = function_ref.name;
    if !name_expr.is_null() {
      // if (AstExprGlobal* g = function->name->as<AstExprGlobal>())
      let global = unsafe { ast_node_as::<AstExprGlobal>(name_expr as *mut AstNode) };

      if !global.is_null() {
        // TypeId bt = arena->addType(BlockedType{})
        let bt: TypeId = unsafe { (*self.arena.as_ptr()).add_type(BlockedType::default()) };

        let global_name = unsafe { (*global).name };
        // uninitializedGlobals.insert(g->name)
        self.uninitialized_globals.insert(global_name);

        let global_scope = self.global_scope.as_ptr();
        unsafe {
          // globalScope->bindings[g->name] = Binding{bt}
          (*global_scope).bindings.insert(
            Symbol::from_global(global_name),
            Binding {
              type_id: bt,
              location: function_ref.base.base.location,
              deprecated: false,
              deprecated_suggestion: String::new(),
              documentation_symbol: None,
            },
          );
        }
      }
    }

    true
  }
}
