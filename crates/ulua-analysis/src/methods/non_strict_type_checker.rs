//! `non_strict_type_checker` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use ulua_ast::records::{
  ast_expr::AstExpr, ast_node::AstNode, ast_type_pack::AstTypePack, location::Location,
};

use crate::{
  functions::{follow_type, follow_type_pack, get_type, push_module_scope::push_module_scope},
  records::{
    arena_handle::Handle, negation_type::NegationType, never_type::NeverType,
    non_strict_type_checker::NonStrictTypeChecker, scope::Scope, scope_registry::resolve_scope,
    stack_pusher::StackPusher, type_error::TypeError,
    type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::{type_error_data::TypeErrorData, type_id::TypeId, type_pack_id::TypePackId},
};

impl<'a> NonStrictTypeChecker<'a> {
  pub fn check_for_type_function_inhabitance(
    &mut self,
    _instance: TypeId,
    _location: Location,
  ) -> TypeId {
    _instance
  }
}

impl<'a> NonStrictTypeChecker<'a> {
  /// 沿模块 Scope 树向内收窄到包含 `location` 的最内层作用域。
  ///
  /// 返回 [`Handle<Scope>`]：模块根作用域由 `get_module_scope()` 返回的
  /// `Arc<Scope>` 克隆保活，children 经 `ScopeId` 注册表还原（scope_registry
  /// 契约），句柄只复制非空地址，解引用契约集中在 `arena_handle`。
  pub fn find_innermost_scope(&self, location: Location) -> Handle<Scope> {
    let module_scope = self.module_ref().get_module_scope();
    let mut best_scope: Handle<Scope> = Handle::from_ref(module_scope.as_ref());

    loop {
      let mut did_narrow = false;
      // `Scope.children: Vec<ScopeId>` 为原 NotNull<Scope*> 的句柄化，元素恒
      // 指向存活 Scope；`resolve_scope` 只读还原，全程共享借用无别名冲突。
      for &child_id in best_scope.get().children.iter() {
        let Some(child) = resolve_scope(child_id) else {
          continue;
        };
        if child.location.encloses(&location) {
          best_scope = Handle::from_ref(child);
          did_narrow = true;
          break;
        }
      }

      if !(did_narrow && !best_scope.get().children.is_empty()) {
        break;
      }
    }

    best_scope
  }
}

impl<'a> NonStrictTypeChecker<'a> {
  pub fn get_or_create_negation(&mut self, base_type: TypeId) -> TypeId {
    let cached_result = self.cached_negations.get_or_insert(base_type);
    if cached_result.is_null() {
      *cached_result = self
        .arena
        .get_mut()
        .add_type(NegationType { ty: base_type });
    }
    *cached_result
  }
}

impl<'a> NonStrictTypeChecker<'a> {
  pub fn lookup_pack_annotation(&self, annotation: &AstTypePack) -> Option<TypePackId> {
    let module = self.module_ref();
    // 指针仅作映射键（身份语义），不触碰解引用。
    let tp = module
      .ast_resolved_type_packs
      .find(&(annotation as *const AstTypePack));
    tp.map(|tp| follow_type_pack::follow(*tp))
  }
}

impl<'a> NonStrictTypeChecker<'a> {
  /// 对应 C++ `TypeId NonStrictTypeChecker::lookupType(AstExpr* expr)`
  /// (`cpp/Analysis/src/NonStrictTypeChecker.cpp:264`)。`expr` 为本次遍历期间
  /// 存活的 parse-arena 节点引用（非空由引用类型承载）；指针仅作
  /// `ast_types`/`ast_type_packs` 的映射键（身份语义）。
  pub fn lookup_type(&mut self, expr: &AstExpr) -> TypeId {
    let module = self.module_ref();

    if let Some(ty) = module.ast_types.find(&(expr as *const AstExpr)) {
      self.check_for_type_function_inhabitance(follow_type::follow(*ty), expr.base.location)
    } else if let Some(tp) = module.ast_type_packs.find(&(expr as *const AstExpr)) {
      let flattened = self.flatten_pack(*tp);
      self.check_for_type_function_inhabitance(flattened, expr.base.location)
    } else {
      self.builtin_types_ref().any_type
    }
  }
}

impl<'a> NonStrictTypeChecker<'a> {
  pub fn push_stack(&mut self, node: &AstNode) -> Option<StackPusher> {
    push_module_scope(self.module, &mut self.stack, node)
  }
}

impl<'a> NonStrictTypeChecker<'a> {
  pub fn report_error(&mut self, data: TypeErrorData, location: &Location) {
    let module = self.module_mut();
    module
      .errors
      .push(TypeError::type_error_location_module_name_type_error_data(
        *location,
        module.name.clone(),
        data,
      ));
  }
}

impl<'a> NonStrictTypeChecker<'a> {
  pub fn should_skip_runtime_error_testing(&mut self, test: TypeId) -> bool {
    let t = follow_type::follow(test);

    get_type::get::<NeverType>(t).is_some()
      || get_type::get::<TypeFunctionInstanceType>(t).is_some()
  }
}
