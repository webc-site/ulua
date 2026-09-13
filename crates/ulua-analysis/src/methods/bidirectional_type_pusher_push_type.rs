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
  rtti::ast_node_as_const,
};
use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::{relation::Relation, type_function_instance_state::TypeFunctionInstanceState},
  functions::{
    contains_generic_type_utils::contains_generic_type_id_not_null_dense_hash_set_void,
    contains_generic_type_utils_alt_b::contains_generic, extend_type_pack::extend_type_pack,
    extract_matching_table_type::extract_matching_table_type,
    extract_matching_table_type_deprecated::extract_matching_table_type_deprecated,
    flatten_type_pack::flatten_type_pack_id, follow_type::follow_type_id,
    follow_type_pack::follow_type_pack_id, get_type_alt_j::get_type_id,
    get_type_pack::get_type_pack_id, is_literal::is_literal, is_record::is_record,
    maybe_singleton::maybe_singleton, relate_simplify_alt_b::relate_type_id_type_id,
    strip_nil::strip_nil,
  },
  records::{
    any_type::AnyType, bidirectional_type_pusher::BidirectionalTypePusher,
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
      *(*self.ast_expected_types).get_or_insert(expr) = expected_type;
      if !(*self.ast_types).contains(&expr) {
        return (*(*self.solver).builtin_types).any_type;
      }

      let mut expr_type = *(*self.ast_types).find(&expr).unwrap();

      if self.seen.contains(&(expected_type, expr)) {
        return expr_type;
      }
      self.seen.insert((expected_type, expr));

      expected_type = follow_type_id(expected_type);
      expr_type = follow_type_id(expr_type);

      if let Some(tfit) = get_type_id::<TypeFunctionInstanceType>(expected_type)
        && tfit.state == TypeFunctionInstanceState::Unsolved
      {
        self.incomplete_inferences.push(IncompleteInference {
          expected_type,
          target_type: expr_type,
          expr,
        });
        return expr_type;
      }

      if !get_type_id::<BlockedType>(expected_type).is_none()
        || !get_type_id::<PendingExpansionType>(expected_type).is_none()
      {
        self.incomplete_inferences.push(IncompleteInference {
          expected_type,
          target_type: expr_type,
          expr,
        });
        return expr_type;
      }

      if !get_type_id::<AnyType>(expected_type).is_none()
        || !get_type_id::<UnknownType>(expected_type).is_none()
      {
        return expr_type;
      }

      let group = ast_node_as_const::<AstExprGroup>(expr as *const AstNode);
      if !group.is_null() {
        self.push_type(expected_type, (*group).expr as *const AstExpr);
        return expr_type;
      }

      let ternary = ast_node_as_const::<AstExprIfElse>(expr as *const AstNode);
      if !ternary.is_null() {
        self.push_type(expected_type, (*ternary).true_expr as *const AstExpr);
        self.push_type(expected_type, (*ternary).false_expr as *const AstExpr);
        return expr_type;
      }

      if !is_literal(expr) {
        return expr_type;
      }

      if (!ast_node_as_const::<AstExprConstantString>(expr as *const AstNode).is_null()
        || !ast_node_as_const::<AstExprConstantNumber>(expr as *const AstNode).is_null()
        || !ast_node_as_const::<AstExprConstantBool>(expr as *const AstNode).is_null()
        || !ast_node_as_const::<AstExprConstantNil>(expr as *const AstNode).is_null())
        && let Some(ft) = get_type_id::<FreeType>(expr_type)
      {
        if maybe_singleton(expected_type) && maybe_singleton(ft.lower_bound) {
          (*self.solver).bind_not_null_constraint_type_id_type_id(
            self.constraint,
            expr_type,
            ft.lower_bound,
          );
          return expr_type;
        }

        let upper_bound_relation = relate_type_id_type_id(ft.upper_bound, expected_type);
        if upper_bound_relation == Relation::Subset || upper_bound_relation == Relation::Coincident
        {
          (*self.solver).bind_not_null_constraint_type_id_type_id(
            self.constraint,
            expr_type,
            expected_type,
          );
          return expr_type;
        }

        let lower_bound_relation = relate_type_id_type_id(ft.lower_bound, expected_type);
        if lower_bound_relation == Relation::Subset || lower_bound_relation == Relation::Coincident
        {
          (*self.solver).bind_not_null_constraint_type_id_type_id(
            self.constraint,
            expr_type,
            expected_type,
          );
          return expr_type;
        }
      }

      let expr_lambda = ast_node_as_const::<AstExprFunction>(expr as *const AstNode);
      if !expr_lambda.is_null() {
        let lambda_ty = get_type_id::<FunctionType>(expr_type);

        let expected_lambda_ty: Option<&FunctionType> =
          if FFlag::LuauBidirectionalInferenceBetterUnionHandling.get() {
            let mut ffti = FindFunctionTypeIn::new((*expr_lambda).args.size as i32);
            ffti.run_type_id(expected_type);
            // SAFETY: candidate 非 null 时指向类型 arena 中的 FunctionType
            // （外层 unsafe 块覆盖此裸指针 as_ref）
            ffti.candidate.as_ref()
          } else {
            get_type_id::<FunctionType>(strip_nil(
              (*self.solver).builtin_types,
              &mut *(*self.solver).arena,
              expected_type,
            ))
          };

        if let (Some(lambda_ty), Some(expected_lambda_ty)) = (lambda_ty, expected_lambda_ty) {
          let (lambda_arg_tys, _lambda_tail) = flatten_type_pack_id(lambda_ty.arg_types);
          let extended_pack = extend_type_pack(
            &mut *(*self.solver).arena,
            (*self.solver).builtin_types,
            expected_lambda_ty.arg_types,
            (*expr_lambda).args.size,
            Vec::new(),
          );
          let expected_lambda_arg_tys = extended_pack.head;
          let limit = lambda_arg_tys
            .len()
            .min(expected_lambda_arg_tys.len())
            .min((*expr_lambda).args.size);
          for arg_index in 0..limit {
            let local = *(*expr_lambda).args.data.add(arg_index);
            if (*local).annotation.is_null()
              && !get_type_id::<FreeType>(follow_type_id(lambda_arg_tys[arg_index])).is_none()
              && !contains_generic_type_id_not_null_dense_hash_set_void(
                expected_lambda_arg_tys[arg_index],
                self.generic_types_and_packs,
              )
            {
              (*self.solver).bind_not_null_constraint_type_id_type_id(
                self.constraint,
                lambda_arg_tys[arg_index],
                expected_lambda_arg_tys[arg_index],
              );
            }
          }

          if (*expr_lambda).return_annotation.is_null()
            && !get_type_pack_id::<FreeTypePack>(follow_type_pack_id(lambda_ty.ret_types)).is_none()
            && !contains_generic(expected_lambda_ty.ret_types, self.generic_types_and_packs)
          {
            (*self.solver).bind_not_null_constraint_type_pack_id_type_pack_id(
              self.constraint,
              lambda_ty.ret_types,
              expected_lambda_ty.ret_types,
            );
          }
        }
      }

      let expr_table = ast_node_as_const::<AstExprTable>(expr as *const AstNode);
      if !expr_table.is_null() {
        let Some(expected_table_ty) = get_type_id::<TableType>(expected_type) else {
          if let Some(utv) = get_type_id::<UnionType>(expected_type) {
            if FFlag::LuauBidirectionalInferenceBetterUnionHandling.get() {
              if let Some(tt) =
                extract_matching_table_type(utv, expr_type, (*self.solver).builtin_types)
              {
                let _ = self.push_type(tt, expr);
              }
            } else {
              let mut parts: Vec<TypeId> = utv.options.clone();
              if let Some(tt) = extract_matching_table_type_deprecated(
                &mut parts,
                expr_type,
                (*self.solver).builtin_types,
              ) {
                let _ = self.push_type(tt, expr);
              }
            }
          } else if let Some(itv) = get_type_id::<IntersectionType>(expected_type) {
            let parts: Vec<TypeId> = itv.parts.clone();
            for part in parts {
              let _ = self.push_type(part, expr);
            }
            *(*self.ast_expected_types).get_or_insert(expr) = expected_type;
          }
          return expr_type;
        };

        for item in (*expr_table).items.as_slice() {
          if is_record(item) {
            let key_const_string =
              ast_node_as_const::<AstExprConstantString>(item.key as *const AstNode);
            let key_str = (*key_const_string).value.as_str().unwrap_or("").to_string();

            let read_ty_opt = expected_table_ty.props.get(&key_str).map(|p| p.read_ty);
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
              (*self.unifier).unify(index_type, (*(*self.solver).builtin_types).number_type);
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
