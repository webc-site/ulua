use alloc::{string::String, sync::Arc};

use crate::{
  enums::polarity::Polarity,
  functions::{freeze::freeze, unfreeze::unfreeze},
  records::{
    generic_type::GenericType, generic_type_definition::GenericTypeDefinition,
    global_types::GlobalTypes, metatable_type::MetatableType, negation_type::NegationType,
    scope::Scope, type_fun::TypeFun,
  },
};

impl GlobalTypes {
  pub fn register_hidden_test_types(&mut self) {
    unfreeze(&mut self.global_types);

    let t = self
      .global_types
      .add_type(GenericType::generic_type_name_polarity(
        &String::from("T"),
        Polarity::Mixed,
      ));
    let generic_t = GenericTypeDefinition {
      ty: t,
      default_value: None,
    };

    let u = self
      .global_types
      .add_type(GenericType::generic_type_name_polarity(
        &String::from("U"),
        Polarity::Mixed,
      ));
    let generic_u = GenericTypeDefinition {
      ty: u,
      default_value: None,
    };

    let not_type = self.global_types.add_type(NegationType::new(t));
    let mt_type = self.global_types.add_type(MetatableType {
      table: t,
      metatable: u,
      synthetic_name: None,
    });

    let (function_type, extern_type, error_type, table_type) = unsafe {
      let builtins = self.builtin_types.as_ref();
      (
        builtins.function_type,
        builtins.extern_type,
        builtins.error_type,
        builtins.table_type,
      )
    };

    let scope = Arc::as_ptr(&self.global_scope) as *mut Scope;
    unsafe {
      (*scope).exported_type_bindings.insert(
        String::from("Not"),
        TypeFun::type_fun_vector_generic_type_definition_type_id_optional_location(
          vec![generic_t],
          not_type,
          None,
        ),
      );
      (*scope).exported_type_bindings.insert(
        String::from("Mt"),
        TypeFun::type_fun_vector_generic_type_definition_type_id_optional_location(
          vec![generic_t, generic_u],
          mt_type,
          None,
        ),
      );
      (*scope).exported_type_bindings.insert(
        String::from("fun"),
        TypeFun::type_fun_type_id(function_type),
      );
      (*scope)
        .exported_type_bindings
        .insert(String::from("cls"), TypeFun::type_fun_type_id(extern_type));
      (*scope)
        .exported_type_bindings
        .insert(String::from("err"), TypeFun::type_fun_type_id(error_type));
      (*scope)
        .exported_type_bindings
        .insert(String::from("tbl"), TypeFun::type_fun_type_id(table_type));
    }

    freeze(&mut self.global_types);
  }
}
