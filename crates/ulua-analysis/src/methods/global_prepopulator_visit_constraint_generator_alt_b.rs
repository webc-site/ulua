use alloc::string::String;
use core::ffi::CStr;

use ulua_ast::{
  records::{ast_expr_global::AstExprGlobal, ast_node::AstNode, ast_stat_assign::AstStatAssign},
  rtti::ast_node_as,
};

use crate::{
  records::{
    binding::Binding, blocked_type::BlockedType, global_prepopulator::GlobalPrepopulator,
    scope::Scope, symbol::Symbol,
  },
  type_aliases::type_id::TypeId,
};
impl GlobalPrepopulator {
  /// # Safety
  /// 调用方须保证 `assign` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_assign(&mut self, assign: *mut AstStatAssign) -> bool {
    unsafe {
      let vars = (*assign).vars;
      for &expr in vars.as_slice() {
        if expr.is_null() {
          continue;
        }

        let global = ast_node_as::<AstExprGlobal>(expr as *mut AstNode);
        if global.is_null() {
          continue;
        }

        let name = (*global).name;
        let sym = Symbol::from_global(name);

        let scope: &mut Scope = self.global_scope.as_mut();

        // if (!globalScope->lookup(g->name)) globalScope->globalsToWarn.insert(g->name.value)
        if scope.lookup_symbol(sym.clone()).is_none() {
          let name_str = CStr::from_ptr(name.value).to_string_lossy().into_owned();
          scope.globals_to_warn.insert(name_str);
        }

        if scope.bindings.contains_key(&sym) {
          continue;
        }

        // TypeId bt = arena->addType(BlockedType{})
        let bt_ty: TypeId = (*self.arena.as_ptr()).add_type(BlockedType::default());
        self.uninitialized_globals.insert(name);

        // globalScope->bindings[g->name] = Binding{bt, g->location}
        let binding = Binding {
          type_id: bt_ty,
          location: (*expr).base.location,
          deprecated: false,
          deprecated_suggestion: String::new(),
          documentation_symbol: None,
        };

        scope.bindings.insert(sym, binding);
      }
    }
    true
  }
}
