use alloc::{boxed::Box, string::String, sync::Arc};
use core::ptr::NonNull;

use ulua_analysis::{
  enums::polarity::Polarity,
  functions::{freeze::freeze, unfreeze::unfreeze},
  records::{
    generic_type::GenericType, generic_type_definition::GenericTypeDefinition, scope::Scope,
    type_fun::TypeFun, type_function::TypeFunction,
    type_function_instance_type::TypeFunctionInstanceType,
  },
};

use crate::{
  functions::type_function_fixture_swap_reducer::type_function_fixture_swap_reducer,
  records::{fixture::Fixture, type_function_fixture::TypeFunctionFixture},
};

impl TypeFunctionFixture {
  pub fn new() -> Self {
    let mut fixture = TypeFunctionFixture {
      base: Fixture::fixture_bool(false),
      swap_function: Box::new(TypeFunction {
        name: String::from("Swap"),
        reducer: type_function_fixture_swap_reducer,
        can_reduce_generics: false,
      }),
    };

    let swap_function = NonNull::from(&*fixture.swap_function);
    let frontend = fixture.base.get_frontend();

    unfreeze(frontend.globals.global_types_mut());

    let generic_t = {
      let arena = frontend.globals.global_types_mut();
      let t = arena.add_type(GenericType::generic_type_name_polarity(
        &String::from("T"),
        Polarity::Negative,
      ));
      let result_type = arena.add_type(
                TypeFunctionInstanceType::type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id(
                    swap_function,
                    alloc::vec![t],
                    alloc::vec![],
                ),
            );
      (GenericTypeDefinition::new(t), result_type)
    };

    let global_scope = frontend.globals.global_scope();
    let global_scope_ptr = Arc::as_ptr(&global_scope) as *mut Scope;
    unsafe {
      (*global_scope_ptr).exported_type_bindings.insert(
        String::from("Swap"),
        TypeFun::type_fun_vector_generic_type_definition_type_id_optional_location(
          alloc::vec![generic_t.0],
          generic_t.1,
          None,
        ),
      );
    }

    freeze(frontend.globals.global_types_mut());
    fixture
  }
}

impl Default for TypeFunctionFixture {
  fn default() -> Self {
    Self::new()
  }
}
