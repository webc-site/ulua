//! `expected_type_visitor` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::vec::Vec;
use core::{cmp::min, mem::take};

use ulua_ast::{
  records::{
    ast_expr::AstExpr,
    ast_expr_call::AstExprCall,
    ast_expr_constant_string::AstExprConstantString,
    ast_expr_group::AstExprGroup,
    ast_expr_if_else::AstExprIfElse,
    ast_expr_index_expr::AstExprIndexExpr,
    ast_expr_table::{AstExprTable, ItemKind},
    ast_expr_type_assertion::AstExprTypeAssertion,
    ast_node::AstNode,
    ast_stat_assign::AstStatAssign,
    ast_stat_compound_assign::AstStatCompoundAssign,
    ast_stat_local::AstStatLocal,
    ast_stat_return::AstStatReturn,
  },
  rtti::ast_node_try_as,
};
use ulua_common::fflag;

use crate::{
  functions::{
    begin_type::begin_union_type, begin_type_pack::begin, end_type_pack::end_type_pack_id,
    extract_matching_table_type::extract_matching_table_type,
    extract_matching_table_type_deprecated::extract_matching_table_type_deprecated, follow_type,
    get_type, is_record::is_record,
  },
  records::{
    expected_type_visitor::ExpectedTypeVisitor, function_type::FunctionType,
    generic_type_visitor::GenericTypeVisitorTrait, index_collector::IndexCollector,
    singleton_type::SingletonType, string_singleton::StringSingleton, table_type::TableType,
    union_type::UnionType,
  },
  type_aliases::{singleton_variant::SingletonVariant, type_id::TypeId},
};

impl ExpectedTypeVisitor {
  pub fn apply_expected_type(&mut self, expected_type: TypeId, expr: *const AstExpr) {
    // SAFETY: AST 遍历分发器保证 expr 指向存活的 AstExpr 节点。
    let expr_node = unsafe { &*(expr as *const AstNode) };

    let expected_type = follow_type::follow(expected_type);

    // No matter what, we set the expected type of the current expression to
    // whatever was just passed in. We may traverse the type and do more.
    // SAFETY: ast_expected_types 在 visitor 存活期内有效。
    unsafe {
      *(*self.ast_expected_types).get_or_insert(expr) = expected_type;
    }

    if let Some(expr_table) = ast_node_try_as::<AstExprTable>(expr_node) {
      let Some(expected_table_type) = get_type::get::<TableType>(expected_type) else {
        if let Some(utv) = get_type::get::<UnionType>(expected_type)
          // SAFETY: ast_types 在 visitor 存活期内有效。
          && let Some(&expr_type) = unsafe { (*self.ast_types).find(&expr) }
        {
          if fflag::LuauBidirectionalInferenceBetterUnionHandling.get() {
            if let Some(tt) = extract_matching_table_type(utv, expr_type, self.builtin_types) {
              self.apply_expected_type(tt, expr);
              return;
            }
          } else {
            // C++ 直接传 utv，函数内走 TypeIterator 展平嵌套 union 并
            // follow，调用侧先展平对齐语义。
            let mut parts: Vec<TypeId> = begin_union_type(utv).collect();
            if let Some(tt) =
              extract_matching_table_type_deprecated(&mut parts, expr_type, self.builtin_types)
            {
              self.apply_expected_type(tt, expr);
              return;
            }
          }
        }
        return;
      };

      // If we have a table, then the expected type for any given key is a
      // union between all the possible keys and an indexer type (if it exists).
      let mut possible_key_types: Vec<TypeId> = Vec::with_capacity(
        expected_table_type.props.len() + usize::from(expected_table_type.indexer.is_some()),
      );
      for name in expected_table_type.props.keys() {
        // SAFETY: arena 在 visitor 存活期内有效。
        possible_key_types.push(self.arena.get_mut().add_type(SingletonType::new(
          SingletonVariant::V1(StringSingleton::new(name.clone())),
        )));
      }

      if let Some(indexer) = &expected_table_type.indexer {
        possible_key_types.push(indexer.index_type);
      }

      let expected_key_type: TypeId = if possible_key_types.is_empty() {
        // SAFETY: builtin_types 指向全局 BuiltinTypes。
        self.builtin_types.get_mut().never_type
      } else if possible_key_types.len() == 1 {
        possible_key_types[0]
      } else {
        // SAFETY: arena 在 visitor 存活期内有效。
        self.arena.get_mut().add_type(UnionType {
          options: take(&mut possible_key_types),
        })
      };

      for item in expr_table.items.as_slice() {
        if is_record(item) {
          // SAFETY: item.key 由 AST 节点字段保证有效。
          let Some(key_const_string) =
            ast_node_try_as::<AstExprConstantString>(unsafe { &*(item.key as *const AstNode) })
          else {
            continue;
          };
          let key_str = key_const_string.value.as_str().unwrap_or("").to_string();

          // No mater what, we can claim that the expected key type is the
          // union of all possible props plus the indexer.
          self.apply_expected_type(expected_key_type, item.key);

          // - If the property is defined and has a read type, apply it
          //   as an expected type. e.g.:
          //
          //      -- _ will have expected type `number`
          //      local t: { [string]: number, write foo: boolean } = { foo = _ }
          //
          // - Otherwise if the property has an indexer, apply the result type.
          // - Otherwise do nothing.
          if let Some(prop) = expected_table_type.props.get(&key_str) {
            if let Some(read_ty) = prop.read_ty {
              self.apply_expected_type(read_ty, item.value);
            } else if let Some(indexer) = &expected_table_type.indexer {
              self.apply_expected_type(indexer.index_result_type, item.value);
            }
          } else if let Some(indexer) = &expected_table_type.indexer {
            self.apply_expected_type(indexer.index_result_type, item.value);
          }
        } else if item.kind == ItemKind::List
          && let Some(indexer) = &expected_table_type.indexer
        {
          self.apply_expected_type(indexer.index_result_type, item.value);
        } else if item.kind == ItemKind::General
          && let Some(indexer) = &expected_table_type.indexer
        {
          self.apply_expected_type(indexer.index_result_type, item.value);
          self.apply_expected_type(expected_key_type, item.key);
        }
      }
    } else if let Some(group) = ast_node_try_as::<AstExprGroup>(expr_node) {
      // expr 已句柄化；apply_expected_type 为既有裸指针 API，经 as_ptr 桥接。
      self.apply_expected_type(expected_type, group.expr.as_ptr());
    } else if let Some(ternary) = ast_node_try_as::<AstExprIfElse>(expr_node) {
      // true/false_expr 已句柄化；apply_expected_type 为既有裸指针 API，经 as_ptr 桥接。
      self.apply_expected_type(expected_type, ternary.true_expr.as_ptr());
      self.apply_expected_type(expected_type, ternary.false_expr.as_ptr());
    }
  }
}

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
    // 是构造期 `shared_mut(root_scope)` 注入的 Arc<Scope> 内嵌 Scope 地址，
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
