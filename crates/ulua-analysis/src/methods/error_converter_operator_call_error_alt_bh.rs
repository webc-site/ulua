use alloc::string::String;

use crate::records::{
  error_converter::ErrorConverter,
  instantiate_generics_on_non_function::InstantiateGenericsOnNonFunction,
};

impl ErrorConverter {
  pub fn operator_call_12(&self, e: &InstantiateGenericsOnNonFunction) -> String {
    match e.interesting_edge_case {
      InstantiateGenericsOnNonFunction::NONE => {
        String::from("Cannot instantiate type parameters on something without type parameters.")
      }
      InstantiateGenericsOnNonFunction::METATABLE_CALL => {
        // `__call` is complicated because `f<<T>>()` is interpreted as `f<<T>>` as its own expression that is then called.
        // This is so that you can write code like `local f2 = f<<number>>`, and then call `f2()`.
        // With metatables, it's not so obvious what this would result in.
        String::from(
          "Luau does not currently support explicitly instantiating a table with a `__call` metamethod. You may be able to work around this by creating a function that calls the table, and using that instead.",
        )
      }
      InstantiateGenericsOnNonFunction::INTERSECTION => String::from(
        "Luau does not currently support explicitly instantiating an overloaded function type.",
      ),
    }
  }
}
