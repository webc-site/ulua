use alloc::string::String;
use core::ptr::from_ref;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_stat_assign::AstStatAssign,
    ast_stat_function::AstStatFunction,
  },
  rtti::ast_node_try_as,
};

use crate::{
  records::{
    arena_handle::alias_ref, binding::Binding, blocked_type::BlockedType,
    global_prepopulator::GlobalPrepopulator, scope::Scope, symbol::Symbol,
  },
  type_aliases::type_id::TypeId,
};

impl GlobalPrepopulator {
  /// cpp `GlobalPrepopulator::visit(AstExprGlobal* global)`。`global` 由
  /// AstVisitor 分发从 visit 期内独占的 `&mut AstExprGlobal` 转交的共享借用
  /// （本 hook 只读其 `name`）；`scope_mut`/`dfg` 走记录侧收口访问器，
  /// lookup_symbol 只读绑定表，get_def 是 dfg 只读查询，def 为按值 DefId 句柄。
  pub(crate) fn visit_ast_expr_global(&mut self, global: &AstExprGlobal) -> bool {
    let global_name = global.name;
    let scope = self.scope_mut();

    if let Some(ty) = scope.lookup_symbol(Symbol::from_global(global_name)) {
      let def = self.dfg().get_def(from_ref(global).cast::<AstExpr>());
      *scope.lvalue_types.get_or_insert(def) = ty;
    }

    true
  }

  /// cpp `GlobalPrepopulator::visit(AstStatAssign* assign)`。`vars` 是 parser
  /// 成对写入的 arena 数组，as_slice() 自带 null+0 区域证明，元素经 alias_ref
  /// 收口后 ast_node_try_as 判型下转；`self.arena_mut()` 的 add_type 只向稳定
  /// bump 块追加 BlockedType 节点，不影响后续既有 AST 内存的读取。
  pub(crate) fn visit_ast_stat_assign(&mut self, assign: &AstStatAssign) -> bool {
    let vars = assign.vars;
    for &expr in vars.as_slice() {
      let Some(global) = ast_node_try_as::<AstExprGlobal>(alias_ref(expr)) else {
        continue;
      };

      let name = global.name;
      let sym = Symbol::from_global(name);

      let scope: &mut Scope = self.scope_mut();

      // if (!globalScope->lookup(g->name)) globalScope->globalsToWarn.insert(g->name.value)
      if scope.lookup_symbol(sym.clone()).is_none() {
        let name_str = name.as_str_or_empty().to_string();
        scope.globals_to_warn.insert(name_str);
      }

      if scope.bindings.contains_key(&sym) {
        continue;
      }

      // TypeId bt = arena->addType(BlockedType{})
      let bt_ty: TypeId = self.arena_mut().add_type(BlockedType::default());
      self.uninitialized_globals.insert(name);

      // globalScope->bindings[g->name] = Binding{bt, g->location}
      let binding = Binding {
        type_id: bt_ty,
        location: global.base.base.location,
        deprecated: false,
        deprecated_suggestion: String::new(),
        documentation_symbol: None,
      };

      scope.bindings.insert(sym, binding);
    }
    true
  }

  /// cpp `GlobalPrepopulator::visit(AstStatFunction* function)`。`function` 为
  /// AstVisitor 分发交出的共享借用，只读其字段；`name_expr` 经 alias_ref 收口
  /// 后 ast_node_try_as 判型下转（cpp `name->as<AstExprGlobal>()`）。
  pub(crate) fn visit_ast_stat_function(&mut self, function: &AstStatFunction) -> bool {
    let name_expr = function.name;
    if let Some(global) = ast_node_try_as::<AstExprGlobal>(name_expr.get()) {
      // TypeId bt = arena->addType(BlockedType{}); add_type 只追加节点，
      // 块地址不移动，后续对 AST/global 的读取不受影响。
      let bt: TypeId = self.arena_mut().add_type(BlockedType::default());

      let global_name = global.name;
      // uninitializedGlobals.insert(g->name)
      self.uninitialized_globals.insert(global_name);

      // globalScope->bindings[g->name] = Binding{bt}；scope_mut 为本 pass
      // 独占写窗口，bindings.insert 仅改 Scope 自有表。
      self.scope_mut().bindings.insert(
        Symbol::from_global(global_name),
        Binding {
          type_id: bt,
          location: function.base.base.location,
          deprecated: false,
          deprecated_suggestion: String::new(),
          documentation_symbol: None,
        },
      );
    }

    true
  }
}
