//! Faithful port of `BidirectionalTypePusher::pushType`
//! (`Analysis/src/TableLiteralInference.cpp:115-359`).

use alloc::vec::Vec;

use ulua_ast::{
  records::{
    ast_expr::AstExpr,
    ast_expr_constant_bool::AstExprConstantBool,
    ast_expr_constant_nil::AstExprConstantNil,
    ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString,
    ast_expr_function::AstExprFunction,
    ast_expr_group::AstExprGroup,
    ast_expr_if_else::AstExprIfElse,
    ast_expr_table::{AstExprTable, ItemKind},
    ast_node::AstNode,
  },
  rtti::{ast_node_is, ast_node_try_as},
};
use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::{relation::Relation, type_function_instance_state::TypeFunctionInstanceState},
  functions::{
    begin_type::{begin_intersection_type, begin_union_type},
    contains_generic_type_utils::{
      contains_generic, contains_generic_type_id_not_null_dense_hash_set_void,
    },
    extend_type_pack::extend_type_pack,
    extract_matching_table_type::extract_matching_table_type,
    extract_matching_table_type_deprecated::extract_matching_table_type_deprecated,
    flatten_type_pack::flatten_type_pack_id,
    follow_type, follow_type_pack, get_type, get_type_pack,
    is_literal::is_literal,
    is_record::is_record,
    maybe_singleton::maybe_singleton,
    relate_simplify::relate_type_id_type_id,
    strip_nil::strip_nil,
  },
  records::{
    any_type::AnyType, arena_handle::Handle, bidirectional_type_pusher::BidirectionalTypePusher,
    blocked_type::BlockedType, find_function_type_in::FindFunctionTypeIn, free_type::FreeType,
    free_type_pack::FreeTypePack, function_type::FunctionType,
    incomplete_inference::IncompleteInference, intersection_type::IntersectionType,
    iterative_type_visitor::IterativeTypeVisitorTrait,
    pending_expansion_type::PendingExpansionType, table_type::TableType,
    type_function_instance_type::TypeFunctionInstanceType, union_type::UnionType,
    unknown_type::UnknownType,
  },
  type_aliases::type_id::TypeId,
};

impl BidirectionalTypePusher {
  pub fn push_type(&mut self, expected_type: TypeId, expr: *const AstExpr) -> TypeId {
    let mut expected_type = expected_type;
    unsafe {
      *self.ast_expected_types.get_mut().get_or_insert(expr) = expected_type;
      // 双写合一：`!contains` 早退与 `find().unwrap()` 并为一次 `let Some else`
      // return——未命中即返回 any_type，命中直接绑定，消除二次哈希查找。
      let Some(found) = self.ast_types.get_mut().find(&expr) else {
        return self.solver.get_mut().builtin_types.get_mut().any_type;
      };
      let mut expr_type = *found;

      if self.seen.contains(&(expected_type, expr)) {
        return expr_type;
      }
      self.seen.insert((expected_type, expr));

      expected_type = follow_type::follow(expected_type);
      expr_type = follow_type::follow(expr_type);

      if let Some(tfit) = get_type::get::<TypeFunctionInstanceType>(expected_type)
        && tfit.state == TypeFunctionInstanceState::Unsolved
      {
        self.incomplete_inferences.push(IncompleteInference {
          expected_type,
          target_type: expr_type,
          expr,
        });
        return expr_type;
      }

      if get_type::get::<BlockedType>(expected_type).is_some()
        || get_type::get::<PendingExpansionType>(expected_type).is_some()
      {
        self.incomplete_inferences.push(IncompleteInference {
          expected_type,
          target_type: expr_type,
          expr,
        });
        return expr_type;
      }

      if get_type::get::<AnyType>(expected_type).is_some()
        || get_type::get::<UnknownType>(expected_type).is_some()
      {
        return expr_type;
      }

      let node = &*(expr as *const AstNode);

      if let Some(group) = ast_node_try_as::<AstExprGroup>(node) {
        // expr 已句柄化；push_type 为既有裸指针 API，经 as_ptr 桥接。
        self.push_type(expected_type, group.expr.as_ptr());
        return expr_type;
      }

      if let Some(ternary) = ast_node_try_as::<AstExprIfElse>(node) {
        // true/false_expr 已句柄化；push_type 为既有裸指针 API，经 as_ptr 桥接。
        self.push_type(expected_type, ternary.true_expr.as_ptr());
        self.push_type(expected_type, ternary.false_expr.as_ptr());
        return expr_type;
      }

      if !is_literal(expr) {
        return expr_type;
      }

      if (ast_node_is::<AstExprConstantString>(node)
        || ast_node_is::<AstExprConstantNumber>(node)
        || ast_node_is::<AstExprConstantBool>(node)
        || ast_node_is::<AstExprConstantNil>(node))
        && let Some(ft) = get_type::get::<FreeType>(expr_type)
      {
        if maybe_singleton(expected_type) && maybe_singleton(ft.lower_bound) {
          self
            .solver
            .get_mut()
            .bind_not_null_constraint_type_id_type_id(
              self.constraint.as_ptr(),
              expr_type,
              ft.lower_bound,
            );
          return expr_type;
        }

        let upper_bound_relation = relate_type_id_type_id(ft.upper_bound, expected_type);
        if matches!(
          upper_bound_relation,
          Relation::Subset | Relation::Coincident
        ) {
          self
            .solver
            .get_mut()
            .bind_not_null_constraint_type_id_type_id(
              self.constraint.as_ptr(),
              expr_type,
              expected_type,
            );
          return expr_type;
        }

        let lower_bound_relation = relate_type_id_type_id(ft.lower_bound, expected_type);
        if matches!(
          lower_bound_relation,
          Relation::Subset | Relation::Coincident
        ) {
          self
            .solver
            .get_mut()
            .bind_not_null_constraint_type_id_type_id(
              self.constraint.as_ptr(),
              expr_type,
              expected_type,
            );
          return expr_type;
        }
      }

      if let Some(expr_lambda) = ast_node_try_as::<AstExprFunction>(node) {
        let lambda_ty = get_type::get::<FunctionType>(expr_type);

        let expected_lambda_ty: Option<&FunctionType> =
          if fflag::LuauBidirectionalInferenceBetterUnionHandling.get() {
            let mut ffti = FindFunctionTypeIn::new(expr_lambda.args.len() as i32);
            ffti.run_type_id(expected_type);
            // SAFETY: candidate 非 null 时指向类型 arena 中的 FunctionType
            // （外层 unsafe 块覆盖此裸指针 as_ref）
            ffti.candidate.as_ref()
          } else {
            get_type::get::<FunctionType>(strip_nil(
              Handle::from_ptr(self.solver.get_mut().builtin_types.as_ptr()),
              self.solver.get_mut().arena.get_mut(),
              expected_type,
            ))
          };

        if let (Some(lambda_ty), Some(expected_lambda_ty)) = (lambda_ty, expected_lambda_ty) {
          let (lambda_arg_tys, _lambda_tail) = flatten_type_pack_id(lambda_ty.arg_types);
          let extended_pack = extend_type_pack(
            self.solver.get_mut().arena.get_mut(),
            Handle::from_ptr(self.solver.get_mut().builtin_types.as_ptr()),
            expected_lambda_ty.arg_types,
            expr_lambda.args.len(),
            Vec::new(),
          );
          let expected_lambda_arg_tys = extended_pack.head;
          let limit = lambda_arg_tys
            .len()
            .min(expected_lambda_arg_tys.len())
            .min(expr_lambda.args.len());
          // 三序列 zip 单遍历，消除越界检查
          for ((local, &lambda_arg_ty), &expected_lambda_arg_ty) in expr_lambda
            .args
            .iter()
            .zip(&lambda_arg_tys)
            .zip(&expected_lambda_arg_tys)
            .take(limit)
          {
            if local.annotation.is_null()
              && get_type::get::<FreeType>(follow_type::follow(lambda_arg_ty)).is_some()
              && !contains_generic_type_id_not_null_dense_hash_set_void(
                expected_lambda_arg_ty,
                self.generic_types_and_packs.as_ptr(),
              )
            {
              self
                .solver
                .get_mut()
                .bind_not_null_constraint_type_id_type_id(
                  self.constraint.as_ptr(),
                  lambda_arg_ty,
                  expected_lambda_arg_ty,
                );
            }
          }

          if expr_lambda.return_annotation.is_null()
            && get_type_pack::get::<FreeTypePack>(follow_type_pack::follow(lambda_ty.ret_types))
              .is_some()
            && !contains_generic(
              expected_lambda_ty.ret_types,
              self.generic_types_and_packs.as_ptr(),
            )
          {
            self
              .solver
              .get_mut()
              .bind_not_null_constraint_type_pack_id_type_pack_id(
                self.constraint.as_ptr(),
                lambda_ty.ret_types,
                expected_lambda_ty.ret_types,
              );
          }
        }
      }

      if let Some(expr_table) = ast_node_try_as::<AstExprTable>(node) {
        let Some(expected_table_ty) = get_type::get::<TableType>(expected_type) else {
          if let Some(utv) = get_type::get::<UnionType>(expected_type) {
            if fflag::LuauBidirectionalInferenceBetterUnionHandling.get() {
              if let Some(tt) = extract_matching_table_type(
                utv,
                expr_type,
                Handle::from_ptr(self.solver.get_mut().builtin_types.as_ptr()),
              ) {
                let _ = self.push_type(tt, expr);
              }
            } else {
              // C++ 直接传 utv，函数内走 TypeIterator 展平嵌套 union 并
              // follow，调用侧先展平对齐语义。
              let mut parts: Vec<TypeId> = begin_union_type(utv).collect();
              if let Some(tt) = extract_matching_table_type_deprecated(
                &mut parts,
                expr_type,
                Handle::from_ptr(self.solver.get_mut().builtin_types.as_ptr()),
              ) {
                let _ = self.push_type(tt, expr);
              }
            }
          } else if let Some(itv) = get_type::get::<IntersectionType>(expected_type) {
            // C++ `for (const auto part : itv)`——IntersectionTypeIterator
            // 展平嵌套 intersection 并 follow，裸遍历 parts 会漏掉嵌套成员。
            for part in begin_intersection_type(itv) {
              let _ = self.push_type(part, expr);
            }
            *self.ast_expected_types.get_mut().get_or_insert(expr) = expected_type;
          }
          return expr_type;
        };

        for item in expr_table.items.as_slice() {
          if is_record(item) {
            let key_const_string =
              ast_node_try_as::<AstExprConstantString>(&*(item.key as *const AstNode))
                .expect("record key is a constant string");
            let key_str = key_const_string.value.as_str().unwrap_or("");

            let read_ty_opt = expected_table_ty.props.get(key_str).map(|p| p.read_ty);
            match read_ty_opt {
              None => {
                let index_result_type = expected_table_ty
                  .indexer
                  .as_ref()
                  .map(|ix| ix.index_result_type);
                if let Some(index_result_type) = index_result_type {
                  let _ = self.push_type(index_result_type, item.value as *const AstExpr);
                }
                continue;
              }
              Some(read_ty) => {
                if let Some(read_ty) = read_ty {
                  let _ = self.push_type(read_ty, item.value as *const AstExpr);
                }
              }
            }
          } else if item.kind == ItemKind::List {
            let index_pair = expected_table_ty
              .indexer
              .as_ref()
              .map(|ix| (ix.index_type, ix.index_result_type));
            if let Some((index_type, index_result_type)) = index_pair {
              self.unifier.get_mut().unify(
                index_type,
                self.solver.get_mut().builtin_types.get_mut().number_type,
              );
              let _ = self.push_type(index_result_type, item.value as *const AstExpr);
            }
          } else if item.kind == ItemKind::General {
            let index_pair = expected_table_ty
              .indexer
              .as_ref()
              .map(|ix| (ix.index_type, ix.index_result_type));
            if let Some((index_type, index_result_type)) = index_pair {
              let _ = self.push_type(index_type, item.key as *const AstExpr);
              let _ = self.push_type(index_result_type, item.value as *const AstExpr);
            }
          } else {
            LUAU_ASSERT!(false /* "Unexpected" */);
          }
        }
      }

      expr_type
    }
  }
}
