//! `TypeChecker2::testPotentialLiteralIsSubtype`（TypeChecker2.cpp 对照）。
use alloc::{
  string::{String, ToString},
  vec::Vec,
};

use ulua_ast::{
  records::{
    ast_expr::AstExpr,
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_expr_constant_string::AstExprConstantString,
    ast_expr_group::AstExprGroup,
    ast_expr_if_else::AstExprIfElse,
    ast_expr_table::{AstExprTable, ItemKind},
    ast_node,
    ast_node::AstNode,
  },
  rtti::ast_node_try_as,
};
use ulua_common::FFlag;

use crate::{
  functions::{
    extract_matching_table_type::extract_matching_table_type,
    extract_matching_table_type_deprecated::extract_matching_table_type_deprecated,
    follow_type::follow_type_id, get_type_alt_j::get_type_id, is_optional::is_optional,
    is_record::is_record,
    simplify_intersection_simplify_alt_b::simplify_intersection_not_null_builtin_types_not_null_type_arena_type_ids,
  },
  records::{
    intersection_type::IntersectionType,
    missing_properties::{Context as MissingPropertiesContext, MissingProperties},
    singleton_type::SingletonType,
    string_singleton::StringSingleton,
    table_type::TableType,
    type_checker_2::TypeChecker2,
    type_ids::TypeIds,
    unexpected_array_like_table_item::UnexpectedArrayLikeTableItem,
    union_type::UnionType,
  },
  type_aliases::{
    singleton_variant::SingletonVariant, type_error_data::TypeErrorData, type_id::TypeId,
  },
};
impl TypeChecker2 {
  pub fn test_potential_literal_is_subtype(
    &mut self,
    expr: &AstExpr,
    expected_type: TypeId,
  ) -> bool {
    let expr_type = follow_type_id(self.lookup_type(expr));
    let expected_type = follow_type_id(expected_type);

    // SAFETY: AstExpr 是 #[repr(C)] 单继承，AstNode 子对象在偏移 0，cast 有效。
    let node = unsafe { &*((expr as *const AstExpr).cast::<ast_node::AstNode>()) };

    if let Some(group) = ast_node_try_as::<AstExprGroup>(node) {
      // SAFETY: group->expr 指向 AST arena 节点。
      return self.test_potential_literal_is_subtype(unsafe { &*group.expr }, expected_type);
    }

    if let Some(if_else) = ast_node_try_as::<AstExprIfElse>(node) {
      // SAFETY: true/false_expr 指向 AST arena 节点。
      let mut passes =
        unsafe { self.test_potential_literal_is_subtype(&*if_else.true_expr, expected_type) };
      passes &=
        unsafe { self.test_potential_literal_is_subtype(&*if_else.false_expr, expected_type) };
      return passes;
    }

    if let Some(bin_expr) = ast_node_try_as::<AstExprBinary>(node)
      && bin_expr.op == AstExprBinaryOp::Or
    {
      // SAFETY: self.module/builtin_types 由构造方保证有效；&mut self 下经
      // 裸指针 place 写 internal_types（C++ 同契约）。
      let relaxed_expected_lhs = unsafe {
        (*self.module).internal_types.add_type(UnionType {
          options: alloc::vec![(*self.builtin_types).falsy_type, expected_type],
        })
      };
      // SAFETY: left/right 指向 AST arena 节点。
      let mut passes =
        unsafe { self.test_potential_literal_is_subtype(&*bin_expr.left, relaxed_expected_lhs) };
      passes &= unsafe { self.test_potential_literal_is_subtype(&*bin_expr.right, expected_type) };
      return passes;
    }

    let expr_table = ast_node_try_as::<AstExprTable>(node);
    let expr_table_type = get_type_id::<TableType>(expr_type);
    let expected_table_type = get_type_id::<TableType>(expected_type);

    let (Some(expr_table), Some(_)) = (expr_table, expr_table_type) else {
      return self.test_is_subtype_type_id_type_id_location(
        expr_type,
        expected_type,
        expr.base.location,
      );
    };

    let Some(expected_table_type) = expected_table_type else {
      if let Some(expected_union) = get_type_id::<UnionType>(expected_type) {
        if FFlag::LuauBidirectionalInferenceBetterUnionHandling.get() {
          if let Some(tt) =
            extract_matching_table_type(expected_union, expr_type, self.builtin_types)
          {
            // test_literal_or_ast_type_is_subtype（清单外）仍收裸指针。
            return unsafe {
              self.test_literal_or_ast_type_is_subtype(expr as *const AstExpr as *mut AstExpr, tt)
            };
          }
        } else {
          let mut parts = expected_union.options.clone();
          if let Some(tt) = unsafe {
            extract_matching_table_type_deprecated(&mut parts, expr_type, self.builtin_types)
          } {
            return self.test_potential_literal_is_subtype(expr, tt);
          }
        }
      }

      if let Some(expected_intersection) = get_type_id::<IntersectionType>(expected_type) {
        let mut parts = TypeIds::new();
        for &part in &expected_intersection.parts {
          parts.insert_type_id(part);
        }

        // SAFETY: self.module 同上。
        let simplified = unsafe {
          simplify_intersection_not_null_builtin_types_not_null_type_arena_type_ids(
            self.builtin_types,
            &mut (*self.module).internal_types,
            parts,
          )
        }
        .result;

        if get_type_id::<TableType>(simplified).is_some() {
          return self.test_potential_literal_is_subtype(expr, simplified);
        }
      }

      return self.test_is_subtype_type_id_type_id_location(
        expr_type,
        expected_type,
        expr.base.location,
      );
    };

    let mut missing_keys: Vec<String> = Vec::new();
    for (name, prop) in expected_table_type.props.iter() {
      if let Some(read_ty) = prop.read_ty
        && !is_optional(read_ty)
      {
        missing_keys.push(name.clone());
      }
    }

    let scope = self.find_innermost_scope(expr.base.location);
    let mut is_array_like = false;
    if let Some(indexer) = &expected_table_type.indexer {
      // SAFETY: self.subtyping/builtin_types 由构造方保证有效（C++ 同契约）。
      let result = unsafe {
        (*self.subtyping).is_subtype_type_id_type_id_not_null_scope(
          (*self.builtin_types).number_type,
          indexer.index_type,
          scope,
        )
      };
      is_array_like = result.is_subtype
        || self.is_error_suppressing_location_type_id(expr.base.location, indexer.index_type);
    }

    let mut is_subtype = true;
    for item in expr_table.items.iter() {
      if is_record(item) {
        // SAFETY: is_record 保证 item.key 非 null；rtti 偏移 0 cast 有效。
        let key_const_string = ast_node_try_as::<AstExprConstantString>(unsafe {
          &*((item.key as *const AstExpr).cast::<AstNode>())
        });
        let Some(key_const_string) = key_const_string else {
          continue;
        };

        let key_str = key_const_string.value.as_str().unwrap_or("").to_string();

        missing_keys.retain(|key| key != &key_str);

        if let Some(prop) = expected_table_type.props.get(&key_str) {
          if let Some(read_ty) = prop.read_ty {
            // SAFETY: self.module 同上；item.value 指向 AST arena 节点。
            unsafe {
              *(*self.module)
                .ast_expected_types
                .get_or_insert(item.value as *const AstExpr) = read_ty;
            }
            // SAFETY: item.value 指向 AST arena 节点。
            is_subtype &= unsafe { self.test_potential_literal_is_subtype(&*item.value, read_ty) };
          }
        } else if let Some(indexer) = &expected_table_type.indexer {
          // SAFETY: self.module 同上。
          unsafe {
            *(*self.module)
              .ast_expected_types
              .get_or_insert(item.key as *const AstExpr) = indexer.index_type;
            *(*self.module)
              .ast_expected_types
              .get_or_insert(item.value as *const AstExpr) = indexer.index_result_type;
            let inferred_key_type = (*self.module).internal_types.add_type(SingletonType {
              variant: SingletonVariant::V1(StringSingleton {
                value: key_str.clone(),
              }),
            });
            is_subtype &= self.test_is_subtype_type_id_type_id_location(
              inferred_key_type,
              indexer.index_type,
              (*item.key).base.location,
            );
            is_subtype &=
              self.test_potential_literal_is_subtype(&*item.value, indexer.index_result_type);
          }
        }
      } else if item.kind == ItemKind::List {
        if !is_array_like {
          is_subtype = false;
          // SAFETY: item.value 指向 AST arena 节点。
          self.report_error_type_error_data_location(
            TypeErrorData::UnexpectedArrayLikeTableItem(UnexpectedArrayLikeTableItem::default()),
            unsafe { &(*item.value).base.location },
          );
        }

        if let Some(indexer) = &expected_table_type.indexer {
          // SAFETY: self.module 同上。
          unsafe {
            *(*self.module)
              .ast_expected_types
              .get_or_insert(item.value as *const AstExpr) = indexer.index_result_type;
            is_subtype &=
              self.test_potential_literal_is_subtype(&*item.value, indexer.index_result_type);
          }
        }
      } else if item.kind == ItemKind::General
        && let Some(indexer) = &expected_table_type.indexer
      {
        // SAFETY: self.module 同上。
        unsafe {
          *(*self.module)
            .ast_expected_types
            .get_or_insert(item.key as *const AstExpr) = indexer.index_type;
          *(*self.module)
            .ast_expected_types
            .get_or_insert(item.value as *const AstExpr) = indexer.index_result_type;
          is_subtype &= self.test_potential_literal_is_subtype(&*item.key, indexer.index_type);
          is_subtype &=
            self.test_potential_literal_is_subtype(&*item.value, indexer.index_result_type);
        }
      }
    }

    if !missing_keys.is_empty() {
      self.report_error_type_error_data_location(
        TypeErrorData::MissingProperties(MissingProperties {
          super_type: expected_type,
          sub_type: expr_type,
          properties: missing_keys,
          context: MissingPropertiesContext::Missing,
        }),
        &expr.base.location,
      );
      return false;
    }

    is_subtype
  }
}
