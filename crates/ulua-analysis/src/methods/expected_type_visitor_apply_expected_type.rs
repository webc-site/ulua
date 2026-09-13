//! @interface-stub
use alloc::vec::Vec;
use core::mem::take;

use ulua_ast::{
  records::{
    ast_expr::AstExpr,
    ast_expr_constant_string::AstExprConstantString,
    ast_expr_group::AstExprGroup,
    ast_expr_if_else::AstExprIfElse,
    ast_expr_table::{AstExprTable, ItemKind},
    ast_node::AstNode,
  },
  rtti::ast_node_try_as,
};
use ulua_common::FFlag;

use crate::{
  functions::{
    extract_matching_table_type::extract_matching_table_type,
    extract_matching_table_type_deprecated::extract_matching_table_type_deprecated,
    follow_type::follow_type_id, get_type_alt_j::get_type_id, is_record::is_record,
  },
  records::{
    expected_type_visitor::ExpectedTypeVisitor, singleton_type::SingletonType,
    string_singleton::StringSingleton, table_type::TableType, union_type::UnionType,
  },
  type_aliases::{singleton_variant::SingletonVariant, type_id::TypeId},
};
impl ExpectedTypeVisitor {
  pub fn apply_expected_type(&mut self, expected_type: TypeId, expr: *const AstExpr) {
    // SAFETY: AST 遍历分发器保证 expr 指向存活的 AstExpr 节点。
    let expr_node = unsafe { &*(expr as *const AstNode) };

    let expected_type = follow_type_id(expected_type);

    // No matter what, we set the expected type of the current expression to
    // whatever was just passed in. We may traverse the type and do more.
    // SAFETY: ast_expected_types 在 visitor 存活期内有效。
    unsafe {
      *(*self.ast_expected_types).get_or_insert(expr) = expected_type;
    }

    if let Some(expr_table) = ast_node_try_as::<AstExprTable>(expr_node) {
      let Some(expected_table_type) = get_type_id::<TableType>(expected_type) else {
        if let Some(utv) = get_type_id::<UnionType>(expected_type)
          // SAFETY: ast_types 在 visitor 存活期内有效。
          && let Some(&expr_type) = unsafe { (*self.ast_types).find(&expr) }
        {
          if FFlag::LuauBidirectionalInferenceBetterUnionHandling.get() {
            if let Some(tt) = extract_matching_table_type(utv, expr_type, self.builtin_types) {
              self.apply_expected_type(tt, expr);
              return;
            }
          } else {
            let mut parts: Vec<TypeId> = utv.options.clone();
            if let Some(tt) = unsafe {
              extract_matching_table_type_deprecated(&mut parts, expr_type, self.builtin_types)
            } {
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
        possible_key_types.push(unsafe {
          (*self.arena).add_type(SingletonType::new(SingletonVariant::V1(
            StringSingleton::new(name.clone()),
          )))
        });
      }

      if let Some(indexer) = &expected_table_type.indexer {
        possible_key_types.push(indexer.index_type);
      }

      let expected_key_type: TypeId = if possible_key_types.is_empty() {
        // SAFETY: builtin_types 指向全局 BuiltinTypes。
        unsafe { (*self.builtin_types).never_type }
      } else if possible_key_types.len() == 1 {
        possible_key_types[0]
      } else {
        // SAFETY: arena 在 visitor 存活期内有效。
        unsafe {
          (*self.arena).add_type(UnionType {
            options: take(&mut possible_key_types),
          })
        }
      };

      for idx in 0..expr_table.items.size {
        // SAFETY: idx < items.size，AstArray 元素连续存储。
        let item = unsafe { &*expr_table.items.data.add(idx) };
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
      self.apply_expected_type(expected_type, group.expr);
    } else if let Some(ternary) = ast_node_try_as::<AstExprIfElse>(expr_node) {
      self.apply_expected_type(expected_type, ternary.true_expr);
      self.apply_expected_type(expected_type, ternary.false_expr);
    }
  }
}
