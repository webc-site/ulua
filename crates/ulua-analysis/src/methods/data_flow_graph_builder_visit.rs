use alloc::vec::Vec;
use core::ptr::from_ref;

use ulua_ast::{
  enums::{ast_expr_ref::AstExprRef, ast_stat_ref::AstStatRef},
  records::{
    ast_class_method::AstClassMethod, ast_class_property::AstClassProperty, ast_stat::AstStat,
    ast_stat_assign::AstStatAssign, ast_stat_block::AstStatBlock, ast_stat_break::AstStatBreak,
    ast_stat_class::AstStatClass, ast_stat_compound_assign::AstStatCompoundAssign,
    ast_stat_continue::AstStatContinue, ast_stat_declare_extern_type::AstStatDeclareExternType,
    ast_stat_declare_function::AstStatDeclareFunction,
    ast_stat_declare_global::AstStatDeclareGlobal, ast_stat_error::AstStatError,
    ast_stat_expr::AstStatExpr, ast_stat_for::AstStatFor, ast_stat_for_in::AstStatForIn,
    ast_stat_function::AstStatFunction, ast_stat_if::AstStatIf, ast_stat_local::AstStatLocal,
    ast_stat_local_function::AstStatLocalFunction, ast_stat_repeat::AstStatRepeat,
    ast_stat_return::AstStatReturn, ast_stat_type_alias::AstStatTypeAlias,
    ast_stat_type_function::AstStatTypeFunction, ast_stat_while::AstStatWhile,
  },
};
use ulua_common::{LUAU_ASSERT, fflag};

use crate::{
  enums::{control_flow::ControlFlow, scope_type::ScopeType},
  functions::{
    arena_ref::arena_ref, contains_subscripted_definition::contains_subscripted_definition,
    does_call_error::does_call_error, matches::matches,
  },
  records::{
    arena_handle::{alias, alias_opt, alias_ref},
    data_flow_graph_builder::DataFlowGraphBuilder,
    dfg_scope::DfgScope,
    push_scope::PushScope,
    symbol::Symbol,
  },
  type_aliases::def_id_def::DefId,
};

impl DataFlowGraphBuilder {
  /// 栈顶 scope 可变物化的唯一收口点：经 `arena_handle::alias` 的契约门面重建
  /// 独占借用。前提（各调用点成立）：scope 单元存于 builder 自有
  /// `PinnedStorage<DfgScope>`（地址稳定、活至 builder 析构），`build()` 单线程
  /// 串行驱动；`with` 回调体只允许对 scope 的 bindings/props/inherit 做单次短
  /// 写，不得持借用重入操作 scope 树或栈的 builder 方法。
  fn with_current_scope_mut<R>(&mut self, with: impl FnOnce(&mut DfgScope) -> R) -> R {
    let scope = self.current_scope();
    with(alias(scope))
  }

  /// `bindings[symbol] = def` 写栈顶 scope 的收口点（安全论证见
  /// [`Self::with_current_scope_mut`]；对应 cpp `currentScope()->bindings[symbol] = def`）。
  fn bind_current_scope(&mut self, symbol: Symbol, def: DefId) {
    self.with_current_scope_mut(|scope| {
      *scope.bindings.get_or_insert(symbol) = def;
    });
  }

  /// 同上，落点为指定的 `make_child_scope` 活单元（如签名作用域）。
  fn bind_scope(&mut self, scope: *mut DfgScope, symbol: Symbol, def: DefId) {
    *alias(scope).bindings.get_or_insert(symbol) = def;
  }

  /// 把 `child` 的 bindings/props 吸回栈顶 scope（cpp
  /// `currentScope()->inherit(childScope)`）。`child` 为 PinnedStorage 活单元，
  /// `alias_ref` 只读取快照、与栈顶写借用不重叠（`inherit` 先物化 child 拷贝再
  /// 写 self，见 methods/dfg_scope.rs）。
  fn inherit_into_current_scope(&mut self, child: *const DfgScope) {
    self.with_current_scope_mut(|scope| scope.inherit(alias_ref(child)));
  }

  /// `join` 的调用点收口（本文件四个 visit 分支共用）：p 恒取弹栈后的栈顶
  /// （cpp `join(currentScope(), ...)` 家族），a/b 为本帧 `make_child_scope` 划出
  /// 的 PinnedStorage 活单元；循环可零次执行形态的 p==a 重叠由 join 实现的
  /// 「先快照 a/b 再写 p」消化（契约见 `DataFlowGraphBuilder::join`）。
  fn join_into_current(&mut self, a: *mut DfgScope, b: *mut DfgScope) {
    let p = self.current_scope();
    // SAFETY: p 经 current_scope、a/b 经 make_child_scope 产出，均为 builder
    // 自有 PinnedStorage 的存活非空 scope 单元（地址稳定、单线程独占），满足
    // join 的前置契约；p 与 a 重叠（while/for/forin 形态）在契约显式容忍内。
    unsafe { self.join(p, a, b) };
  }
}

impl DataFlowGraphBuilder {
  /// cpp `visit(AstStatBlock*)`：压入线性子作用域遍历语句表，再向父帧继承。
  pub(crate) fn visit_stat_block(&mut self, b: &AstStatBlock) -> ControlFlow {
    let child = self.make_child_scope(ScopeType::Linear);

    let cf;
    {
      let _ps = PushScope::new(&mut self.scope_stack, child);
      // child 已被 PushScope 压栈，正是 visit_block_without_child_scope 期望的
      // 当前作用域（cpp visit(AstStatBlock*) 同序）。
      cf = self.visit_block_without_child_scope(b);
    }

    // child 已弹栈（父即调用前作用域），经 inherit_into_current_scope 收口点
    // 把 child 快照吸回栈顶（两借用不重叠，契约见该方法）。
    self.inherit_into_current_scope(child);
    cf
  }

  /// cpp `DataFlowGraphBuilder::visit(AstStat*)` 的分派入口。
  pub fn visit_stat(&mut self, s: &AstStat) -> ControlFlow {
    match s.as_stat_ref() {
      AstStatRef::Block(b) => self.visit_stat_block(b),
      AstStatRef::If(i) => self.visit_stat_if(i),
      AstStatRef::While(w) => self.visit_stat_while(w),
      AstStatRef::Repeat(r) => self.visit_stat_repeat(r),
      AstStatRef::Break(b) => self.visit_stat_break(b),
      AstStatRef::Continue(c) => self.visit_stat_continue(c),
      AstStatRef::Return(r) => self.visit_stat_return(r),
      AstStatRef::Expr(e) => self.visit_stat_expr(e),
      AstStatRef::Local(l) => self.visit_stat_local(l),
      AstStatRef::For(f) => self.visit_stat_for(f),
      AstStatRef::ForIn(f) => self.visit_stat_for_in(f),
      AstStatRef::Assign(a) => self.visit_stat_assign(a),
      AstStatRef::CompoundAssign(c) => self.visit_stat_compound_assign(c),
      AstStatRef::Function(f) => self.visit_stat_function(f),
      AstStatRef::LocalFunction(l) => self.visit_stat_local_function(l),
      AstStatRef::TypeAlias(t) => self.visit_stat_type_alias(t),
      AstStatRef::TypeFunction(f) => self.visit_stat_type_function(f),
      AstStatRef::DeclareGlobal(d) => self.visit_stat_declare_global(d),
      AstStatRef::DeclareFunction(d) => self.visit_stat_declare_function(d),
      AstStatRef::DeclareExternType(d) => self.visit_stat_declare_extern_type(d),
      AstStatRef::DeclareClass(d) => {
        LUAU_ASSERT!(fflag::DebugLuauUserDefinedClasses.get());
        self.visit_stat_class(d)
      }
      AstStatRef::Error(e) => self.visit_stat_error(e),
    }
  }

  /// cpp `visit(AstStatIf*)`：then/else 双子作用域，按控制流走向决定继承或 join。
  pub(crate) fn visit_stat_if(&mut self, i: &AstStatIf) -> ControlFlow {
    let condition = i.condition.get();
    self.visit_expr(condition);

    let then_scope = self.make_child_scope(ScopeType::Linear);
    let else_scope = self.make_child_scope(ScopeType::Linear);

    let then_cf = {
      let mut ps = PushScope::new(&mut self.scope_stack, then_scope);
      let thenbody = i.thenbody.get();
      let cf = self.visit_stat_block(thenbody);
      ps.pop();
      cf
    };

    let mut else_cf = ControlFlow::None;
    if let Some(elsebody) = i.elsebody.as_ref() {
      let mut ps = PushScope::new(&mut self.scope_stack, else_scope);
      else_cf = self.visit_stat(elsebody);
      ps.pop();
    }

    if then_cf != ControlFlow::None && else_cf == ControlFlow::None {
      // else 分支已终止：把 else_scope 快照吸回父帧（契约见收口点方法）。
      self.inherit_into_current_scope(else_scope);
    } else if then_cf == ControlFlow::None && else_cf != ControlFlow::None {
      // 只换继承方向（then 分支未终止时把 then_scope 吸回父帧）。
      self.inherit_into_current_scope(then_scope);
    } else if (then_cf | else_cf) == ControlFlow::None {
      // then/else 分支均线性落空：三 scope 互异且存活，phi 合并进栈顶。
      self.join_into_current(then_scope, else_scope);
    }

    if then_cf == else_cf {
      then_cf
    } else if matches(then_cf, ControlFlow::Returns)
      || matches(then_cf, ControlFlow::Throws)
        && (matches(else_cf, ControlFlow::Returns) || matches(else_cf, ControlFlow::Throws))
    {
      ControlFlow::Returns
    } else {
      ControlFlow::None
    }
  }

  /// cpp `visit(AstStatWhile*)`。
  pub(crate) fn visit_stat_while(&mut self, w: &AstStatWhile) -> ControlFlow {
    let while_scope = self.make_child_scope(ScopeType::Loop);

    let cf = {
      let mut ps = PushScope::new(&mut self.scope_stack, while_scope);
      let condition = w.condition.get();
      self.visit_expr(condition);
      let body = w.body.get();
      let cf = self.visit_stat_block(body);
      ps.pop();
      cf
    };

    if !matches(cf, ControlFlow::Returns) && !matches(cf, ControlFlow::Throws) {
      // 循环可零次执行，父帧与自身 join 取 phi（cpp join(currentScope(),
      // currentScope(), whileScope) 同此 p==a 形状）。
      let current = self.current_scope();
      self.join_into_current(current, while_scope);
    }

    ControlFlow::None
  }

  /// cpp `visit(AstStatRepeat*)`。
  pub(crate) fn visit_stat_repeat(&mut self, r: &AstStatRepeat) -> ControlFlow {
    let repeat_scope = self.make_child_scope(ScopeType::Loop);

    let cf = {
      let _ps = PushScope::new(&mut self.scope_stack, repeat_scope);
      // body/condition 已句柄化为 Node（parser 契约非空由类型层承载），`.get()`
      // 直出存活引用，`arena_ref` 判空 panic 门面消失。
      let body = r.body.get();
      let condition = r.condition.get();
      // repeat_scope 已被 PushScope 置为当前作用域，满足
      // visit_block_without_child_scope 对入栈时机的期望（cpp 同序）。
      let cf = self.visit_block_without_child_scope(body);
      let _ = self.visit_expr(condition);
      cf
    };

    // 弹栈后父帧恢复，把 repeat_scope 快照吸回父帧（契约见收口点方法）。
    self.inherit_into_current_scope(repeat_scope);

    if matches(cf, ControlFlow::Breaks) || matches(cf, ControlFlow::Continues) {
      ControlFlow::None
    } else {
      cf
    }
  }

  /// cpp `visit(AstStatBreak*)`。
  pub fn visit_stat_break(&mut self, _b: &AstStatBreak) -> ControlFlow {
    ControlFlow::Breaks
  }

  /// cpp `visit(AstStatContinue*)`。
  pub fn visit_stat_continue(&mut self, _c: &AstStatContinue) -> ControlFlow {
    ControlFlow::Continues
  }

  /// cpp `visit(AstStatReturn*)`。
  pub(crate) fn visit_stat_return(&mut self, r: &AstStatReturn) -> ControlFlow {
    for e in r.list.iter_nodes() {
      let _ = self.visit_expr(e);
    }

    ControlFlow::Returns
  }

  /// cpp `visit(AstStatExpr*)`：表达式语句若是 error 魔法调用则传播 Throws。
  pub(crate) fn visit_stat_expr(&mut self, e: &AstStatExpr) -> ControlFlow {
    // expr 已句柄化：`.get()` 直接给出存活只读引用，原 `arena_ref` 判空门面消失。
    let expr = e.expr.get();
    self.visit_expr(expr);

    if let AstExprRef::Call(call) = expr.as_expr_ref()
      && does_call_error(call)
    {
      ControlFlow::Throws
    } else {
      ControlFlow::None
    }
  }

  /// cpp `visit(AstStatLocal*)`：先 visit 值表达式取 defs，再逐个登记 local 的
  /// def（表字面量值直接复用对应 def）。
  pub(crate) fn visit_stat_local(&mut self, l: &AstStatLocal) -> ControlFlow {
    let values = l.values.as_slice();
    let mut defs: Vec<DefId> = Vec::with_capacity(values.len());
    for expr in l.values.iter_nodes() {
      defs.push(self.visit_expr(expr).def);
    }

    let vars = l.vars.as_slice();
    for (i, &var_ptr) in vars.iter().enumerate() {
      let local = arena_ref(var_ptr, "AstStatLocal.vars 元素");
      // annotation 是显式可空字段，alias_opt 把判空折叠进取引用（收口在
      // arena_handle 契约门面；cpp `if (local->annotation)` 同款）。
      if let Some(annotation) = alias_opt(local.annotation) {
        self.visit_type(annotation);
      }

      // We need to create a new def to intentionally avoid alias tracking, but we'd like to
      // make sure that the non-aliased defs are also marked as a subscript for refinements.
      let subscripted = i < defs.len() && contains_subscripted_definition(defs[i]);
      // 持有借用（&AstLocal）走引用臂建 symbol；local_defs 键取同址身份键。
      let symbol = Symbol::from_local_ref(local);
      let local_key = from_ref(local);
      let mut def =
        self
          .def_arena
          .get_mut()
          .fresh_cell(symbol.clone(), local.location, subscripted);

      if i < values.len() {
        let expr = arena_ref(values[i], "AstStatLocal.values 元素");
        if matches!(expr.as_expr_ref(), AstExprRef::Table(_)) {
          def = defs[i];
        }
      }

      *self.graph.local_defs.get_or_insert(local_key) = def;
      self.bind_current_scope(symbol.clone(), def);
      self.captures.get_or_insert(symbol).all_versions.push(def);
    }

    ControlFlow::None
  }
}

fn returns_or_throws(cf: ControlFlow) -> bool {
  cf.intersects(ControlFlow::Returns | ControlFlow::Throws)
}

impl DataFlowGraphBuilder {
  /// cpp `visit(AstStatFor*)`。
  pub(crate) fn visit_stat_for(&mut self, f: &AstStatFor) -> ControlFlow {
    let for_scope = self.make_child_scope(ScopeType::Loop);

    // from/to 已句柄化为非空 Node（parser 必建上下界，非空由类型层承载），
    // `.get()` 直出存活引用，arena_ref 判空 panic 门面消失；step 落可空
    // OptNode，`get()` 即 Option（cpp `if (f->step)` 同款）。
    let from = f.from.get();
    let to = f.to.get();
    self.visit_expr(from);
    self.visit_expr(to);
    if let Some(step) = f.step.get() {
      self.visit_expr(step);
    }

    let cf;
    {
      let _ps = PushScope::new(&mut self.scope_stack, for_scope);

      // var 已句柄化为 Node（push_local alloc 恒非空），`.get()` 直出引用。
      let var = f.var.get();
      // annotation 可空，alias_opt 判空折叠（收口在 arena_handle 契约门面）。
      if let Some(annotation) = alias_opt(var.annotation) {
        self.visit_type(annotation);
      }

      // symbol 经 Node 的 as_ptr 桥接（指针身份），local_defs 键取同址
      //（可变句柄到共享键型的极性弱化隐式完成）。
      let var_ptr = f.var.as_ptr();
      let symbol = Symbol::from_local(var_ptr);
      let def = self
        .def_arena
        .get_mut()
        .fresh_cell(symbol.clone(), var.location, false);
      *self.graph.local_defs.get_or_insert(var_ptr) = def;
      self.bind_current_scope(symbol.clone(), def);
      self.captures.get_or_insert(symbol).all_versions.push(def);

      cf = self.visit_stat_block(f.body.get());
    }

    if !returns_or_throws(cf) {
      // p==a 形状即 cpp join(currentScope(), currentScope(), forScope)。
      let current = self.current_scope();
      self.join_into_current(current, for_scope);
    }

    ControlFlow::None
  }

  /// cpp `visit(AstStatForIn*)`。
  pub(crate) fn visit_stat_for_in(&mut self, f: &AstStatForIn) -> ControlFlow {
    let for_scope = self.make_child_scope(ScopeType::Loop);

    let cf;
    {
      let _ps = PushScope::new(&mut self.scope_stack, for_scope);

      for &local_ptr in f.vars.as_slice() {
        let local = arena_ref(local_ptr, "AstStatForIn.vars 元素");
        // annotation 是显式可空字段，alias_opt 判空折叠（收口在 arena_handle
        // 契约门面）。
        if let Some(annotation) = alias_opt(local.annotation) {
          self.visit_type(annotation);
        }

        // 持有借用（&AstLocal）走引用臂建 symbol；local_defs 键取同址身份键。
        let symbol = Symbol::from_local_ref(local);
        let local_key = from_ref(local);
        let def = self
          .def_arena
          .get_mut()
          .fresh_cell(symbol.clone(), local.location, false);
        *self.graph.local_defs.get_or_insert(local_key) = def;
        self.bind_current_scope(symbol.clone(), def);
        self.captures.get_or_insert(symbol).all_versions.push(def);
      }

      for expr in f.values.iter_nodes() {
        self.visit_expr(expr);
      }

      // body 已句柄化为非空 Node，arena_ref 判空 panic 门面随类型折叠。
      cf = self.visit_stat_block(f.body.get());
    }

    if !returns_or_throws(cf) {
      // 同 visit_stat_for——p==a 重叠由 join 实现的快照式读写消化。
      let current = self.current_scope();
      self.join_into_current(current, for_scope);
    }

    ControlFlow::None
  }

  /// cpp `visit(AstStatAssign*)`：值先成 def，再逐个左值消费；缺值兜底新鲜 def。
  pub(crate) fn visit_stat_assign(&mut self, a: &AstStatAssign) -> ControlFlow {
    let mut defs: Vec<DefId> = Vec::with_capacity(a.values.size);

    for expr in a.values.iter_nodes() {
      defs.push(self.visit_expr(expr).def);
    }

    for (i, &var_ptr) in a.vars.as_slice().iter().enumerate() {
      let var = arena_ref(var_ptr, "AstStatAssign.vars 元素");
      let incoming_def = if i < defs.len() {
        defs[i]
      } else {
        self
          .def_arena
          .get_mut()
          .fresh_cell(Symbol::default(), var.base.location, false)
      };
      self.visit_lvalue(var, incoming_def);
    }

    ControlFlow::None
  }

  /// cpp `visit(AstStatCompoundAssign*)`。
  pub(crate) fn visit_stat_compound_assign(&mut self, c: &AstStatCompoundAssign) -> ControlFlow {
    // var/value 已句柄化：`.get()` 直供 arena 只读视图，原 `arena_ref` 判空门面消失。
    let value = c.value.get();
    let var = c.var.get();

    let _ = self.visit_expr(value);
    let _ = self.visit_expr(var);

    ControlFlow::None
  }

  /// cpp `visit(AstStatFunction*)`：先按名 typestate，再进签名作用域登记
  /// 全局/索引属性的 def，最后 visit 函数体并调整 local 的 capture 偏移。
  pub(crate) fn visit_stat_function(&mut self, f: &AstStatFunction) -> ControlFlow {
    // name/func 已句柄化为 Node（parser 契约非空由类型层承载），`.get()` 直出
    // 存活引用，`arena_ref` 判空 panic 门面消失。
    let name = f.name.get();
    let func = f.func.get();

    // local f
    // function f()
    //   if cond() then
    //     f() -- should reference only the function version and other future version, and nothing prior
    //   end
    // end
    let incoming_def =
      self
        .def_arena
        .get_mut()
        .fresh_cell(Symbol::default(), name.base.location, false);
    self.visit_lvalue(name, incoming_def);

    // DataFlowGraph.cpp:747-800: bind the signature scope's defs for
    // global/indexed properties, then adjust a local's capture version offset.
    // （name/func 已句柄化为 Node，非空由类型层兑现，原 `if !f.name.is_null()` /
    // `if !f.func.is_null()` 守卫在引用语义下恒真，删除。）

    let signature_scope = self.make_child_scope(ScopeType::Function);
    let _ps = PushScope::new(&mut self.scope_stack, signature_scope);

    let name_def = self.graph.get_def_ast_expr(from_ref(name));

    match name.as_expr_ref() {
      AstExprRef::Global(global) => {
        let symbol = Symbol::from_global(global.name);
        // signature_scope 为本帧 make_child_scope 的 PinnedStorage 活单元，
        // 写入经 bind_scope 收口点（契约见该方法）。
        self.bind_scope(signature_scope, symbol, name_def);
      }
      AstExprRef::IndexName(index_name) => {
        // 索引接收者 `expr` 已句柄化恒非空（cpp 判空为死守卫），.get() 安全
        // 借用直接下转；命中即存活只读节点。
        if let AstExprRef::Local(receiver_expr) = index_name.expr.get().as_expr_ref() {
          let receiver_def = self.lookup_symbol_location(
            Symbol::from_local(receiver_expr.local.as_ptr()),
            func.base.base.location,
          );
          // null 名由 as_str_or_empty 统一按 "" 处理
          let key = index_name.index.as_str_or_empty().to_string();

          // signature_scope 为 PinnedStorage 活单元、receiver_def 为 lookup
          // 回吐的合法 DefId；可变物化经 arena_handle::alias 契约门面，借用
          // 止于本语句。
          alias(signature_scope)
            .props
            .get_or_insert(receiver_def)
            .insert(key, name_def);
        }
      }
      _ => {}
    }

    // visitFunction(f->func, NotNull{signatureScope});
    let _ = self.visit_function(func, signature_scope);

    if let AstExprRef::Local(local) = name.as_expr_ref() {
      let capture = self
        .captures
        .get_or_insert(Symbol::from_local(local.local.as_ptr()));
      LUAU_ASSERT!(!capture.all_versions.is_empty());
      capture.version_offset = capture.all_versions.len() - 1;
    }

    ControlFlow::None
  }

  /// cpp `visit(AstStatLocalFunction*)`。
  pub(crate) fn visit_stat_local_function(&mut self, l: &AstStatLocalFunction) -> ControlFlow {
    // name/func 已句柄化为 Node（类型层非空 + arena 存活契约），`as_ptr` 直供
    // 指针身份键，`arena_ref` 判空 panic 门面随非空类型消失；local_defs 键的
    // 极性弱化（可变句柄→共享键型）隐式完成。
    let name_ptr = l.name.as_ptr();
    let symbol = Symbol::from_local(name_ptr);
    let def = self
      .def_arena
      .get_mut()
      .fresh_cell(symbol.clone(), l.base.base.location, false);
    *self.graph.local_defs.get_or_insert(name_ptr) = def;
    // 栈顶 scope 的 bindings 写经 bind_current_scope 收口点（其契约覆盖
    // build() 根 push + PinnedStorage 地址稳定前提）。
    self.bind_current_scope(symbol.clone(), def);
    self.captures.get_or_insert(symbol).all_versions.push(def);

    // 保持既有实现：上转基类经分派入口 visit_expr 走函数节点（含缓存回写
    // astDefs 的现有行为；repr(C) 首字段使 `&func.base` 即 cpp
    // `static_cast<AstExpr*>(l->func)` 的同址结果）。
    let func = l.func.get();
    self.visit_expr(&func.base);
    ControlFlow::None
  }

  /// 对照 cpp `DataFlowGraph::visit(AstStatTypeAlias*)`（DataFlowGraph.cpp:805）。
  pub(crate) fn visit_stat_type_alias(&mut self, t: &AstStatTypeAlias) -> ControlFlow {
    let unreachable = self.make_child_scope(ScopeType::Linear);
    let mut ps = PushScope::new(&mut self.scope_stack, unreachable);

    self.visit_generics(t.generics);
    self.visit_generic_packs(t.generic_packs);
    // type_ptr 槽已句柄化（别名右值 parser 必建），get() 直取。
    self.visit_type(t.type_ptr.get());

    ps.pop();

    ControlFlow::None
  }

  /// 对照 cpp `DataFlowGraph::visit(AstStatTypeFunction*)`（DataFlowGraph.cpp:817）。
  pub fn visit_stat_type_function(&mut self, f: &AstStatTypeFunction) -> ControlFlow {
    let unreachable = self.make_child_scope(ScopeType::Linear);
    let _ps = PushScope::new(&mut self.scope_stack, unreachable);

    // parser 对 type function 语句必生成非空函数体（cpp `visitExpr(f->body)`
    // 的直接解引用同款前提）。
    let body = arena_ref(f.body, "AstStatTypeFunction.body");
    self.visit_expr_function(body);

    ControlFlow::None
  }

  /// cpp `visit(AstStatDeclareGlobal*)`。
  pub(crate) fn visit_stat_declare_global(&mut self, d: &AstStatDeclareGlobal) -> ControlFlow {
    let symbol = Symbol::from_global(d.name);
    let def = self
      .def_arena
      .get_mut()
      .fresh_cell(symbol.clone(), d.name_location, false);
    // declared_defs 以派生节点的 AstStat 子对象地址作身份键：repr(C) 首字段
    // 使其与本节点同址，`from_ref` + `cast` 取同址等价原指针形态。
    let d_stat_ptr = from_ref(d).cast::<AstStat>();
    *self.graph.declared_defs.get_or_insert(d_stat_ptr) = def;
    self.bind_current_scope(symbol.clone(), def);
    self.captures.get_or_insert(symbol).all_versions.push(def);

    // 对应 cpp DataFlowGraph.cpp:833 `visitType(d->type)`——declare 语句的
    // 类型标注是 parser 必写节点。
    let ty = arena_ref(d.type_, "AstStatDeclareGlobal.type_");
    self.visit_type(ty);

    ControlFlow::None
  }

  /// cpp `visit(AstStatDeclareFunction*)`。
  pub(crate) fn visit_stat_declare_function(&mut self, d: &AstStatDeclareFunction) -> ControlFlow {
    let symbol = Symbol::from_global(d.name);
    let def = self
      .def_arena
      .get_mut()
      .fresh_cell(symbol.clone(), d.name_location, false);
    let d_stat_ptr = from_ref(d).cast::<AstStat>();
    *self.graph.declared_defs.get_or_insert(d_stat_ptr) = def;
    self.bind_current_scope(symbol.clone(), def);
    self.captures.get_or_insert(symbol).all_versions.push(def);

    let unreachable = self.make_child_scope(ScopeType::Linear);
    let _ps = PushScope::new(&mut self.scope_stack, unreachable);

    self.visit_generics(d.generics);
    self.visit_generic_packs(d.generic_packs);
    self.visit_type_list(d.params);
    // ret_types 槽已句柄化（declare 文法对缺省返回类型现场补建显式空 pack），get() 直取。
    self.visit_type_pack(d.ret_types.get());

    ControlFlow::None
  }

  /// cpp `visit(AstStatDeclareExternType*)`。
  pub(crate) fn visit_stat_declare_extern_type(
    &mut self,
    d: &AstStatDeclareExternType,
  ) -> ControlFlow {
    // This declaration does not "introduce" any bindings in value namespace,
    // so there's no symbolic value to begin with. We'll traverse the properties
    // because their type annotations may depend on something in the value namespace.
    let unreachable = self.make_child_scope(ScopeType::Linear);
    let _ps = PushScope::new(&mut self.scope_stack, unreachable);

    for &prop in d.props.as_slice() {
      let ty = arena_ref(prop.ty, "AstDeclaredExternTypeProperty.ty");
      self.visit_type(ty);
    }

    ControlFlow::None
  }

  /// 对照 cpp `DataFlowGraph::visit(AstStatClass*)`（DataFlowGraph.cpp:871 起）。
  pub fn visit_stat_class(&mut self, d: &AstStatClass) -> ControlFlow {
    LUAU_ASSERT!(fflag::DebugLuauUserDefinedClasses.get());

    // parser 为 class 语句必绑定非空 AstLocal（cpp `d->name->name` 的链式
    // 解引用同款前提）；持有借用走引用臂建 symbol，local_defs 键取同址身份键。
    let name = arena_ref(d.name, "AstStatClass.name");
    // cpp:874 freshCell 以 AstLocal*（local symbol）建 def；但 cpp:876-877
    // bindings/captures 键是 `d->name->name`（AstName→global symbol），
    // 使用处 `Bar` 解析为 AstExprGlobal 才查得同类 def。
    let symbol = Symbol::from_local_ref(name);
    let name_symbol = Symbol::from_global(name.name);
    let def = self
      .def_arena
      .get_mut()
      .fresh_cell(symbol, name.location, false);

    *self.graph.local_defs.get_or_insert(from_ref(name)) = def;
    self.bind_current_scope(name_symbol.clone(), def);
    self
      .captures
      .get_or_insert(name_symbol)
      .all_versions
      .push(def);

    // 对照 C++:880-881 `if (d->super) visitExpr(d->super);`：
    // super 表达式须先于成员登记，否则其 refinement key 缺失会触发断言。
    // super_ 显式可空，alias_opt 判空折叠（收口在 arena_handle 契约门面）。
    if let Some(super_expr) = alias_opt(d.super_) {
      self.visit_expr(super_expr);
    }

    for member in d.members.as_slice() {
      if let Some(prop) = member.get_if::<AstClassProperty>() {
        // prop.ty 可空（cpp `if (prop.ty)` 同款判空），alias_opt 收口。
        if let Some(ty) = alias_opt(prop.ty) {
          self.visit_type(ty);
        }
      } else if let Some(method) = member.get_if::<AstClassMethod>() {
        let function = arena_ref(method.function, "AstClassMethod.function");
        self.visit_expr_function(function);
      }
    }

    ControlFlow::None
  }

  /// 对照 cpp `DataFlowGraph::visit(AstStatError*)`（DataFlowGraph.cpp:905-917）。
  pub(crate) fn visit_stat_error(&mut self, error: &AstStatError) -> ControlFlow {
    let unreachable = self.make_child_scope(ScopeType::Linear);
    let mut ps = PushScope::new(&mut self.scope_stack, unreachable);

    for s in error.statements.iter_nodes() {
      self.visit_stat(s);
    }

    for e in error.expressions.iter_nodes() {
      self.visit_expr(e);
    }

    ps.pop();

    ControlFlow::None
  }
}
