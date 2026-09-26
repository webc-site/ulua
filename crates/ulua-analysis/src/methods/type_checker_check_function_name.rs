use alloc::string::String;
use core::ptr::null;

use ulua_ast::{
  enums::ast_expr_ref::AstExprRef,
  records::{ast_expr::AstExpr, location::Location},
};

use crate::{
  enums::table_state::TableState,
  functions::{arc_as_mut::arc_as_mut, get_mutable_table_type::get_mutable_table_type},
  records::{binding::Binding, symbol::Symbol, type_checker::TypeChecker, type_level::TypeLevel},
  type_aliases::{name_type::Name, scope_ptr_type::ScopePtr, type_id::TypeId},
};

impl TypeChecker {
  // Answers the question: "Can I define another function with this name?"
  // Primarily about detecting duplicates.
  pub fn check_function_name(
    &mut self,
    scope: &ScopePtr,
    fun_name: &AstExpr,
    level: TypeLevel,
  ) -> TypeId {
    match fun_name.as_expr_ref() {
      AstExprRef::Global(global_name) => {
        let module_scope = self.current_module.as_ref().expect("current_module 由 check_without_recursion_check 入口置入 Some、末尾才 take()，check 调用树内恒为 Some").get_module_scope();
        let name = Symbol::from_global(global_name.name);
        if module_scope.bindings.contains_key(&name) {
          if self.is_nonstrict_mode() {
            return module_scope
              .bindings
              .get(&name)
              .expect("上一行 contains_key 判定已命中")
              .type_id;
          }

          self.error_recovery_type_scope_ptr(scope)
        } else {
          let ty = self.fresh_type_type_level(level);
          let module_scope_ptr = arc_as_mut(&module_scope);
          let binding = Binding {
            type_id: ty,
            location: fun_name.base.location,
            deprecated: false,
            deprecated_suggestion: String::new(),
            documentation_symbol: None,
          };
          // Safety: `module_scope_ptr` 来自 `arc_as_mut(&module_scope)`，即 `Arc::as_ptr` 的
          // 可变视图，指向仍然存活的 `Scope`（`module_scope` 是本作用域内克隆出的 `Arc`，
          // 其引用计数保证对象活到本次写入结束）。写入 `bindings` 与函数其余部分对同一
          // Scope 的访问在单线程内串行发生，此刻没有其他 `&`/`&mut` 借用该 map（上面仅用
          // `contains_key`/`get` 完成的只读检查已经结束），故不产生别名冲突。
          unsafe {
            (*module_scope_ptr).bindings.insert(name, binding);
          }
          ty
        }
      }
      AstExprRef::Local(local_name) => {
        let name = Symbol::from_local(local_name.local.as_ptr());
        let scope_ptr = arc_as_mut(scope);
        // Binding& binding = scope->bindings[name];  — default-constructs (typeId == nullptr) if absent.
        // Safety: `scope_ptr = arc_as_mut(scope)` 是 `Arc<Scope>` 的可变视图，`scope` 由调用方
        // 以 `&ScopePtr` 借出并在本调用期间存活，故指向的 `Scope` 非空、对齐且不悬垂。
        // `entry().or_insert_with()` 返回的 `&mut Binding` 指向 map 桶内的值，地址随 map 稳定
        // （entry API 不触发重排），只活到本 if 块结束；期间对 `self` 的调用（fresh_type）不
        // 触及此 Scope，单线程串行下无第二处可变借用该 map。
        let binding = unsafe {
          (*scope_ptr)
            .bindings
            .entry(name)
            .or_insert_with(|| Binding {
              type_id: null(),
              location: fun_name.base.location,
              deprecated: false,
              deprecated_suggestion: String::new(),
              documentation_symbol: None,
            })
        };
        if binding.type_id.is_null() {
          *binding = Binding {
            type_id: self.fresh_type_type_level(level),
            location: fun_name.base.location,
            deprecated: false,
            deprecated_suggestion: String::new(),
            documentation_symbol: None,
          };
        }
        binding.type_id
      }
      AstExprRef::IndexName(index_name) => {
        let lhs_type = self
          .check_expr(
            scope,
            // index_name.expr 已句柄化恒非空（parser 构造 `AstExprIndexName` 时必填
            // expr 子节点）：.get() 只读借用仅供本次调用参数位置使用，无别名冲突。
            index_name.expr.get(),
            None,
            false,
          )
          .r#type;
        let ttv = get_mutable_table_type(lhs_type);

        // C++ TypeChecker.cpp（checkFunctionName）: if (!ttv || ttv->state == TableState::Sealed)
        if ttv.as_ref().is_none_or(|t| t.state == TableState::Sealed) {
          let name: Name = index_name.index.as_str_or_empty().to_string();
          if let Some(ty) = self.get_index_type_from_type(
            scope.clone(),
            lhs_type,
            &name,
            &index_name.index_location,
            false,
          ) {
            return ty;
          }

          return self.error_recovery_type_scope_ptr(scope);
        }

        let name: Name = index_name.index.as_str_or_empty().to_string();

        // 上面分支全部 return，走到这里 ttv 必为 Some 且非 Sealed。
        let ttv = ttv.expect("上方注释契约：所有 None/Sealed 分支均已 return，此处必为 Some");

        if ttv.props.contains_key(&name) {
          return ttv
            .props
            .get(&name)
            .expect("上一行 contains_key 判定已命中")
            .type_deprecated();
        }

        let fresh = self.fresh_type_type_level(level);
        let property = ttv.props.entry(name).or_default();
        property.set_type(fresh);
        property.location = Some(index_location_clone(&index_name.index_location));
        property.type_deprecated()
      }
      AstExprRef::Error(_) => self.error_recovery_type_scope_ptr(scope),
      _ => {
        self.ice_string_location("Unexpected AST node type", &fun_name.base.location);
        self.error_recovery_type_scope_ptr(scope)
      }
    }
  }
}

#[inline(always)]
fn index_location_clone(loc: &Location) -> Location {
  *loc
}
