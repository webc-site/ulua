//! `type_attacher` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use core::{cmp::Ordering, ptr::null_mut};

use ulua_ast::records::{
  allocator::Allocator,
  ast_array::{AstArray, AstArrayBuilder},
  ast_expr_function::AstExprFunction,
  ast_expr_local::AstExprLocal,
  ast_local::AstLocal,
  ast_stat_for::AstStatFor,
  ast_stat_for_in::AstStatForIn,
  ast_stat_local::AstStatLocal,
  ast_type::AstType,
  ast_type_list::AstTypeList,
  ast_type_pack::AstTypePack,
  ast_type_pack_explicit::AstTypePackExplicit,
  location::Location,
  node_handle::OptNode,
};

use crate::{
  functions::flatten_type_pack::flatten_type_pack_id,
  records::{
    arena_handle::{alias, alias_ref},
    module::Module,
    symbol::Symbol,
    type_attacher::TypeAttacher,
    type_rehydration_options::TypeRehydrationOptions,
    type_rehydration_visitor::TypeRehydrationVisitor,
  },
  type_aliases::{
    scope_ptr_type::ScopePtr, synthetic_names::SyntheticNames, type_id::TypeId,
    type_pack_id::TypePackId,
  },
};

impl TypeAttacher {
  #[inline]
  fn rehydration_visitor(&mut self) -> TypeRehydrationVisitor {
    // 注：visitor 记录字段 `synthetic_names: *mut SyntheticNames` 属 arena 裸指针
    // 字段既有约定（读写收口于 `synthetic_names_mut`），此处借出本字段地址喂给
    // 该字段（本批次不改字段类型）。
    TypeRehydrationVisitor::type_rehydration_visitor_type_rehydration_visitor(
      self.allocator,
      &mut self.synthetic_names as *mut SyntheticNames,
      &TypeRehydrationOptions::default(),
    )
  }

  pub fn get_scope(&mut self, loc: &Location) -> ScopePtr {
    let module = alias_ref(self.module);
    module
      .scopes
      .iter()
      .filter(|(s_loc, _)| s_loc.encloses(loc))
      // 三态比较子按行为原样保留：`encloses` 自反，同界域并列时原式给出
      // Greater，改写为 bool-cmp 后并列变 Equal，min_by 的替换条件随之改变，
      // get_scope 会选中重复界域的另一个 scope（typeguard 推断回归实证）。
      // 该闭包属 review.md §3「惯用化」禁区：不得改写为 bool 比较或 then_with_compare 形态。
      .min_by(|(loc_a, _), (loc_b, _)| {
        if loc_a.encloses(loc_b) {
          Ordering::Greater
        } else if loc_b.encloses(loc_a) {
          Ordering::Less
        } else {
          Ordering::Equal
        }
      })
      .map(|(_, scope)| scope.clone())
      .expect("module scopes 至少包含一个全局 scope 覆盖任何位置")
  }

  /// §2 清扫批（W3）判定：C 型可免折返消除——cpp `rehydrateAnnotation(TypeId type, ...)`
  /// （TypeAttach.cpp:667）形参即非空 `TypeId`，缺席分支（lookup 未命中）在 cpp 与本文件
  /// `visit_local` 都由调用方守卫；原 `Option<TypeId>` 入参是移植层自加的可空往返，唯一
  /// 消费方（`visit_local` 命中支）恒传 `Some`，故入参收口为 `TypeId`。返回裸指针的落点
  /// 字段布局约定不变（见下方注）。
  pub fn type_ast(&mut self, ty: TypeId) -> *mut AstType {
    // C++ `return Luau::visit(TypeRehydrationVisitor(allocator, &synthetic_names), (*type)->ty);`
    // 注：返回的裸指针直存 ulua-ast 结点 `*mut AstType` 字段（读取侧经
    // optional_node::node_opt 折回 Option），属既有落点字段布局约定（本批次不改字段类型）。
    self.rehydration_visitor().visit_type(ty)
  }

  pub fn type_ast_pack(&mut self, r#type: TypePackId) -> AstArray<*mut AstType> {
    let (v, _tail) = flatten_type_pack_id(r#type);

    // 槽位申请与逐槽写入收口在 `AstArrayBuilder`（容量 v.len()、每元素恰一槽，
    // 定形 size = v.len()）。
    let mut slots = AstArrayBuilder::new(alias(self.allocator), v.len());

    for &item in &v {
      // C++ `result.data[i] = Luau::visit(TypeRehydrationVisitor(allocator, &syntheticNames), v[i]->ty);`
      // `visit_type` 已降 safe（变体读取收口于 `type_variant_of`）；`item` 是
      // flatten 出的存活 TypeId，满足其 arena 契约。
      slots.push(self.rehydration_visitor().visit_type(item));
    }

    slots.finish()
  }

  pub fn type_attacher_type_attacher(checker: *mut Module, alloc: *mut Allocator) -> Self {
    Self {
      module: checker,
      allocator: alloc,
      // C++ default-constructs `SyntheticNames synthetic_names;`; the Rust
      // DenseHashMap uses a null-pointer empty-key sentinel.
      // 既有约定（review.md §2）：空指针即 synthetic name 裸指针身份键的缺省哨兵，
      // 与 methods/constraint_graph.rs `TypePackIds::new` 同族，不改 map 键类型。
      synthetic_names: SyntheticNames::new(null_mut()),
    }
  }

  /// # Safety
  /// 调用方须保证 `local` 非空、对齐，指向 attach 期间存活、地址稳定的 `AstLocal`；本函数读 `annotation`/
  /// `location` 并在命中时**写回** `(*local).annotation`，故须对该 arena 节点持有独占可变访问、无并存借用。
  /// cpp `Analysis/src/TypeAttach.cpp:595`（`bool TypeAttacher::visitLocal(AstLocal*)`）。单线程。
  pub(crate) unsafe fn visit_local(&mut self, local: *mut AstLocal) -> bool {
    // C++ `AstType* annotation = local->annotation;`
    // C++ `if (annotation == nullptr)`
    if alias_ref(local).annotation.is_null() {
      // C++ `if (auto scope = getScope(local->location))` — the Rust
      // `get_scope` always resolves an enclosing scope (the global scope
      // encloses any location), so it is unconditionally usable here.
      let location = alias_ref(local).location;
      let scope = self.get_scope(&location);
      // C++ `if (auto result = scope->lookup(local))` — the implicit
      // `Symbol(AstLocal*)` conversion then `lookup(Symbol)`.
      if let Some(type_id) = scope.lookup_symbol(Symbol::from_local(local)) {
        alias(local).annotation = self.type_ast(type_id);
      }
    }
    true
  }

  /// `AstVisitor::visit_stat_local` 的实现桥接。`al` 由 dispatch 处
  /// `from_mut(node)`（见 `attach_type_data.rs`）取得，即当前遍历节点的
  /// 可变借用再转裸指针，故非空、对齐且在整个 attach 期间指向 `SourceModule`
  /// AST arena 保活的 `AstStatLocal`。
  pub(crate) fn visit_ast_stat_local(&mut self, al: *mut AstStatLocal) -> bool {
    let al_ref = alias_ref(al);

    al_ref.vars.iter().for_each(|&var| {
      // Safety: `var` 取自 `al_ref.vars`，均指同一 AST arena 保活、地址稳定的
      // `AstLocal`，满足被调 `unsafe fn visit_local` 的入参存活契约。
      unsafe { self.visit_local(var) };
    });

    true
  }

  /// `AstVisitor::visit_expr_local` 桥接。`al` 同上，为 dispatch `from_mut(node)`
  /// 传入的、遍历期存活的 `AstExprLocal` 裸指针。
  pub(crate) fn visit_ast_expr_local(&mut self, al: *mut AstExprLocal) -> bool {
    let al_ref = alias_ref(al);
    // Safety: `al_ref.local` 已句柄化恒非空（SourceModule arena 保活、地址稳定），
    // `visit_local` 入参契约经 as_ptr 桥接。
    unsafe { self.visit_local(al_ref.local.as_ptr()) }
  }

  /// `AstVisitor::visit_stat_for` 桥接。`stat` 为 dispatch `from_mut(node)` 传入、
  /// 遍历期存活的 `AstStatFor` 裸指针。
  pub(crate) fn visit_ast_stat_for(&mut self, stat: *mut AstStatFor) -> bool {
    // Safety: `stat` 源自 dispatch `from_mut(node)`，指向遍历期存活的 `AstStatFor`；
    // var 已句柄化为 Node（非空由类型层承载，arena 保活、地址稳定），as_ptr
    // 桥交仍以指针形态消费的 `visit_local`。
    unsafe { self.visit_local((*stat).var.as_ptr()) };
    true
  }

  /// `AstVisitor::visit_stat_for_in` 桥接。`stat` 为 dispatch `from_mut(node)` 传入、
  /// 遍历期存活的 `AstStatForIn` 裸指针。
  pub(crate) fn visit_ast_stat_for_in(&mut self, stat: *mut AstStatForIn) -> bool {
    let stat_ref = alias_ref(stat);
    stat_ref.vars.iter().for_each(|&var| {
      // Safety: 每个 `var` 来自 `stat_ref.vars`，均为 AST arena 保活、地址稳定的
      // `AstLocal*`，满足 `visit_local` 入参契约。
      unsafe { self.visit_local(var) };
    });
    true
  }

  /// # Safety
  ///
  /// `fn_` 须为非空、正确对齐、指向在本次 attach 遍历期间存活的
  /// `AstExprFunction`（由 `SourceModule` 的 AST arena 保活、地址稳定，dispatch 处
  /// 由 `from_mut(node)` 转裸指针传入）。此外调用方须保证对该节点的**独占可变访问**：
  /// 本函数会写穿 `fn_`（回填 `return_annotation`）并经 `self.allocator` 独占分配，
  /// 故调用点不得同时持有指向该节点的其它引用（`&`/`&mut`），也不得并发访问同一
  /// `TypeAttacher`。`self.allocator` 须为构造期传入、整程存活的 arena 指针。
  ///
  /// 实现注记：本函数按 cpp `TypeAttacher::visit(AstExprFunction* fn)` 语义
  /// **写穿 `fn_`**（回填 returnAnnotation，Ast/src/Parser.cpp 侧字段为此可变）。
  /// 因此体内禁止构造 `&AstExprFunction` 共享引用（如曾经的
  /// `let fn_ref = unsafe { &*fn_ }`）——共享引用活跃期经裸指针写同一内存是
  /// 别名 UB；nightly/edition2024 下 rustc 会为局部共享引用发 readonly 别名
  /// 标注，release（LTO）据此丢弃该 store，症状为 decorateWithTypes 丢失
  /// 函数返回类型注解（unit-test 6 例 release-only 失败的根因）。字段读取
  /// 一律走 `(*fn_)` 裸指针。
  pub(crate) unsafe fn visit_ast_expr_function(&mut self, fn_: *mut AstExprFunction) -> bool {
    // Safety: `fn_` 依入参契约非空/对齐/指向存活 `AstExprFunction`；`args` 数组由
    // AST arena 保活，遍历期地址稳定。
    unsafe { (*fn_).args.iter_nodes() }.for_each(|arg| {
      // Safety: 每个 `arg` 是该函数节点记录的 AST `AstLocal*`，arena 保活、地址稳定，
      // 满足 `visit_local` 入参契约。
      unsafe { self.visit_local(arg.as_ptr()) };
    });

    // Safety: `fn_` 依入参契约存活，此处仅瞬态只读取 `return_annotation` 判空。
    if unsafe { (*fn_).return_annotation.is_null() } {
      // C++ `if (auto result = getScope(fn->body->location))` — Rust
      // `get_scope` always resolves an enclosing scope.
      // Safety: `fn_.body` 为 AST arena 保活、地址稳定的子节点，`base.base.location`
      // 为 repr(C) 前缀字段；此处仅瞬态只读取 location。
      let body = unsafe { (*fn_).body };
      let scope = self.get_scope(&body.base.base.location);
      let ret = scope.return_type;
      let (_v, tail) = flatten_type_pack_id(ret);

      // 注：`map_or(null_mut, ...)` 的结果直存 `AstTypeList.tail_type` 裸指针
      // 字段（读取侧经 optional_node::node_opt 折回 Option），空即「无尾随」，
      // 属落点字段既有布局约定（本批次不改字段类型）。
      let variadic_annotation = tail.map_or(null_mut(), |tail_tp| {
        self.rehydration_visitor().rehydrate(tail_tp)
      });

      let types = self.type_ast_pack(ret);
      let type_list = AstTypeList {
        types,
        tail_type: variadic_annotation,
      };
      let allocator = alias(self.allocator);
      alias(fn_).return_annotation = OptNode::from_ptr(
        allocator
          .alloc(AstTypePackExplicit::new(Location::default(), type_list))
          .cast::<AstTypePack>(),
      );
    }

    true
  }
}
