use alloc::{string::String, vec::Vec};

use ulua_ast::{
  records::{
    ast_expr_index_name::AstExprIndexName, ast_stat::AstStat, ast_stat_block::AstStatBlock,
    ast_stat_declare_extern_type::AstStatDeclareExternType, ast_stat_function::AstStatFunction,
    ast_stat_local_function::AstStatLocalFunction, ast_stat_type_alias::AstStatTypeAlias,
  },
  rtti::ast_node_try_as,
};

use crate::{
  enums::control_flow::ControlFlow,
  functions::{
    arc_as_mut::arc_as_mut, contains_function_call_or_return::contains_function_call_or_return,
    follow_type, toposort::toposort,
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

    let mut sorted: Vec<*mut AstStat> = block.body.iter_nodes().map(|n| n.as_ptr()).collect();
    toposort(&mut sorted);

    for &stat in &sorted {
      // Safety: `sorted` 的元素复制自 block.body（parser arena 分配的语句节点，
      // 分析期只读存活），toposort 只重排指针不改对象；此处再借用为只读引用，
      // 随后一律经安全的 ast_node_try_as 做 RTTI 甄别。
      let stat_ref = unsafe { &*stat };
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
    let mut function_decls: HashMap<*mut AstStat, (TypeId, ScopePtr)> = HashMap::new();
    let mut first_flow: Option<ControlFlow> = None;

    for (proto_iter, &proto_stat) in sorted.iter().enumerate() {
      // Safety: 同上——proto_stat 是 toposort 后数组里的 arena 存活语句指针，
      // 只读再借用；本循环内对它的全部写路径（如下方 scope 写穿）不触及 AST。
      let proto_ref = unsafe { &*proto_stat };

      if contains_function_call_or_return(proto_ref) {
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
        // 是裸 Scope* 直写；Rust 侧照此以 arc_as_mut 取写穿句柄。
        // Safety: 单线程分析（库级契约）且此调用链独占 &mut self，按 arc_as_mut
        // 契约要求对应 Arc 在写期间存活；此刻作用域内不存在其他并存借用
        // （fun/func/name 皆指向 AST arena，与 Scope 对象无重叠），写入的
        // bindings 项在语句末完成，随后即弹出作用域。
        let scope_mut = unsafe { &mut *(arc_as_mut(scope)) };
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
    stat: *mut AstStat,
    function_decls: &HashMap<*mut AstStat, (TypeId, ScopePtr)>,
  ) {
    // Safety: stat 来自上方同一 toposort 数组——arena 存活、分析期只读的语句节点；
    // 绑定为只读引用后，RTTI 甄别改用安全 ast_node_try_as。
    let stat_ref = unsafe { &*stat };
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
