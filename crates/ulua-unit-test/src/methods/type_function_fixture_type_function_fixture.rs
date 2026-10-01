use alloc::{boxed::Box, string::String};
use core::ptr::NonNull;

use ulua_analysis::{
  enums::polarity::Polarity,
  functions::{freeze::freeze, unfreeze::unfreeze},
  records::{
    generic_type::GenericType, generic_type_definition::GenericTypeDefinition, type_fun::TypeFun,
    type_function::TypeFunction, type_function_instance_type::TypeFunctionInstanceType,
  },
};

use crate::{
  functions::{
    raw_handle::raw_handle, type_function_fixture_swap_reducer::type_function_fixture_swap_reducer,
  },
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
        "T",
        Polarity::Negative,
      ));
      let result_type = arena.add_type(
                TypeFunctionInstanceType::type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id(
                    swap_function,
                    alloc::vec![t],
                    alloc::vec![],
                ),
            );
      (GenericTypeDefinition::new(t, None), result_type)
    };

    // Safety: cpp `globalScope->exportedTypeBindings["Swap"] = TypeFun{..}`——
    // `global_scope()` 克隆的正是 `frontend.globals.globalScope` 指向的那个 `Scope`，
    // 单线程顺序写入，且此处到 `freeze` 之间没有其他借用活跃（前后 `unfreeze`/`freeze`
    // 都只借用 `globalTypes`）。句柄仅用于该次写入，不造 `&mut Scope`。
    let global_scope = frontend.globals.global_scope();
    let global_scope_ptr = raw_handle(&global_scope);
    // Safety: global_scope_ptr = raw_handle(&global_scope)：临时 Arc 克隆指向的 Scope 堆块地址，底层由 frontend.globals.globalScope 强引用保活（非空存活至 freeze）；cpp exportedTypeBindings["Swap"] 同款单写入，52-55 行注记论证无并发可变借用（其余借用只及 globalTypes 且在帧内结束）。
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
