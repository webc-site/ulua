use alloc::{string::String, vec::Vec};

use ulua_ast::{
  records::{
    ast_expr_index_name::AstExprIndexName, ast_stat::AstStat, ast_stat_block::AstStatBlock,
    ast_stat_declare_extern_type::AstStatDeclareExternType, ast_stat_function::AstStatFunction,
    ast_stat_local_function::AstStatLocalFunction, ast_stat_type_alias::AstStatTypeAlias,
    node_handle::Node as StatHandle,
  },
  rtti::ast_node_try_as,
};

use crate::{
  enums::control_flow::ControlFlow,
  functions::{
    contains_function_call_or_return::contains_function_call_or_return, follow_type,
    shared_mut::shared_mut, toposort::toposort,
  },
  records::{binding::Binding, symbol::Symbol, type_checker::TypeChecker},
  type_aliases::{collections::HashMap, scope_ptr_type::ScopePtr, type_id::TypeId},
};

impl TypeChecker {
  // cpp TypeInfer.cpp:494
  pub fn check_block_without_recursion_check(
    &mut self,
    scope: &ScopePtr,
    block: &AstStatBlock,
  ) -> ControlFlow {
    let mut sub_level: i32 = 0;

    let stats: Vec<StatHandle<AstStat>> = block.body.iter_nodes().copied().collect();
    let mut sorted = toposort(&stats);

    for &stat in &sorted {
      let stat_ref = stat.get();
      if let Some(typealias) = ast_node_try_as::<AstStatTypeAlias>(&stat_ref.base) {
        self.prototype_scope_ptr_ast_stat_type_alias_i32(scope.clone(), typealias, sub_level);
        sub_level += 1;
      } else if let Some(declared_extern_type) =
        ast_node_try_as::<AstStatDeclareExternType>(&stat_ref.base)
      {
        self.prototype_scope_ptr_ast_stat_declare_extern_type(scope.clone(), declared_extern_type);
      }
    }

    let mut check_iter: usize = 0;
    // 以 arena 句柄为键:句柄 `Eq`/`Hash` 按指针地址判等,与 cpp `unordered_map<AstStat*, ...>`
    // 的裸指针身份判等逐格同构,函数声明的登记/查回行为不变。
    let mut function_decls: HashMap<StatHandle<AstStat>, (TypeId, ScopePtr)> = HashMap::new();
    let mut first_flow: Option<ControlFlow> = None;

    for (proto_iter, &proto_stat) in sorted.iter().enumerate() {
      // 遍历侧收窄为 `&mut AstStat`：句柄 Copy 出本地可变槽后 `get_mut()` 物化
      // 独占借用（同 toposort `elements` 先例），调用结束借用即止；共享借用
      // `proto_ref` 在独占借用结束之后才再出借，与原 cpp 顺序裸指针透传同构。
      let mut proto_stat = proto_stat;
      let contains_call_or_return = contains_function_call_or_return(proto_stat.get_mut());
      let proto_ref = proto_stat.get();

      if contains_call_or_return {
        // 补齐 [check_iter, proto_iter) 的滞后区段，切片迭代与原逐个推进同序。
        for &stat in &sorted[check_iter..proto_iter] {
          self.check_body(scope, stat, &function_decls);
        }

        // We do check the current element, so advance checkIter beyond it.
        check_iter = proto_iter + 1;
        let flow = self.check_stat(scope, proto_ref);
        if flow != ControlFlow::None && first_flow.is_none() {
          first_flow = Some(flow);
        }
      } else if let Some(fun) = ast_node_try_as::<AstStatFunction>(&proto_ref.base) {
        let self_type: Option<TypeId> = None; // TODO clip
        let mut expected_type: Option<TypeId> = None;

        // `fun.func`/`fun.name` 已句柄化为 Node（parser 对全局函数声明必然二者
        // 齐备——cpp:596/601 直接解引用同名槽位；非空+arena 存活由句柄契约承载），
        // `.get()` 各绑定一个只读引用，块内不再经裸指针写，无并存可变借用。
        let func = fun.func.get();
        let name = fun.name.get();

        if func.self_.is_null()
          && let Some(index_name) = ast_node_try_as::<AstExprIndexName>(&name.base)
        {
          // AstExprIndexName.expr 已句柄化恒非空：.get() 安全借用（cpp:578-582
          // `indexName->expr` 直接传入 check）。
          let base_expr = index_name.expr.get();
          let expr_ty = self.check_expr(scope, base_expr, None, false);
          let index_name_str = index_name.index.as_str_or_empty().to_string();
          expected_type = self.get_index_type_from_type(
            scope.clone(),
            expr_ty.r#type,
            &index_name_str,
            &index_name.index_location,
            false,
          );
        }

        let pair = self.check_function_signature(
          scope,
          sub_level,
          func,
          Some(name.base.location),
          self_type,
          expected_type,
        );
        let fun_ty = pair.0;
        let fun_scope = pair.1.clone();

        function_decls.insert(proto_stat, pair);
        sub_level += 1;

        let left_type = follow_type::follow(self.check_function_name(scope, name, fun_scope.level));

        self.unify_type_id_type_id_scope_ptr_location(
          fun_ty,
          left_type,
          scope,
          &fun.base.base.location,
        );
      } else if let Some(fun) = ast_node_try_as::<AstStatLocalFunction>(&proto_ref.base) {
        // func/name 已句柄化为 Node（parser 必建非空，cpp:608 直接解引用同前提），
        // `.get()` 直出只读引用，无裸指针解引用。
        let func = fun.func.get();
        let name = fun.name.get();

        let pair =
          self.check_function_signature(scope, sub_level, func, Some(name.location), None, None);
        let fun_ty = pair.0;

        function_decls.insert(proto_stat, pair);
        sub_level += 1;

        // Scope 经 Arc 共享，cpp TypeInfer.cpp:615 `scope->bindings[fun->name] = {...}`
        // 是裸 Scope* 直写；Rust 侧照此以 shared_mut 取写穿句柄。
        // Safety: 单线程分析（库级契约）且此调用链独占 &mut self，按 shared_mut
        // 契约要求对应 Arc 在写期间存活；此刻作用域内不存在其他并存借用
        // （fun/func/name 皆指向 AST arena，与 Scope 对象无重叠），写入的
        // bindings 项在语句末完成，随后即弹出作用域。
        let scope_mut = { &mut *(shared_mut(scope)) };
        scope_mut.bindings.insert(
          Symbol::from_local(fun.name.as_ptr()),
          Binding {
            type_id: fun_ty,
            location: name.location,
            deprecated: false,
            deprecated_suggestion: String::new(),
            documentation_symbol: None,
          },
        );
      } else {
        let flow = self.check_stat(scope, proto_ref);
        if flow != ControlFlow::None && first_flow.is_none() {
          first_flow = Some(flow);
        }
      }
    }

    // 收尾：水位之后的剩余语句一次切片遍历补查。
    for &stat in &sorted[check_iter..] {
      self.check_body(scope, stat, &function_decls);
    }

    self.check_block_type_aliases(scope, &mut sorted);

    first_flow.unwrap_or(ControlFlow::None)
  }

  /// C++ `checkBody` lambda inside `checkBlockWithoutRecursionCheck`.
  fn check_body(
    &mut self,
    scope: &ScopePtr,
    stat: StatHandle<AstStat>,
    function_decls: &HashMap<StatHandle<AstStat>, (TypeId, ScopePtr)>,
  ) {
    let stat_ref = stat.get();
    if let Some(fun) = ast_node_try_as::<AstStatFunction>(&stat_ref.base) {
      let (fun_ty, fun_scope) = function_decls
        .get(&stat)
        .map(|(t, s)| (*t, s.clone()))
        .expect("functionDecls.count(stat)");
      self.check_stat_function(scope, fun_ty, &fun_scope, fun);
      return;
    }

    if let Some(fun_local) = ast_node_try_as::<AstStatLocalFunction>(&stat_ref.base) {
      let (fun_ty, fun_scope) = function_decls
        .get(&stat)
        .map(|(t, s)| (*t, s.clone()))
        .expect("functionDecls.count(stat)");
      self.check_stat_local_function(scope, fun_ty, &fun_scope, fun_local);
    }
  }
}
