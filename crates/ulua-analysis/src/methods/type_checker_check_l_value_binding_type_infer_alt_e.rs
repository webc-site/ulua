use alloc::string::String;

use ulua_ast::{
  records::{
    ast_expr_constant_string::AstExprConstantString, ast_expr_index_expr::AstExprIndexExpr,
  },
  rtti::ast_node_try_as,
};
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::{table_state::TableState, value_context::ValueContext},
  functions::{
    follow_type::follow_type_id, get_mutable_table_type::get_mutable_table_type,
    get_type_alt_j::get_type_id, is_table_intersection::is_table_intersection,
    lookup_extern_type_prop::lookup_extern_type_prop, reduce_union::reduce_union,
  },
  records::{
    any_type::AnyType,
    cannot_extend_table::{CannotExtendTable, Context},
    dynamic_property_lookup_on_extern_types_unsafe::DynamicPropertyLookupOnExternTypesUnsafe,
    extern_type::ExternType,
    intersection_type::IntersectionType,
    never_type::NeverType,
    not_a_table::NotATable,
    table_indexer::TableIndexer,
    table_type::TableType,
    type_checker::TypeChecker,
    type_error::TypeError,
    union_type::UnionType,
    unknown_property::UnknownProperty,
  },
  type_aliases::{
    error_type::ErrorType, name_type::Name, scope_ptr_type::ScopePtr,
    type_error_data::TypeErrorData, type_id::TypeId,
  },
};
impl TypeChecker {
  pub fn check_l_value_binding_scope_ptr_ast_expr_index_expr_value_context(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprIndexExpr,
    ctx: ValueContext,
  ) -> TypeId {
    // SAFETY: expr.expr / expr.index 指向 AST arena 节点（parser 保证非空）。
    let (expr_ref, index_ref) = unsafe { (&*expr.expr, &*expr.index) };
    let index_loc = index_ref.base.location;

    let expr_type = self
      .check_expr_scope_ptr_ast_expr_optional_type_id_bool(scope, expr_ref, None, false)
      .r#type;
    self.tablify(expr_type);
    let expr_type = self.strip_from_nil_and_report(expr_type, &expr_ref.base.location);
    let index_type = self
      .check_expr_scope_ptr_ast_expr_optional_type_id_bool(scope, index_ref, None, false)
      .r#type;
    let expr_type = follow_type_id(expr_type);

    if get_type_id::<AnyType>(expr_type).is_some() || get_type_id::<ErrorType>(expr_type).is_some()
    {
      return expr_type;
    }

    if get_type_id::<NeverType>(expr_type).is_some() {
      return self.unknown_type;
    }

    let value = ast_node_try_as::<AstExprConstantString>(&index_ref.base);
    let value_name: Option<Name> =
      value.map(|v| String::from_utf8_lossy(v.value.as_bytes()).into_owned());

    if let Some(ref value_name) = value_name {
      if let Some(expr_extern_type) = get_type_id::<ExternType>(expr_type) {
        if let Some(prop) = lookup_extern_type_prop(expr_extern_type, value_name) {
          if ctx == ValueContext::LValue
            && let Some(write_ty) = prop.write_ty
          {
            return write_ty;
          }

          return prop.type_deprecated();
        }

        if let Some(ref indexer) = expr_extern_type.indexer {
          self.unify_type_id_type_id_scope_ptr_location(
            self.string_type,
            indexer.index_type,
            scope,
            &index_loc,
          );
          return indexer.index_result_type;
        }

        self.report_error_type_error(&TypeError::type_error_location_type_error_data(
          expr.base.base.location,
          TypeErrorData::UnknownProperty(UnknownProperty {
            table: expr_type,
            key: value_name.clone(),
          }),
        ));
        return self.error_recovery_type_scope_ptr(scope);
      } else if get_type_id::<IntersectionType>(expr_type).is_some() {
        let name = value_name;

        if let Some(ty) = self.get_index_type_from_type(
          scope.clone(),
          expr_type,
          name,
          &expr.base.base.location,
          false,
        ) {
          return ty;
        }

        if is_table_intersection(expr_type) {
          self.report_error_type_error(&TypeError::type_error_location_type_error_data(
            expr.base.base.location,
            TypeErrorData::CannotExtendTable(CannotExtendTable {
              table_type: expr_type,
              context: Context::Property,
              prop: name.clone(),
            }),
          ));
          return self.error_recovery_type_scope_ptr(scope);
        }
      }
    } else {
      if let Some(expr_extern_type) = get_type_id::<ExternType>(expr_type)
        && let Some(ref indexer) = expr_extern_type.indexer
      {
        self.unify_type_id_type_id_scope_ptr_location(
          index_type,
          indexer.index_type,
          scope,
          &index_loc,
        );
        return indexer.index_result_type;
      }

      if get_type_id::<ExternType>(expr_type).is_some() {
        if self.is_nonstrict_mode() {
          return self.unknown_type;
        }
        self.report_error_type_error(&TypeError::type_error_location_type_error_data(
          expr.base.base.location,
          TypeErrorData::DynamicPropertyLookupOnExternTypesUnsafe(
            DynamicPropertyLookupOnExternTypesUnsafe { ty: expr_type },
          ),
        ));
        return self.error_recovery_type_scope_ptr(scope);
      }
    }

    let mut table_types: Vec<&'static mut TableType> = Vec::new();
    let mut is_union = true;

    if let Some(expr_union) = get_type_id::<UnionType>(expr_type) {
      table_types.reserve(expr_union.options.len());

      for option in &expr_union.options {
        let Some(option_table) = get_mutable_table_type(*option) else {
          self.report_error_type_error(&TypeError::type_error_location_type_error_data(
            expr_ref.base.location,
            TypeErrorData::NotATable(NotATable { ty: expr_type }),
          ));
          return self.error_recovery_type_scope_ptr(scope);
        };
        table_types.push(option_table);
      }
    } else if let Some(expr_intersection) = get_type_id::<IntersectionType>(expr_type) {
      table_types.reserve(expr_intersection.parts.len());
      is_union = false;

      for part in &expr_intersection.parts {
        let Some(part_table) = get_mutable_table_type(*part) else {
          self.report_error_type_error(&TypeError::type_error_location_type_error_data(
            expr_ref.base.location,
            TypeErrorData::NotATable(NotATable { ty: expr_type }),
          ));
          return self.error_recovery_type_scope_ptr(scope);
        };
        table_types.push(part_table);
      }
    } else {
      let Some(expr_table) = get_mutable_table_type(expr_type) else {
        self.report_error_type_error(&TypeError::type_error_location_type_error_data(
          expr_ref.base.location,
          TypeErrorData::NotATable(NotATable { ty: expr_type }),
        ));
        return self.error_recovery_type_scope_ptr(scope);
      };
      table_types.push(expr_table);
    }

    if let Some(ref key_name) = value_name {
      let mut property_types: DenseHashSet<TypeId> = DenseHashSet::new(TypeId::default());

      for table_ref in &mut table_types {
        let table_ref: &mut TableType = table_ref;
        if let Some(prop) = table_ref.props.get(key_name) {
          property_types.insert(prop.type_deprecated());
        } else if (ctx == ValueContext::LValue && table_ref.state == TableState::Unsealed)
          || table_ref.state == TableState::Free
        {
          let result_type = self.fresh_type_scope_ptr(scope.clone());
          let property = table_ref.props.entry(key_name.clone()).or_default();
          property.set_type(result_type);
          property.location = Some(index_loc);
          property_types.insert(result_type);
        }
      }

      if property_types.size() == 1
        && let Some(&ty) = property_types.iter().next()
      {
        return ty;
      }

      if !property_types.empty() {
        if is_union {
          let options_vec: Vec<TypeId> = property_types.iter().copied().collect();
          let options = reduce_union(&options_vec);
          if options.is_empty() {
            return self.never_type;
          }
          if options.len() == 1 {
            return options[0];
          }
          return self.add_type(&UnionType { options });
        }

        let parts_vec: Vec<TypeId> = property_types.iter().copied().collect();
        return self.add_type(&IntersectionType { parts: parts_vec });
      }
    }

    let mut result_types: DenseHashSet<TypeId> = DenseHashSet::new(TypeId::default());

    for table_ref in &mut table_types {
      let table_ref: &mut TableType = table_ref;
      if let Some(ref indexer) = table_ref.indexer {
        self.unify_type_id_type_id_scope_ptr_location(
          index_type,
          indexer.index_type,
          scope,
          &index_loc,
        );
        result_types.insert(indexer.index_result_type);
      } else if (ctx == ValueContext::LValue && table_ref.state == TableState::Unsealed)
        || table_ref.state == TableState::Free
      {
        let level = table_ref.level;
        let indexer_type = self.fresh_type_type_level(level);
        self.unify_type_id_type_id_scope_ptr_location(
          index_type,
          indexer_type,
          scope,
          &expr.base.base.location,
        );
        let index_result_type = self.fresh_type_type_level(level);

        let index_type_any = self.any_if_nonstrict(indexer_type);
        let index_result_type_any = self.any_if_nonstrict(index_result_type);
        table_ref.indexer = Some(TableIndexer {
          index_type: index_type_any,
          index_result_type: index_result_type_any,
          is_read_only: false,
        });
        result_types.insert(index_result_type);
      } else {
        if is_union {
          return self.any_type;
        }
        result_types.insert(self.any_type);
      }
    }

    if result_types.size() == 1
      && let Some(&ty) = result_types.iter().next()
    {
      return ty;
    }

    if is_union {
      let options_vec: Vec<TypeId> = result_types.iter().copied().collect();
      let options = reduce_union(&options_vec);
      if options.is_empty() {
        return self.never_type;
      }
      if options.len() == 1 {
        return options[0];
      }
      return self.add_type(&UnionType { options });
    }

    let parts_vec: Vec<TypeId> = result_types.iter().copied().collect();
    self.add_type(&IntersectionType { parts: parts_vec })
  }
}
