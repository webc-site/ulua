use alloc::string::String;
use core::ffi::CStr;

use ulua_ast::records::ast_expr_index_name::AstExprIndexName;

use crate::{
  enums::{table_state::TableState, value_context::ValueContext},
  functions::{
    get_mutable_table_type::get_mutable_table_type, get_type_alt_j::get_type_id,
    is_table_intersection::is_table_intersection, lookup_extern_type_prop::lookup_extern_type_prop,
  },
  records::{
    any_type::AnyType,
    cannot_extend_table::{CannotExtendTable, Context},
    extern_type::ExternType,
    generic_error::GenericError,
    intersection_type::IntersectionType,
    never_type::NeverType,
    not_a_table::NotATable,
    property_type::Property,
    type_checker::TypeChecker,
    type_error::TypeError,
    unknown_property::UnknownProperty,
  },
  type_aliases::{
    error_type::ErrorType, name_type::Name, scope_ptr_type::ScopePtr,
    type_error_data::TypeErrorData, type_id::TypeId,
  },
};
impl TypeChecker {
  pub fn check_l_value_binding_scope_ptr_ast_expr_index_name_value_context(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprIndexName,
    ctx: ValueContext,
  ) -> TypeId {
    let mut lhs = self
      .check_expr_scope_ptr_ast_expr_optional_type_id_bool(
        scope,
        // SAFETY: expr.expr 指向 AST arena 节点（parser 保证非空）。
        unsafe { &*expr.expr },
        None,
        false,
      )
      .r#type;

    if get_type_id::<ErrorType>(lhs).is_some() || get_type_id::<AnyType>(lhs).is_some() {
      return lhs;
    }

    if get_type_id::<NeverType>(lhs).is_some() {
      return self.unknown_type;
    }

    self.tablify(lhs);

    // SAFETY: index.value 为 NUL 结尾 C 字符串（AST arena 持有）。
    let name: Name = unsafe {
      CStr::from_ptr(expr.index.value)
        .to_string_lossy()
        .into_owned()
    };

    // SAFETY: expr.expr 指向 AST arena 节点。
    lhs = self.strip_from_nil_and_report(lhs, unsafe { &(*expr.expr).base.location });

    if let Some(lhs_table_ref) = get_mutable_table_type(lhs) {
      if let Some(prop) = lhs_table_ref.props.get(&name) {
        return prop.type_deprecated();
      } else if (ctx == ValueContext::LValue && lhs_table_ref.state == TableState::Unsealed)
        || lhs_table_ref.state == TableState::Free
      {
        let the_type = self.fresh_type_scope_ptr(scope.clone());
        let property = lhs_table_ref
          .props
          .entry(name)
          .or_insert_with(Property::default);
        property.set_type(the_type);
        property.location = Some(expr.index_location);
        return the_type;
      } else if let Some(indexer) = lhs_table_ref.indexer {
        let ok = self.unify_type_id_type_id_scope_ptr_location(
          self.string_type,
          indexer.index_type,
          scope,
          &expr.base.base.location,
        );
        let mut ret_type = indexer.index_result_type;
        if !ok {
          self.report_error_location_type_error_data(
            &expr.base.base.location,
            TypeErrorData::UnknownProperty(UnknownProperty {
              table: lhs,
              key: name,
            }),
          );
          ret_type = self.error_recovery_type_type_id(ret_type);
        }
        return ret_type;
      } else if lhs_table_ref.state == TableState::Sealed {
        self.report_error_type_error(&TypeError::type_error_location_type_error_data(
          expr.base.base.location,
          TypeErrorData::CannotExtendTable(CannotExtendTable {
            table_type: lhs,
            context: Context::Property,
            prop: name,
          }),
        ));
        return self.error_recovery_type_scope_ptr(scope);
      } else {
        self.report_error_type_error(&TypeError::type_error_location_type_error_data(
          expr.base.base.location,
          TypeErrorData::GenericError(GenericError::new(String::from(
            "Internal error: generic tables are not lvalues",
          ))),
        ));
        return self.error_recovery_type_scope_ptr(scope);
      }
    } else if let Some(lhs_extern_type) = get_type_id::<ExternType>(lhs) {
      if let Some(prop) = lookup_extern_type_prop(lhs_extern_type, &name) {
        if ctx == ValueContext::LValue
          && let Some(write_ty) = prop.write_ty
        {
          return write_ty;
        }

        return prop.type_deprecated();
      }

      if let Some(indexer) = lhs_extern_type.indexer {
        let ok = self.unify_type_id_type_id_scope_ptr_location(
          self.string_type,
          indexer.index_type,
          scope,
          &expr.base.base.location,
        );
        if ok {
          return indexer.index_result_type;
        }
      }

      self.report_error_type_error(&TypeError::type_error_location_type_error_data(
        expr.base.base.location,
        TypeErrorData::UnknownProperty(UnknownProperty {
          table: lhs,
          key: name,
        }),
      ));
      return self.error_recovery_type_scope_ptr(scope);
    } else if get_type_id::<IntersectionType>(lhs).is_some() {
      if let Some(ty) =
        self.get_index_type_from_type(scope.clone(), lhs, &name, &expr.base.base.location, false)
      {
        return ty;
      }

      // If intersection has a table part, report that it cannot be extended just as a sealed table
      if is_table_intersection(lhs) {
        self.report_error_type_error(&TypeError::type_error_location_type_error_data(
          expr.base.base.location,
          TypeErrorData::CannotExtendTable(CannotExtendTable {
            table_type: lhs,
            context: Context::Property,
            prop: name,
          }),
        ));
        return self.error_recovery_type_scope_ptr(scope);
      }
    }

    self.report_error_type_error(&TypeError::type_error_location_type_error_data(
      expr.base.base.location,
      TypeErrorData::NotATable(NotATable { ty: lhs }),
    ));
    self.error_recovery_type_scope_ptr(scope)
  }
}
