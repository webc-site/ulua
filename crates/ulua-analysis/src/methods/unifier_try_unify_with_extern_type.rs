use alloc::{format, string::String};
use core::{
  mem::swap,
  ptr::{eq, null},
};

use crate::{
  enums::{table_state::TableState, variance::Variance},
  functions::{
    get_mutable_type::get_mutable_type_id, get_type_alt_j::get_type_id,
    is_subclass_type::is_subclass_extern_type_extern_type,
    lookup_extern_type_prop::lookup_extern_type_prop,
  },
  records::{
    extern_type::ExternType, generic_error::GenericError, table_type::TableType,
    type_mismatch::TypeMismatch, unifier::Unifier, unknown_property::UnknownProperty,
  },
  type_aliases::{type_error_data::TypeErrorData, type_id::TypeId},
};
impl Unifier {
  pub fn unifier_try_unify_with_extern_type(
    &mut self,
    mut sub_ty: TypeId,
    mut super_ty: TypeId,
    reversed: bool,
  ) {
    // C++ Unifier.cpp:2303 tryUnifyWithExternType：reversed 时交换 sub/super
    if reversed {
      swap(&mut super_ty, &mut sub_ty);
    }

    let Some(super_extern_type) = get_type_id::<ExternType>(super_ty) else {
      self.ice_string("tryUnifyExternType invoked with non-class Type");
      return;
    };

    if let Some(sub_extern_type) = get_type_id::<ExternType>(sub_ty) {
      match self.variance {
        Variance::Covariant => {
          if !is_subclass_extern_type_extern_type(sub_extern_type, super_extern_type) {
            self.report_extern_type_mismatch(sub_ty, super_ty, reversed);
          }
        }
        Variance::Invariant => {
          // C++ 指针比较 subExternType != superExternType
          if !eq(sub_extern_type, super_extern_type) {
            self.report_extern_type_mismatch(sub_ty, super_ty, reversed);
          }
        }
      }
    } else if let Some(sub_table) = get_mutable_type_id::<TableType>(sub_ty) {
      // free 表形状未知，可被约束为精确的 extern 类型；非 free 表则失败
      if sub_table.state != TableState::Free {
        self.report_extern_type_mismatch(sub_ty, super_ty, reversed);
        return;
      }

      let mut ok = true;

      for (prop_name, prop) in sub_table.props.iter() {
        let class_prop = lookup_extern_type_prop(super_extern_type, prop_name);
        if let Some(class_prop) = class_prop {
          let mut inner_state = self.unifier_make_child_unifier();
          // C++ classProp->type_DEPRECATED(): readTy 优先，否则 writeTy
          let class_prop_ty = class_prop.read_ty.or(class_prop.write_ty).unwrap_or(null());
          let prop_ty = prop.read_ty.or(prop.write_ty).unwrap_or(null());
          inner_state.try_unify_type_id_type_id_bool_bool_literal_properties(
            class_prop_ty,
            prop_ty,
            false,
            false,
            None,
          );

          self.check_child_unifier_type_mismatch_error_vec_string_type_id_type_id(
            &inner_state.errors,
            prop_name,
            if reversed { sub_ty } else { super_ty },
            if reversed { super_ty } else { sub_ty },
          );

          if inner_state.errors.is_empty() {
            self.log.concat(inner_state.log);
            self.failure |= inner_state.failure;
          } else {
            ok = false;
          }
        } else {
          ok = false;
          self.report_error_location_type_error_data(
            self.location,
            TypeErrorData::UnknownProperty(UnknownProperty {
              table: super_ty,
              key: prop_name.clone(),
            }),
          );
        }
      }

      if sub_table.indexer.is_some() {
        ok = false;
        self.report_error_location_type_error_data(
          self.location,
          TypeErrorData::GenericError(GenericError::new(format!(
            "Extern type {} does not have an indexer",
            super_extern_type.name
          ))),
        );
      }

      if !ok {
        return;
      }

      self.log.bind_table(sub_ty, Some(super_ty));
    } else {
      self.report_extern_type_mismatch(sub_ty, super_ty, reversed);
    }
  }

  fn report_extern_type_mismatch(&mut self, sub_ty: TypeId, super_ty: TypeId, reversed: bool) {
    let context = self.unifier_mismatch_context();
    self.report_error_location_type_error_data(
      self.location,
      TypeErrorData::TypeMismatch(TypeMismatch {
        wanted_type: if reversed { sub_ty } else { super_ty },
        given_type: if reversed { super_ty } else { sub_ty },
        reason: String::new(),
        error: None,
        context,
      }),
    );
  }
}
