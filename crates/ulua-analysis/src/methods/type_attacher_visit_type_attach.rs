use core::ptr::null_mut;

use ulua_ast::records::{
  ast_expr_function::AstExprFunction, ast_expr_local::AstExprLocal, ast_stat_for::AstStatFor,
  ast_stat_for_in::AstStatForIn, ast_stat_local::AstStatLocal, ast_type_list::AstTypeList,
  ast_type_pack::AstTypePack, ast_type_pack_explicit::AstTypePackExplicit, location::Location,
  node_handle::OptNode,
};

use crate::{
  functions::flatten_type_pack::flatten_type_pack_id,
  records::{
    type_attacher::TypeAttacher, type_rehydration_options::TypeRehydrationOptions,
    type_rehydration_visitor::TypeRehydrationVisitor,
  },
  type_aliases::{synthetic_names::SyntheticNames, type_pack_id::TypePackId},
};

impl TypeAttacher {
  /// `AstVisitor::visit_stat_local` 的实现桥接。`al` 由 dispatch 处
  /// `from_mut(node)`（见 `attach_type_data.rs`）取得，即当前遍历节点的
  /// 可变借用再转裸指针，故非空、对齐且在整个 attach 期间指向 `SourceModule`
  /// AST arena 保活的 `AstStatLocal`。
  pub(crate) fn visit_ast_stat_local(&mut self, al: *mut AstStatLocal) -> bool {
    // Safety: `al` 源自 `AstVisitor` dispatch 的 `from_mut(node)`，指向遍历期存活的
    // `AstStatLocal`；转共享引用仅读取 `vars`，本函数不对该节点写入。
    let al_ref = unsafe { &*al };

    for &var in al_ref.vars.as_slice() {
      // Safety: `var` 取自 `al_ref.vars`，均指同一 AST arena 保活、地址稳定的
      // `AstLocal`，满足被调 `unsafe fn visit_local` 的入参存活契约。
      unsafe { self.visit_local(var) };
    }

    true
  }

  /// `AstVisitor::visit_expr_local` 桥接。`al` 同上，为 dispatch `from_mut(node)`
  /// 传入的、遍历期存活的 `AstExprLocal` 裸指针。
  pub(crate) fn visit_ast_expr_local(&mut self, al: *mut AstExprLocal) -> bool {
    // Safety: `al` 源自 dispatch `from_mut(node)`，指向遍历期存活的 `AstExprLocal`，
    // 此处仅只读取其 `local` 字段。
    let al_ref = unsafe { &*al };
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
    // Safety: `stat` 源自 dispatch `from_mut(node)`，指向遍历期存活的 `AstStatForIn`；
    // 共享引用仅读取 `vars`，本函数不写该节点。
    let stat_ref = unsafe { &*stat };
    for &var in stat_ref.vars.as_slice() {
      // Safety: 每个 `var` 来自 `stat_ref.vars`，均为 AST arena 保活、地址稳定的
      // `AstLocal*`，满足 `visit_local` 入参契约。
      unsafe { self.visit_local(var) };
    }
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
    for arg in unsafe { (*fn_).args.iter_nodes() } {
      // Safety: 每个 `arg` 是该函数节点记录的 AST `AstLocal*`，arena 保活、地址稳定，
      // 满足 `visit_local` 入参契约。
      unsafe { self.visit_local(arg.as_ptr()) };
    }

    // Safety: `fn_` 依入参契约存活，此处仅瞬态只读取 `return_annotation` 判空。
    if unsafe { (*fn_).return_annotation.is_null() } {
      // C++ `if (auto result = getScope(fn->body->location))` — Rust
      // `get_scope` always resolves an enclosing scope.
      // Safety: `fn_.body` 为 AST arena 保活、地址稳定的子节点，`base.base.location`
      // 为 repr(C) 前缀字段；此处仅瞬态只读取 location。
      let body = unsafe { (*fn_).body };
      let body_location = body.base.base.location;
      let result = self.get_scope(&body_location);
      let ret: TypePackId = result.return_type;
      let (_v, tail) = flatten_type_pack_id(ret);

      let mut variadic_annotation: *mut AstTypePack = null_mut();
      if let Some(tail_tp) = tail {
        let mut rehydrator =
          TypeRehydrationVisitor::type_rehydration_visitor_type_rehydration_visitor(
            self.allocator,
            &mut self.synthetic_names as *mut SyntheticNames,
            &TypeRehydrationOptions::default(),
          );
        variadic_annotation = rehydrator.rehydrate(tail_tp);
      }

      let types = self.type_ast_pack(ret);
      let type_list = AstTypeList {
        types,
        tail_type: variadic_annotation,
      };
      // Safety: `self.allocator` 为构造期传入、整程存活的 AST arena 指针，独占借出
      // 可变引用做分配；此刻无其它并存 `&mut Allocator`。
      let allocator = unsafe { &mut *self.allocator };
      // Safety: `fn_` 依入参契约指向存活且独占可写的 `AstExprFunction`，体内未构造
      // 其共享引用；`allocator` 分配的 `AstTypePackExplicit` 落在此 arena、与 `fn_`
      // 同生命周期，写穿 `return_annotation`（cpp 直译）。
      unsafe {
        (*fn_).return_annotation = OptNode::from_ptr(
          allocator
            .alloc(AstTypePackExplicit::new(Location::default(), type_list))
            .cast::<AstTypePack>(),
        );
      }
    }

    true
  }
}
