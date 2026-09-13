use alloc::{string::String, sync::Arc, vec::Vec};
use core::{ptr::null_mut, str::from_utf8};

use ulua_ast::{
  records::{
    ast_expr_constant_string::AstExprConstantString, ast_expr_table::AstExprTable,
    ast_node::AstNode,
  },
  rtti::ast_node_as,
};
use ulua_common::{FFlag, FInt};

use crate::{
  enums::{table_state::TableState, type_context::TypeContext},
  functions::{
    add_all_as_reverse_dependencies::add_all_as_reverse_dependencies, checkpoint::checkpoint,
    follow_type::follow_type_id, for_each_constraint::for_each_constraint,
    get_mutable_type::get_mutable_type_id,
  },
  records::{
    constraint_generator::ConstraintGenerator, in_conditional_context::InConditionalContext,
    inference::Inference, module::Module, property_type::Property,
    push_type_constraint::PushTypeConstraint, table_indexer::TableIndexer, table_type::TableType,
  },
  type_aliases::{constraint_v::ConstraintV, scope_ptr_type::ScopePtr, type_id::TypeId},
};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn check_scope_ptr_ast_expr_table_optional_type_id(
    &mut self,
    scope: &ScopePtr,
    expr: *mut AstExprTable,
    expected_type: Option<TypeId>,
  ) -> Inference {
    let _in_context =
      unsafe { InConditionalContext::new(&mut self.type_context, TypeContext::Default) };

    unsafe {
      let table_ty = (*self.arena).add_type(TableType::new());
      // table_ty 刚由 add_type(TableType) 分配，必命中；对照 C++ `getMutable<TableType>(tableTy)` 后的 LUAU_ASSERT
      let ttv = get_mutable_type_id::<TableType>(table_ty);
      ulua_common::LUAU_ASSERT!(ttv.is_some());
      let ttv = ttv.unwrap();

      ttv.state = TableState::Unsealed;
      if let Some(module) = &self.module {
        ttv.definition_module_name = module.name.clone();
      }
      ttv.definition_location = (*expr).base.base.location;
      ttv.scope = scope.as_ref() as *const _ as *mut _;

      let primitive_limit = FInt::LuauPrimitiveInferenceInTableLimit.get();
      let large_table = primitive_limit > 0 && (*expr).items.size > primitive_limit as usize;
      if large_table {
        self.large_table_depth += 1;
      }

      if let Some(interior) = self.interior_free_types.last_mut() {
        interior.types.push(table_ty);
      }

      let mut index_key_lower_bound: Vec<TypeId> = Vec::new();
      let mut index_value_lower_bound: Vec<TypeId> = Vec::new();

      let mut create_indexer = |current_index_type: TypeId, current_result_type: TypeId| {
        let key = follow_type_id(current_index_type);
        if !index_key_lower_bound.contains(&key) {
          index_key_lower_bound.push(key);
        }

        let value = follow_type_id(current_result_type);
        if !index_value_lower_bound.contains(&value) {
          index_value_lower_bound.push(value);
        }
      };

      let start = checkpoint(self as *const ConstraintGenerator);

      for item in (*expr).items.as_slice() {
        let item_ty = self
          .check_scope_ptr_ast_expr_optional_type_id_bool_bool(
            scope, item.value, None, false, false,
          )
          .ty;

        if !item.key.is_null() {
          let key_ty = self.check_scope_ptr_ast_expr(scope, item.key).ty;
          let key = ast_node_as::<AstExprConstantString>(item.key as *mut AstNode);

          if !key.is_null() {
            let prop_name = String::from(from_utf8((*key).value.as_bytes()).unwrap_or(""));
            let mut prop = Property::rw_type_id(item_ty);
            prop.location = Some((*key).base.base.location);
            ttv.props.insert(prop_name, prop);
          } else {
            create_indexer(key_ty, item_ty);
          }
        } else {
          create_indexer((*self.builtin_types).number_type, item_ty);
        }
      }

      let end = checkpoint(self as *const ConstraintGenerator);

      if !index_key_lower_bound.is_empty() {
        ulua_common::LUAU_ASSERT!(!index_value_lower_bound.is_empty());

        let index_key = if index_key_lower_bound.len() == 1 {
          index_key_lower_bound[0]
        } else {
          self.make_union_vector_type_id(index_key_lower_bound)
        };

        let index_value = if index_value_lower_bound.len() == 1 {
          index_value_lower_bound[0]
        } else {
          self.make_union_vector_type_id(index_value_lower_bound)
        };

        ttv.indexer = Some(TableIndexer {
          index_type: index_key,
          index_result_type: index_value,
          is_read_only: false,
        });
      }

      if let Some(expected_type) = expected_type
        && let Some(module) = &self.module
      {
        let module_ptr = Arc::as_ptr(module) as *mut Module;
        let ptc = self.add_constraint_scope_ptr_location_constraint_v(
          scope,
          (*expr).base.base.location,
          ConstraintV::PushType(PushTypeConstraint {
            expected_type,
            target_type: table_ty,
            ast_types: &(*module_ptr).ast_types as *const _,
            ast_expected_types: &(*module_ptr).ast_expected_types as *const _,
            expr: expr as *const _,
          }),
        );

        if FFlag::LuauConstraintGraph.get() {
          add_all_as_reverse_dependencies(start, end, self, ptc);
        } else {
          for_each_constraint(start, end, self, |c| {
            (*c).deprecated_dependencies.push(ptc);
          });
        }
      }

      if large_table {
        self.large_table_depth -= 1;
      }

      Inference::inference_type_id_refinement_id(table_ty, null_mut())
    }
  }
}
