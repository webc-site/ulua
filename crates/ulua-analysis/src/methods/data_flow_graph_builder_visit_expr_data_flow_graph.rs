use alloc::string::String;
use core::ptr::null;

use ulua_ast::{
  enums::ast_expr_ref::AstExprRef,
  functions::is_l_value::is_l_value,
  records::{
    ast_expr::AstExpr,
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_expr_call::AstExprCall,
    ast_expr_error::AstExprError,
    ast_expr_function::AstExprFunction,
    ast_expr_global::AstExprGlobal,
    ast_expr_group::AstExprGroup,
    ast_expr_if_else::AstExprIfElse,
    ast_expr_index_expr::AstExprIndexExpr,
    ast_expr_index_name::AstExprIndexName,
    ast_expr_instantiate::AstExprInstantiate,
    ast_expr_interp_string::AstExprInterpString,
    ast_expr_local::AstExprLocal,
    ast_expr_table::AstExprTable,
    ast_expr_type_assertion::AstExprTypeAssertion,
    ast_expr_unary::AstExprUnary,
    ast_local::AstLocal,
    ast_type_or_pack::AstTypeOrPack,
  },
};
use ulua_common::{
  fflag,
  macros::luau_assert::{LUAU_ASSERT, LUAU_UNREACHABLE},
};

use crate::{
  enums::scope_type::ScopeType,
  functions::{
    arena_ref::arena_ref, contains_subscripted_definition::contains_subscripted_definition,
    should_typestate_for_first_argument::should_typestate_for_first_argument,
  },
  records::{
    data_flow_graph_builder::DataFlowGraphBuilder, data_flow_result::DataFlowResult,
    dfg_scope::DfgScope, push_scope::PushScope, symbol::Symbol,
  },
};

impl DataFlowGraphBuilder {
  /// cpp `DataFlowGraphBuilder::visitExpr(AstExpr*)` 的分派入口。
  pub fn visit_expr(&mut self, expr: &AstExpr) -> DataFlowResult {
    // cpp 全程以 `AstExpr*` 作 astDefs/astRefinementKeys 的身份键；从共享引用
    // 取同一地址仅作查表/回写键值，从不据此写入。
    let expr_ptr: *const AstExpr = expr as *const AstExpr;

    // cpp DataFlowGraph.cpp:920-925：子表达式可能被访问两次，缓存命中直接回吐
    // 既有 def/key；两处 find 只按指针值查表，不解引用，纯安全代码。
    if let Some(def) = self.graph.ast_defs.find(&expr_ptr) {
      let key = self.graph.ast_refinement_keys.find(&expr_ptr);
      return DataFlowResult {
        def: *def,
        parent: if let Some(k) = key { *k } else { null() },
      };
    }

    let result = match expr.as_expr_ref() {
      AstExprRef::Group(group) => self.visit_expr_group(group),
      AstExprRef::ConstantNil(_)
      | AstExprRef::ConstantBool(_)
      | AstExprRef::ConstantNumber(_)
      | AstExprRef::ConstantInteger(_)
      | AstExprRef::ConstantString(_)
      | AstExprRef::Varargs(_) => DataFlowResult {
        def: self
          .def_arena
          .get_mut()
          .fresh_cell(Symbol::default(), expr.base.location, false),
        parent: null(),
      },
      AstExprRef::Local(local) => self.visit_expr_local(local),
      AstExprRef::Global(global) => self.visit_expr_global(global),
      AstExprRef::Call(call) => self.visit_expr_call(call),
      AstExprRef::IndexName(index_name) => self.visit_expr_index_name(index_name),
      AstExprRef::IndexExpr(index_expr) => self.visit_expr_index_expr(index_expr),
      AstExprRef::Function(function) => self.visit_expr_function(function),
      AstExprRef::Table(table) => self.visit_expr_table(table),
      AstExprRef::Unary(unary) => self.visit_expr_unary(unary),
      AstExprRef::Binary(binary) => self.visit_expr_binary(binary),
      AstExprRef::TypeAssertion(type_assertion) => self.visit_expr_type_assertion(type_assertion),
      AstExprRef::IfElse(if_else) => self.visit_expr_if_else(if_else),
      AstExprRef::InterpString(interp_string) => self.visit_expr_interp_string(interp_string),
      AstExprRef::Instantiate(instantiate) => self.visit_expr_instantiate(instantiate),
      AstExprRef::Error(error) => self.visit_expr_error(error),
    };

    // 缓存回写全安全：DenseHashMap 以指针值为键查写，不解引用键。
    *self.graph.ast_defs.get_or_insert(expr_ptr) = result.def;
    if !result.parent.is_null() {
      *self.graph.ast_refinement_keys.get_or_insert(expr_ptr) = result.parent;
    }

    result
  }

  /// cpp `visitExpr(AstExprGroup*)`：剥去圆括号直接 visit 内层表达式。
  pub fn visit_expr_group(&mut self, group: &AstExprGroup) -> DataFlowResult {
    // expr 已句柄化恒非空；arena_ref 为既有指针门面，经 as_ptr 桥接（判空 panic 分支类型端不可达）。
    let inner = arena_ref(group.expr.as_ptr(), "AstExprGroup.expr");
    self.visit_expr(inner)
  }

  /// cpp `visitExpr(AstExprLocal*)`：查 local 符号的当前 def 并造 leaf refinement key。
  pub fn visit_expr_local(&mut self, l: &AstExprLocal) -> DataFlowResult {
    // cpp 在 `l->local` 处直接解引用、不容空：local 槽已句柄化恒非空；
    // arena_ref 为既有指针门面，经 as_ptr 桥接。
    let local = arena_ref(l.local.as_ptr(), "AstExprLocal.local");
    let local_ptr: *mut AstLocal = (local as *const AstLocal).cast_mut();
    // def_arena/key_arena/graph 全走安全句柄接口，借用各止于本语句。
    let def = self.lookup_symbol_location(Symbol::from_local(local_ptr), local.location);
    let key = self.key_arena.get_mut().leaf(def);
    *self.graph.def_to_symbol.get_or_insert(def) = Symbol::from_local(local_ptr);

    DataFlowResult { def, parent: key }
  }

  /// cpp `visitExpr(AstExprGlobal*)`：查全局符号的当前 def 并造 leaf refinement key。
  pub fn visit_expr_global(&mut self, g: &AstExprGlobal) -> DataFlowResult {
    let def = self.lookup_symbol_location(Symbol::from_global(g.name), g.base.base.location);
    *self.graph.def_to_symbol.get_or_insert(def) = Symbol::from_global(g.name);
    let key = self.key_arena.get_mut().leaf(def);

    DataFlowResult { def, parent: key }
  }

  /// cpp `visitExpr(AstExprCall*)`：visit func/类型实参/实参表，命中 typestate
  /// 魔法函数时对首实参做左值式 typestate，最终回吐 subscripted 的新鲜 def。
  pub fn visit_expr_call(&mut self, call: &AstExprCall) -> DataFlowResult {
    let func = arena_ref(call.func, "AstExprCall.func");
    self.visit_expr(func);

    let type_arguments = call.type_arguments.as_slice();
    if fflag::LuauVisitCallTypeArgsInDfg.get() && fflag::LuauExplicitTypeInstantiationSupport.get()
    {
      for type_or_pack in type_arguments {
        // 二选一注解槽位：变体载荷即 arena 存活节点，判序与 cpp
        // `if (typeOrPack.type)` 一致；Error 形态（cpp 双侧皆 null）在 parser
        // 产物中不可达，仅保留 cpp else 臂的 LUAU_ASSERT 上报。
        match *type_or_pack {
          AstTypeOrPack::Type(t) => self.visit_type(t),
          AstTypeOrPack::Pack(pack) => self.visit_type_pack(pack),
          AstTypeOrPack::Error => LUAU_ASSERT!(false),
        }
      }
    }

    for arg in call.args.iter_nodes() {
      self.visit_expr(arg);
    }

    let args = call.args.as_slice();
    // cpp: shouldTypestateForFirstArgument(c) && c->args.size > 1 && isLValue(*c->args.begin())
    if should_typestate_for_first_argument(call) && args.len() > 1 {
      // SAFETY: `args[0]` 是 AstArray data/size 成对写入的首元素，命中即 arena
      // 存活只读节点；判空折叠进取引用（null 时整段跳过，cpp `isLValue(nullptr)
      // == false` 同效），一次取引用供类索引分派、缓存回写与 visit_lvalue 复用。
      if let Some(first_arg) = unsafe { args[0].as_ref() }
        && is_l_value(first_arg)
      {
        let result = match first_arg.as_expr_ref() {
          AstExprRef::Local(l) => self.visit_expr_local(l),
          AstExprRef::Global(g) => self.visit_expr_global(g),
          AstExprRef::IndexName(i) => self.visit_expr_index_name(i),
          AstExprRef::IndexExpr(i) => self.visit_expr_index_expr(i),
          // SAFETY: 整段由 `is_l_value(first_arg)` 守卫，动态类型必为上述四类
          // 之一（cpp 以同一前提 LUAU_UNREACHABLE 兜底）。
          _ => LUAU_UNREACHABLE!(),
        };

        let child: *mut DfgScope = self.make_child_scope(ScopeType::Linear);
        self.scope_stack.push(child);

        // cpp 全程以 `AstExpr*` 作 astDefs/astRefinementKeys 的身份键；从共享
        // 引用取同一地址仅作查表/回写键值，从不据此写入。
        let first_arg_ptr: *const AstExpr = first_arg as *const AstExpr;
        *self.graph.ast_defs.get_or_insert(first_arg_ptr) = result.def;
        if !result.parent.is_null() {
          *self.graph.ast_refinement_keys.get_or_insert(first_arg_ptr) = result.parent;
        }

        self.visit_lvalue(first_arg, result.def);
      }
    }

    let fresh_def =
      self
        .def_arena
        .get_mut()
        .fresh_cell(Symbol::default(), call.base.base.location, true);
    DataFlowResult {
      def: fresh_def,
      parent: null(),
    }
  }

  /// cpp `visitExpr(AstExprIndexName*)`：`a.b` 形态按名字查父 def 的 prop。
  pub fn visit_expr_index_name(&mut self, i: &AstExprIndexName) -> DataFlowResult {
    // expr 已句柄化恒非空；arena_ref 为既有指针门面，经 as_ptr 桥接（判空 panic 分支类型端不可达）。
    let parent_expr = arena_ref(i.expr.as_ptr(), "AstExprIndexName.expr");
    let parent = self.visit_expr(parent_expr);
    let index = String::from(i.index.as_str_or_empty());
    // SAFETY: parent.def 由 visit_expr 产图不变量给出为存活 Def arena 的合法
    // DefId（非空）；parent.parent 是允许为 null 的前缀 refinement key，
    // lookup 的契约正要求此形态——与 cpp `lookup(parentDef, index, loc)` 同前提。
    let def =
      unsafe { self.lookup_def_id_string_location(parent.def, &index, i.base.base.location) };
    let key = self.key_arena.get_mut().node(parent.parent, def, &index);

    DataFlowResult { def, parent: key }
  }

  /// cpp `visitExpr(AstExprIndexExpr*)`：`a[b]` 形态，b 为字符串字面量时按名查
  /// prop，否则视作下标访问造 subscripted def。
  pub fn visit_expr_index_expr(&mut self, i: &AstExprIndexExpr) -> DataFlowResult {
    // expr/index 已句柄化恒非空；arena_ref 为既有指针门面，经 as_ptr 桥接（判空 panic 分支类型端不可达）。
    let parent_expr = arena_ref(i.expr.as_ptr(), "AstExprIndexExpr.expr");
    let index_expr = arena_ref(i.index.as_ptr(), "AstExprIndexExpr.index");
    let parent = self.visit_expr(parent_expr);
    self.visit_expr(index_expr);

    if let AstExprRef::ConstantString(string) = index_expr.as_expr_ref() {
      let index = String::from_utf8_lossy(string.value.as_bytes()).into_owned();
      // SAFETY: 同 visit_expr_index_name——parent.def/parent.parent 满足 lookup
      // 的合法 DefId/可空前缀键契约。
      let def =
        unsafe { self.lookup_def_id_string_location(parent.def, &index, i.base.base.location) };
      let key = self.key_arena.get_mut().node(parent.parent, def, &index);
      return DataFlowResult { def, parent: key };
    }

    let def = self
      .def_arena
      .get_mut()
      .fresh_cell(Symbol::default(), i.base.base.location, true);
    DataFlowResult {
      def,
      parent: null(),
    }
  }

  /// cpp `visitExpr(AstExprFunction*)`：建 Function 签名作用域后转 visit_function。
  pub fn visit_expr_function(&mut self, f: &AstExprFunction) -> DataFlowResult {
    let signature_scope = self.make_child_scope(ScopeType::Function);
    let _ps = PushScope::new(&mut self.scope_stack, signature_scope);

    let f_ptr: *mut AstExprFunction = (f as *const AstExprFunction).cast_mut();
    // SAFETY: `f` 是调用方受检的存活函数节点，取同址裸指针即原入参本身；
    // signature_scope 是上一步 make_child_scope 从 PinnedStorage 划出的单元
    // （非空、地址稳定，存活至 builder 析构），正对 visit_function 的契约。
    unsafe { self.visit_function(f_ptr, signature_scope) }
  }

  /// cpp `visitExpr(AstExprTable*)`：为表字面量造 cell def，并把字符串键的项
  /// 登记进当前作用域 props。
  pub fn visit_expr_table(&mut self, t: &AstExprTable) -> DataFlowResult {
    let table_cell =
      self
        .def_arena
        .get_mut()
        .fresh_cell(Symbol::default(), t.base.base.location, false);
    let scope = self.current_scope();
    // SAFETY: scope 为 current_scope() 所得栈顶活 scope（PinnedStorage 地址
    // 稳定、活至 builder 析构）；props 写在 &mut self 独占路径上，借用止于
    // 本语句。C++: currentScope()->props[tableCell] = {};
    unsafe { *(*scope).props.get_or_insert(table_cell) = Default::default() };

    for item in t.items.as_slice() {
      let value = arena_ref(item.value, "AstExprTable 项的 value");
      let result = self.visit_expr(value);
      // SAFETY: item.key 可为 null（列表式项无键），as_ref 把判空折叠进取引用；
      // 命中即 parser 写入的活键表达式节点；cpp `if (item.key)` 同款。
      if let Some(key_expr) = unsafe { item.key.as_ref() } {
        self.visit_expr(key_expr);
        if let AstExprRef::ConstantString(string) = key_expr.as_expr_ref() {
          let key_str = String::from_utf8_lossy(string.value.as_bytes()).into_owned();
          // SAFETY: scope 仍是本帧登记的栈顶活 scope；与循环内各递归的临时
          // 借用不同时存在（每次借用止于本语句），C++:
          // currentScope()->props[tableCell][string->value.data] = result.def;
          unsafe {
            (*scope)
              .props
              .get_or_insert(table_cell)
              .insert(key_str, result.def)
          };
        }
      }
    }

    // C++: return {tableCell, nullptr};
    DataFlowResult {
      def: table_cell,
      parent: null(),
    }
  }

  /// cpp `visitExpr(AstExprUnary*)`。
  pub fn visit_expr_unary(&mut self, u: &AstExprUnary) -> DataFlowResult {
    // expr 已句柄化恒非空；arena_ref 为既有指针门面，经 as_ptr 桥接（判空 panic 分支类型端不可达）。
    let inner = arena_ref(u.expr.as_ptr(), "AstExprUnary.expr");
    self.visit_expr(inner);

    let def = self
      .def_arena
      .get_mut()
      .fresh_cell(Symbol::default(), u.base.base.location, false);
    DataFlowResult {
      def,
      parent: null(),
    }
  }

  /// cpp `visitExpr(AstExprBinary*)`。
  pub fn visit_expr_binary(&mut self, b: &AstExprBinary) -> DataFlowResult {
    // left/right 已句柄化恒非空；arena_ref 为既有指针门面，经 as_ptr 桥接（判空 panic 分支类型端不可达）。
    let left_expr = arena_ref(b.left.as_ptr(), "AstExprBinary.left");
    let right_expr = arena_ref(b.right.as_ptr(), "AstExprBinary.right");
    let left = self.visit_expr(left_expr);
    let right = self.visit_expr(right_expr);

    let subscripted = (b.op == AstExprBinaryOp::And || b.op == AstExprBinaryOp::Or)
      && (contains_subscripted_definition(left.def) || contains_subscripted_definition(right.def));

    let def =
      self
        .def_arena
        .get_mut()
        .fresh_cell(Symbol::default(), b.base.base.location, subscripted);

    DataFlowResult {
      def,
      parent: null(),
    }
  }

  /// cpp `visitExpr(AstExprTypeAssertion*)`：`(e :: T)` 先 visit 表达式再 visit 标注。
  pub fn visit_expr_type_assertion(&mut self, t: &AstExprTypeAssertion) -> DataFlowResult {
    // expr/annotation 已句柄化恒非空；arena_ref 为既有指针门面，经 as_ptr 桥接（判空 panic 分支类型端不可达）。
    let inner = arena_ref(t.expr.as_ptr(), "AstExprTypeAssertion.expr");
    let annotation = arena_ref(t.annotation.as_ptr(), "AstExprTypeAssertion.annotation");
    let def = self.visit_expr(inner);
    self.visit_type(annotation);
    def
  }

  /// cpp `visitExpr(AstExprIfElse*)`：三元/条件表达式，三个分支全部 visit。
  pub fn visit_expr_if_else(&mut self, i: &AstExprIfElse) -> DataFlowResult {
    // 三子节点已句柄化恒非空；arena_ref 为既有指针门面，经 as_ptr 桥接。
    let condition = arena_ref(i.condition.as_ptr(), "AstExprIfElse.condition");
    let true_expr = arena_ref(i.true_expr.as_ptr(), "AstExprIfElse.true_expr");
    let false_expr = arena_ref(i.false_expr.as_ptr(), "AstExprIfElse.false_expr");
    self.visit_expr(condition);
    self.visit_expr(true_expr);
    self.visit_expr(false_expr);

    let def = self
      .def_arena
      .get_mut()
      .fresh_cell(Symbol::default(), i.base.base.location, false);

    DataFlowResult {
      def,
      parent: null(),
    }
  }

  /// cpp `visitExpr(AstExprInterpString*)`：逐插值段 visit 后造字面量 def。
  pub fn visit_expr_interp_string(&mut self, i: &AstExprInterpString) -> DataFlowResult {
    for expr in i.expressions.iter_nodes() {
      self.visit_expr(expr);
    }

    let def = self
      .def_arena
      .get_mut()
      .fresh_cell(Symbol::default(), i.base.base.location, false);
    DataFlowResult {
      def,
      parent: null(),
    }
  }

  /// cpp `visitExpr(AstExprInstantiate*)`：`f<T>(...)` 的显式类型实参先 visit，
  /// 再回到被实例化表达式。
  pub fn visit_expr_instantiate(&mut self, i: &AstExprInstantiate) -> DataFlowResult {
    if fflag::LuauExplicitTypeInstantiationSupport.get() {
      for type_or_pack in i.type_arguments.as_slice() {
        // 同 visit_expr_call——二选一注解槽位按变体分发。
        match *type_or_pack {
          AstTypeOrPack::Type(t) => self.visit_type(t),
          AstTypeOrPack::Pack(pack) => self.visit_type_pack(pack),
          AstTypeOrPack::Error => LUAU_ASSERT!(false),
        }
      }
    }

    let inner = arena_ref(i.expr, "AstExprInstantiate.expr");
    self.visit_expr(inner)
  }

  /// cpp `visitExpr(AstExprError*)`：错误恢复节点在未可达作用域里 visit 各段。
  pub fn visit_expr_error(&mut self, error: &AstExprError) -> DataFlowResult {
    {
      let unreachable: *mut DfgScope = self.make_child_scope(ScopeType::Linear);
      let _ps = PushScope::new(&mut self.scope_stack, unreachable);

      for expr in error.expressions.iter_nodes() {
        self.visit_expr(expr);
      }
    }

    let def =
      self
        .def_arena
        .get_mut()
        .fresh_cell(Symbol::default(), error.base.base.location, false);
    DataFlowResult {
      def,
      parent: null(),
    }
  }
}
