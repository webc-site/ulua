use alloc::{string::String, sync::Arc};

use crate::{
  enums::table_state::TableState,
  functions::{
    get_mutable_type::get_mutable_type_id, get_type_alt_j::get_type_id,
    has_unification_too_complex::has_unification_too_complex,
  },
  records::{
    any_type::AnyType, metatable_type::MetatableType, table_type::TableType,
    type_mismatch::TypeMismatch, unifier::Unifier,
  },
  type_aliases::{error_type::ErrorType, type_error_data::TypeErrorData, type_id::TypeId},
};
impl Unifier {
  pub fn unifier_try_unify_with_metatable(
    &mut self,
    sub_ty: TypeId,
    super_ty: TypeId,
    reversed: bool,
  ) {
    // C++ Unifier.cpp:2245 tryUnifyWithMetatable
    let Some(super_metatable) = get_type_id::<MetatableType>(super_ty) else {
      self.ice_string("tryUnifyMetatable invoked with non-metatable Type");
      return;
    };

    let wanted = if reversed { sub_ty } else { super_ty };
    let given = if reversed { super_ty } else { sub_ty };

    if let Some(sub_metatable) = get_mutable_type_id::<MetatableType>(sub_ty) {
      let mut inner_state = self.unifier_make_child_unifier();
      inner_state.try_unify_type_id_type_id_bool_bool_literal_properties(
        sub_metatable.table,
        super_metatable.table,
        false,
        false,
        None,
      );
      inner_state.try_unify_type_id_type_id_bool_bool_literal_properties(
        sub_metatable.metatable,
        super_metatable.metatable,
        false,
        false,
        None,
      );

      if let Some(e) = has_unification_too_complex(&inner_state.errors) {
        self.report_error_type_error(e);
      } else if !inner_state.errors.is_empty() {
        let context = self.unifier_mismatch_context();
        self.report_error_location_type_error_data(
          self.location,
          TypeErrorData::TypeMismatch(TypeMismatch {
            wanted_type: wanted,
            given_type: given,
            reason: String::new(),
            error: Some(Arc::new(inner_state.errors[0].clone())),
            context,
          }),
        );
      }

      self.log.concat(inner_state.log);
      self.failure |= inner_state.failure;
    } else if let Some(sub_table) = get_mutable_type_id::<TableType>(sub_ty) {
      match sub_table.state {
        TableState::Free => {
          self.try_unify_type_id_type_id_bool_bool_literal_properties(
            sub_ty,
            super_metatable.table,
            false,
            false,
            None,
          );
          self.log.bind_table(sub_ty, Some(super_ty));
        }
        // 已知 sealed/unsealed/generic 表的形状，无法再挂 metatable
        TableState::Sealed | TableState::Unsealed | TableState::Generic => {
          let context = self.unifier_mismatch_context();
          self.report_error_location_type_error_data(
            self.location,
            TypeErrorData::TypeMismatch(TypeMismatch {
              wanted_type: wanted,
              given_type: given,
              reason: String::new(),
              error: None,
              context,
            }),
          );
        }
      }
    } else if get_mutable_type_id::<AnyType>(sub_ty).is_some()
      || get_mutable_type_id::<ErrorType>(sub_ty).is_some()
    {
    } else {
      let context = self.unifier_mismatch_context();
      self.report_error_location_type_error_data(
        self.location,
        TypeErrorData::TypeMismatch(TypeMismatch {
          wanted_type: wanted,
          given_type: given,
          reason: String::new(),
          error: None,
          context,
        }),
      );
    }
  }
}
