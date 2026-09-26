use core::cmp::min;

use ulua_ast::records::{
  ast_expr_call::AstExprCall, ast_expr_index_expr::AstExprIndexExpr,
  ast_expr_type_assertion::AstExprTypeAssertion, ast_stat_assign::AstStatAssign,
  ast_stat_compound_assign::AstStatCompoundAssign, ast_stat_local::AstStatLocal,
  ast_stat_return::AstStatReturn,
};

use crate::{
  functions::{begin_type_pack::begin, end_type_pack::end_type_pack_id, follow_type, get_type},
  records::{
    expected_type_visitor::ExpectedTypeVisitor, function_type::FunctionType,
    generic_type_visitor::GenericTypeVisitorTrait, index_collector::IndexCollector,
    union_type::UnionType,
  },
};

impl ExpectedTypeVisitor {
  pub(crate) fn visit_ast_stat_assign(&mut self, stat: *mut AstStatAssign) -> bool {
    // Safety: `stat` 由 `AstVisitor` 分发器以 `from_mut(&mut node)` 传入，指向
    // AST arena 中存活的 AstStatAssign，本次回调期间是节点的唯一访问路径；
    // vars/values 是 parser 成对写入 arena 的节点指针区，`as_slice` 界内读取，
    // 子元素仅作为 `ast_types` 映射的键做身份比较，不解引用。`self.ast_types`
    // 指向 `populate_expected_types` 构造期由 `&mut (*module).ast_types` 裸化
    // 注入的 Module 字段，随 Module 存活且遍历为单线程串行，仅只读 find。
    unsafe {
      let stat_ref = &*stat;
      // zip 单遍历：min(vars.size, values.size)，消除越界检查
      for (var, value) in stat_ref
        .vars
        .as_slice()
        .iter()
        .zip(stat_ref.values.as_slice())
        .take(min(stat_ref.vars.size, stat_ref.values.size))
      {
        if let Some(&lhs_type) = (*self.ast_types).find(&(*var as *const _)) {
          self.apply_expected_type(lhs_type, *value as *const _);
        }
      }
    }

    true
  }

  pub(crate) fn visit_ast_stat_local(&mut self, stat: *mut AstStatLocal) -> bool {
    // Safety: 同 visit_ast_stat_assign——`stat` 源自分发器 `from_mut`，指向
    // arena 存活节点；`local_types` 注解子指针与 `(*var).annotation` 均只作
    // 映射键身份使用。`self.ast_resolved_types` 为构造期注入的 Module 字段
    // 裸化借用，遍历期存活、只读 find，单线程串行无别名冲突。
    unsafe {
      let stat_ref = &*stat;
      // zip 单遍历：min(vars.size, values.size)，消除越界检查
      for (&var, value) in stat_ref
        .vars
        .as_slice()
        .iter()
        .zip(stat_ref.values.as_slice())
        .take(min(stat_ref.vars.size, stat_ref.values.size))
      {
        if let Some(&annot) = (*self.ast_resolved_types).find(&((*var).annotation as *const _)) {
          self.apply_expected_type(annot, *value as *const _);
        }
      }
    }

    true
  }

  pub(crate) fn visit_ast_stat_compound_assign(
    &mut self,
    stat: *mut AstStatCompoundAssign,
  ) -> bool {
    // Safety: `stat` 源自分发器 `from_mut`，指向 arena 存活 AstStatCompoundAssign；
    // `var`/`value` 已句柄化，地址仅作 `self.ast_types`（构造期注入的 Module 字段
    // 裸化借用，遍历期存活、只读）的键，解引用全部走安全 `.get()`。
    unsafe {
      let var = (*stat).var;
      let lhs_type = (*self.ast_types).find(&(var.as_ptr() as *const _));
      if let Some(lhs_type) = lhs_type {
        self.apply_expected_type(*lhs_type, (*stat).value.as_ptr());
      }
    }
    true
  }

  pub(crate) fn visit_ast_stat_return(&mut self, stat: *mut AstStatReturn) -> bool {
    // Safety: `stat` 源自分发器 `from_mut`，指向 arena 存活节点；`self.root_scope`
    // 是构造期 `arc_as_mut(root_scope)` 注入的 Arc<Scope> 内嵌 Scope 地址，
    // ScopePtr 由调用栈持有、遍历期存活，find_narrowest_scope 只读。return_type
    // 为存活 TypePackId，迭代器 `operator_deref` 读 arena 类型句柄；list 区
    // idx < size 由循环界保证，安全切片读取表达式节点指针交给 apply。
    unsafe {
      let stat_ref = &*stat;

      let scope = self
        .root_scope_mut()
        .find_narrowest_scope_containing(stat_ref.base.base.location);

      let mut it = begin((*scope).return_type);
      let end_it = end_type_pack_id((*scope).return_type);
      for &expr in &stat_ref.list {
        if it == end_it {
          break;
        }
        self.apply_expected_type(*it.current(), expr);
        it.advance();
      }
    }

    true
  }

  pub(crate) fn visit_ast_expr_index_expr(&mut self, expr: *mut AstExprIndexExpr) -> bool {
    // Safety: `expr` 源自分发器 `from_mut`，指向 arena 存活节点，expr/index 是
    // parser 保证非空的子节点指针（仅作映射键与 apply 的存活入参）。
    // `self.ast_types` 与 `self.arena` 为构造期注入的 Module 字段/内部 arena
    // 裸化借用，遍历期存活；`add_type` 的独占可变借用止于本语句，UnionType
    // options 是 IndexCollector 从存活 arena 句柄收集的 TypeId 值。
    unsafe {
      let expr_ref = &*expr;

      // expr/index 已句柄化恒非空；ast_types 身份键与 apply 为既有指针形态，经 as_ptr 桥接。
      if let Some(&ty) = (*self.ast_types).find(&expr_ref.expr.as_ptr().cast_const()) {
        let mut ic = IndexCollector::new(self.arena);
        ic.traverse_type_id(ty);

        if ic.indexes.size() > 1 {
          let union = self.arena.get_mut().add_type(UnionType {
            options: ic.indexes.take(),
          });
          self.apply_expected_type(union, expr_ref.index.as_ptr());
        } else if ic.indexes.size() == 1 {
          let first = ic.indexes.order[0];
          self.apply_expected_type(first, expr_ref.index.as_ptr());
        }
      }
    }

    true
  }

  pub fn visit_ast_expr_call(&mut self, expr: *mut AstExprCall) -> bool {
    // AST 遍历分发器保证节点存活；解引用收口在私有 helper，
    // 公共方法本体不解引用裸指针参数。
    let expr_ref = expr_call_ref(expr);

    // SAFETY: ast_overload_resolved_types / ast_types 在 visitor 存活期内有效。
    let ty = unsafe {
      let mut found = (*self.ast_overload_resolved_types).find(&(expr_ref as *const _ as *const _));
      if found.is_none() {
        found = (*self.ast_types).find(&(expr_ref.func as *const _));
      }
      found
    };

    if let Some(&ty_id) = ty {
      let followed_ty = follow_type::follow(ty_id);
      if let Some(ftv) = get_type::get::<FunctionType>(followed_ty) {
        let mut it = begin(ftv.arg_types);
        let end_it = end_type_pack_id(ftv.arg_types);
        if expr_ref.self_ && it != end_it {
          it.advance();
        }

        for &arg_expr in &expr_ref.args {
          if it == end_it {
            break;
          }
          let arg_type = *it.current();
          self.apply_expected_type(arg_type, arg_expr);
          it.advance();
        }
      }
    }

    true
  }
}

/// 引用化收口：分发 trait 层（`AstVisitor::visit_expr_call`）仅持有
/// `*mut ()`，此处保留指针参数以免公共分发点新增裸指针解引用。
fn expr_call_ref<'a>(expr: *mut AstExprCall) -> &'a AstExprCall {
  // SAFETY: expr 指向存活的 AstExprCall（AST 遍历分发器保证），仅此一处解引用。
  unsafe { &*expr }
}

impl ExpectedTypeVisitor {
  pub(crate) fn visit_ast_expr_type_assertion(&mut self, expr: *mut AstExprTypeAssertion) -> bool {
    // Safety: `expr` 源自分发器 `from_mut(&mut node)`，指向本次遍历中存活的
    // AstExprTypeAssertion 节点；annotation/expr 已句柄化恒非空，仅分别经
    // as_ptr 用作映射键与 apply 的存活入参。
    let expr_ref = unsafe { &*expr };
    // Safety: `self.ast_resolved_types` 由 `populate_expected_types` 构造期以
    // `&mut (*module).ast_resolved_types` 裸化注入，指向 Module 存活字段；本
    // 借用只读 find，与 `expr_ref` 指向的 AST arena 无重叠，单线程串行。
    let ast_resolved_types = unsafe { &*self.ast_resolved_types };

    if let Some(annot) = ast_resolved_types.find(&(expr_ref.annotation.as_ptr().cast_const())) {
      self.apply_expected_type(*annot, expr_ref.expr.as_ptr().cast_const());
    }

    true
  }
}
