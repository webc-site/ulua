use alloc::string::String;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_stat_assign::AstStatAssign,
    ast_stat_function::AstStatFunction,
  },
  rtti::ast_node_try_as_ptr,
};

use crate::{
  records::{
    binding::Binding, blocked_type::BlockedType, global_prepopulator::GlobalPrepopulator,
    scope::Scope, symbol::Symbol,
  },
  type_aliases::type_id::TypeId,
};

impl GlobalPrepopulator {
  pub(crate) fn visit_ast_expr_global(&mut self, global: *mut AstExprGlobal) -> bool {
    // Safety: `global` 由 AstVisitor 分发经 `from_mut(node)` 从 visit 期内独占的
    // `&mut AstExprGlobal` 导出，非空、对齐、在整次 visit 期间存活；`(*global).name`
    // 是 AstName（Copy）字段读取。`global_scope`/`dfg` 为构造期接线的 NonNull 会话
    // 句柄（prepopulate 装配点以非空上游指针注入），比遍历长寿；as_mut/as_ref 重建
    // 的借用此刻无并存别名（单线程串行，本对象独占写 scope）。lookup_symbol 只读
    // 绑定表，get_def 是 dfg 只读查询，def 为按值 DefId 句柄。
    unsafe {
      let global_name = (*global).name;
      let scope = self.global_scope.as_mut();

      if let Some(ty) = scope.lookup_symbol(Symbol::from_global(global_name)) {
        let def = self.dfg.as_ref().get_def(global as *const AstExpr);
        *scope.lvalue_types.get_or_insert(def) = ty;
      }
    }

    true
  }

  pub(crate) fn visit_ast_stat_assign(&mut self, assign: *mut AstStatAssign) -> bool {
    // Safety: `assign` 由 AstVisitor 分发经 `from_mut` 从 visit 期内独占的
    // `&mut AstStatAssign` 导出，非空且存活于整次 visit；`vars` 是 parser 成对写入
    // 的 arena 数组，as_slice() 自带 null+0 区域证明，expr 由 ast_node_try_as_ptr 判型下转。
    // `self.arena` 是构造期接线 NonNull<TypeArena>，add_type 只向稳定 bump 块追加
    // BlockedType 节点，不影响后续 `(*expr).base.location` 读取的既有 AST 内存；
    // global_scope.as_mut() 重建的 &mut 在本单线程串行段内无并存借用。
    unsafe {
      let vars = (*assign).vars;
      for &expr in vars.as_slice() {
        let Some(global) = ast_node_try_as_ptr::<AstExprGlobal>(expr) else {
          continue;
        };

        let name = global.name;
        let sym = Symbol::from_global(name);

        let scope: &mut Scope = self.global_scope.as_mut();

        // if (!globalScope->lookup(g->name)) globalScope->globalsToWarn.insert(g->name.value)
        if scope.lookup_symbol(sym.clone()).is_none() {
          let name_str = name.as_str_or_empty().to_string();
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

  pub(crate) fn visit_ast_stat_function(&mut self, function: *mut AstStatFunction) -> bool {
    // Safety: `function` 由 AstVisitor 分发经 `from_mut` 从 visit 期内独占的
    // `&mut AstStatFunction` 导出，非空、对齐且存活于整次 visit，&* 重建共享借用
    // 只读其字段。
    let function_ref = unsafe { &*function };

    let name_expr = function_ref.name;
    if let Some(global) = unsafe { ast_node_try_as_ptr::<AstExprGlobal>(name_expr) } {
      // TypeId bt = arena->addType(BlockedType{})
      // Safety: self.arena 是构造期接线的 NonNull<TypeArena>（指向会话存活
      // bump arena）；add_type 只追加节点，块地址不移动，后续对 AST/global 的
      // 读取不受影响。
      let bt: TypeId = unsafe { (*self.arena.as_ptr()).add_type(BlockedType::default()) };

      let global_name = global.name;
      // uninitializedGlobals.insert(g->name)
      self.uninitialized_globals.insert(global_name);

      let global_scope = self.global_scope.as_ptr();
      // Safety: global_scope 源自构造期接线的 NonNull<Scope>（上游 arc_as_mut 的
      // 存活 Arc），as_ptr 保真；此刻无并存借用（单线程串行，本 pass 独占写），
      // bindings.insert 仅改 Scope 自有表。
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

    true
  }
}
