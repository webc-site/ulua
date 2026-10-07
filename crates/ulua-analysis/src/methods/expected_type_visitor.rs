//! `expected_type_visitor` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::vec::Vec;
use core::{mem::take, ptr::from_ref};

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
    ast_stat_assign::AstStatAssign,
    ast_stat_compound_assign::AstStatCompoundAssign,
    ast_stat_local::AstStatLocal,
    ast_stat_return::AstStatReturn,
  },
  rtti::{AstNodeView, ast_node_try_as},
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
    arena_handle::{alias, alias_ref},
    expected_type_visitor::ExpectedTypeVisitor,
    function_type::FunctionType,
    generic_type_visitor::GenericTypeVisitorTrait,
    index_collector::IndexCollector,
    singleton_type::SingletonType,
    string_singleton::StringSingleton,
    table_type::TableType,
    union_type::UnionType,
  },
  type_aliases::{singleton_variant::SingletonVariant, type_id::TypeId},
};

impl ExpectedTypeVisitor {
  fn apply_expected_type(&mut self, expected_type: TypeId, expr: *const AstExpr) {
    // 同型 alias_ref 收口后经 `AstNodeView` 安全下溯；不再手工 `as *const AstNode`
    // 跨基座转型。
    let expr_node = alias_ref(expr);

    let expected_type = follow_type::follow(expected_type);

    // No matter what, we set the expected type of the current expression to
    // whatever was just passed in. We may traverse the type and do more.
    *alias(self.ast_expected_types).get_or_insert(expr) = expected_type;

    if let Some(expr_table) = ast_node_try_as::<AstExprTable>(expr_node) {
      let Some(expected_table_type) = get_type::get::<TableType>(expected_type) else {
        if let Some(utv) = get_type::get::<UnionType>(expected_type)
          && let Some(&expr_type) = alias_ref(self.ast_types).find(&expr)
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
        // builtin_types 指向全局 BuiltinTypes，此处只读 never_type 字段。
        self.builtin_types.get().never_type
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
          let Some(key_const_string) =
            ast_node_try_as::<AstExprConstantString>(alias_ref(item.key))
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
  pub(crate) fn visit_ast_stat_assign(&mut self, stat: &mut AstStatAssign) -> bool {
    // zip 天然以较短切片终止，min/take 为冗余越界检查，已删。
    for (var, value) in stat.vars.as_slice().iter().zip(stat.values.as_slice()) {
      if let Some(&lhs_type) = alias_ref(self.ast_types).find(&(*var).cast_const()) {
        self.apply_expected_type(lhs_type, (*value).cast_const());
      }
    }

    true
  }

  pub(crate) fn visit_ast_stat_local(&mut self, stat: &mut AstStatLocal) -> bool {
    for (&var, value) in stat.vars.as_slice().iter().zip(stat.values.as_slice()) {
      if let Some(&annot) =
        alias_ref(self.ast_resolved_types).find(&alias_ref(var).annotation.cast_const())
      {
        self.apply_expected_type(annot, (*value).cast_const());
      }
    }

    true
  }

  pub(crate) fn visit_ast_stat_compound_assign(
    &mut self,
    stat: &mut AstStatCompoundAssign,
  ) -> bool {
    let var = stat.var;
    let lhs_type = alias_ref(self.ast_types).find(&var.as_ptr().cast_const());
    if let Some(lhs_type) = lhs_type {
      self.apply_expected_type(*lhs_type, stat.value.as_ptr());
    }
    true
  }

  pub(crate) fn visit_ast_stat_return(&mut self, stat: &mut AstStatReturn) -> bool {
    let scope = self
      .root_scope_mut()
      .find_narrowest_scope_containing(stat.base.base.location);

    let mut it = begin(alias_ref(scope).return_type);
    let end_it = end_type_pack_id(alias_ref(scope).return_type);
    for &expr in &stat.list {
      if it == end_it {
        break;
      }
      self.apply_expected_type(*it.current(), expr);
      it.advance();
    }

    true
  }

  pub(crate) fn visit_ast_expr_index_expr(&mut self, expr: &mut AstExprIndexExpr) -> bool {
    // expr/index 已句柄化恒非空；ast_types 身份键与 apply 为既有指针形态，经 as_ptr 桥接。
    if let Some(&ty) = alias_ref(self.ast_types).find(&expr.expr.as_ptr().cast_const()) {
      let mut ic = IndexCollector::new(self.arena);
      ic.traverse_type_id(ty);

      if ic.indexes.size() > 1 {
        let union = self.arena.get_mut().add_type(UnionType {
          options: ic.indexes.take(),
        });
        self.apply_expected_type(union, expr.index.as_ptr());
      } else if ic.indexes.size() == 1 {
        let first = ic.indexes.order[0];
        self.apply_expected_type(first, expr.index.as_ptr());
      }
    }

    true
  }

  pub fn visit_ast_expr_call(&mut self, expr: &mut AstExprCall) -> bool {
    let ty = {
      // 节点基座视图经 `AstNodeView` 安全获取，键身份（地址）与原双重 `as` 转型逐位一致。
      let mut found =
        alias_ref(self.ast_overload_resolved_types).find(&from_ref(expr.as_ast_node()));
      if found.is_none() {
        found = alias_ref(self.ast_types).find(&expr.func.cast_const());
      }
      found
    };

    if let Some(&ty_id) = ty {
      let followed_ty = follow_type::follow(ty_id);
      if let Some(ftv) = get_type::get::<FunctionType>(followed_ty) {
        let mut it = begin(ftv.arg_types);
        let end_it = end_type_pack_id(ftv.arg_types);
        if expr.self_ && it != end_it {
          it.advance();
        }

        for &arg_expr in &expr.args {
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
impl ExpectedTypeVisitor {
  pub(crate) fn visit_ast_expr_type_assertion(&mut self, expr: &mut AstExprTypeAssertion) -> bool {
    let ast_resolved_types = alias_ref(self.ast_resolved_types);

    if let Some(annot) = ast_resolved_types.find(&(expr.annotation.as_ptr().cast_const())) {
      self.apply_expected_type(*annot, expr.expr.as_ptr().cast_const());
    }

    true
  }
}
