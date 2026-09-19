use alloc::{string::String, sync::Arc};
use core::str::from_utf8;

use ulua_ast::{
  records::{
    ast_expr_constant_string::AstExprConstantString,
    ast_expr_table::{AstExprTable, ItemKind},
    ast_node::AstNode,
  },
  rtti::ast_node_try_as,
};

use crate::{
  enums::table_state::TableState,
  functions::{
    follow_type::follow_type_id, get_table_type::get_table_type, get_type_alt_j::get_type_id,
    maybe_string::maybe_string,
  },
  records::{
    function_type::FunctionType, module::Module, property_type::Property,
    table_indexer::TableIndexer, table_type::TableType, type_checker::TypeChecker,
  },
  type_aliases::{props_type::Props, scope_ptr_type::ScopePtr, type_id::TypeId},
};
impl TypeChecker {
  pub fn check_expr_table(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprTable,
    field_types: &[(TypeId, TypeId)],
    expected_type: Option<TypeId>,
  ) -> TypeId {
    let mut props: Props = Props::new();
    let mut indexer: Option<TableIndexer> = None;

    let mut expected_table: Option<&TableType> = None;

    if let Some(expected_type) = expected_type
      && let Some(ttv) = get_type_id::<TableType>(follow_type_id(expected_type))
      && ttv.state == TableState::Sealed
    {
      expected_table = Some(ttv);
    }

    let items = expr.items.as_slice();

    for (i, item) in items.iter().enumerate() {
      let k = item.key;
      let value = item.value;

      let (key_type, value_type) = field_types[i];

      if item.kind == ItemKind::List {
        if indexer.is_none()
          && let Some(et) = expected_table
        {
          indexer = et.indexer;
        }

        if let Some(indexer) = indexer {
          // SAFETY: value 指向 AST arena 节点。
          let value_loc = unsafe { (*value).base.location };
          self.unify_type_id_type_id_scope_ptr_location(
            self.number_type,
            indexer.index_type,
            scope,
            &value_loc,
          );
          self.unify_type_id_type_id_scope_ptr_location(
            value_type,
            indexer.index_result_type,
            scope,
            &value_loc,
          );
        } else {
          let number_type = self.number_type;
          let result_ty = self.any_if_nonstrict(value_type);
          indexer = Some(TableIndexer {
            index_type: number_type,
            index_result_type: result_ty,
            is_read_only: false,
          });
        }
      } else if item.kind == ItemKind::Record || item.kind == ItemKind::General {
        // SAFETY: k 指向 AST arena 节点；AstExpr #[repr(C)] 单继承，
        // base(AstNode) 在偏移 0，cast 有效。
        let key_str = ast_node_try_as::<AstExprConstantString>(unsafe { &*k.cast::<AstNode>() })
          .map(|key| String::from(from_utf8(key.value.as_bytes()).unwrap_or("")));

        if let Some(key_str) = key_str {
          let mut expr_type = follow_type_id(value_type);
          if self.is_nonstrict_mode()
            && get_table_type(expr_type).is_none()
            && get_type_id::<FunctionType>(expr_type).is_none()
          {
            expr_type = self.any_type;
          }

          // SAFETY: k 指向 AST arena 节点。
          let k_loc = unsafe { (*k).base.location };

          if let Some(expected_table) = expected_table {
            if let Some(it) = expected_table.props.get(&key_str) {
              let expected_prop = it.clone();
              let errors =
                self.try_unify(expr_type, expected_prop.type_deprecated(), scope, &k_loc);
              if errors.is_empty() {
                expr_type = expected_prop.type_deprecated();
              }
            } else if let Some(et_indexer) = expected_table.indexer.as_ref()
              && maybe_string(et_indexer.index_type)
            {
              let index_result_type = et_indexer.index_result_type;
              let errors = self.try_unify(expr_type, index_result_type, scope, &k_loc);
              if errors.is_empty() {
                expr_type = index_result_type;
              }
            }
          }

          props.insert(
            key_str,
            Property::property_type_id_bool_string_optional_location_tags_optional_string_optional_location(
              expr_type,
              false,
              String::new(),
              Some(k_loc),
              Default::default(),
              None,
              None,
            ),
          );
        } else {
          if indexer.is_none()
            && let Some(et) = expected_table
          {
            indexer = et.indexer;
          }

          if let Some(indexer) = indexer {
            // SAFETY: k/value 指向 AST arena 节点。
            let (k_loc, value_loc) = unsafe { ((*k).base.location, (*value).base.location) };
            self.unify_type_id_type_id_scope_ptr_location(
              key_type,
              indexer.index_type,
              scope,
              &k_loc,
            );
            self.unify_type_id_type_id_scope_ptr_location(
              value_type,
              indexer.index_result_type,
              scope,
              &value_loc,
            );
          } else if self.is_nonstrict_mode() {
            indexer = Some(TableIndexer {
              index_type: self.any_type,
              index_result_type: self.any_type,
              is_read_only: false,
            });
          } else {
            indexer = Some(TableIndexer {
              index_type: key_type,
              index_result_type: value_type,
              is_read_only: false,
            });
          }
        }
      }
    }

    let state = TableState::Unsealed;
    let mut table = TableType::table_type_props_optional_table_indexer_type_level_table_state(
      &props,
      indexer,
      scope.level,
      state,
    );
    table.definition_module_name = self.current_module.as_ref().unwrap().name.clone();
    table.definition_location = expr.base.base.location;
    // SAFETY: current_module 在类型检查期间独占使用（C++ 直接改 module->internalTypes 同义）。
    unsafe {
      let module = Arc::as_ptr(self.current_module.as_ref().unwrap()) as *mut Module;
      (*module).internal_types.add_type(table)
    }
  }
}
