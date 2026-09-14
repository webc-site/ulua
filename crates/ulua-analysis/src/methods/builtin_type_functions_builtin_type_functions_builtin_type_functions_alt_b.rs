//! C++ `BuiltinTypeFunctions::BuiltinTypeFunctions()` — the real default ctor
//! (BuiltinTypeFunctions.cpp:2555-2585). Brace-initializes all 28 `TypeFunction`
//! members `{name, reducerFn[, canReduceGenerics]}`. Each reducer is stored as a
//! plain `fn` item in the `reducer` field (the project's MagicFunction-style
//! fn-pointer wiring; no transmute / no erased cast).
use alloc::string::ToString;

// The 28 reducer fns. 25 already existed; `unm`/`refine` were ported in this
// cluster; `user_defined_type_function` is the VM-bridge reducer owned by another
// agent (now ported, signature-compatible with `ReducerFunction`).
use crate::functions::add_type_function::add_type_function;
use crate::{
  functions::{
    and_type_function::and_type_function, concat_type_function::concat_type_function,
    div_type_function::div_type_function, getmetatable_type_function::getmetatable_type_function,
    idiv_type_function::idiv_type_function, index_type_function::index_type_function,
    intersect_type_function::intersect_type_function, keyof_type_function::keyof_type_function,
    le_type_function::le_type_function, len_type_function::len_type_function,
    lt_type_function::lt_type_function, mod_type_function::mod_type_function,
    mul_type_function::mul_type_function, not_type_function::not_type_function,
    objectof_type_function::objectof_type_function, or_type_function::or_type_function,
    pow_type_function::pow_type_function, rawget_type_function::rawget_type_function,
    rawkeyof_type_function::rawkeyof_type_function, refine_type_function::refine_type_function,
    setmetatable_type_function::setmetatable_type_function,
    singleton_type_function::singleton_type_function, sub_type_function::sub_type_function,
    union_type_function::union_type_function, unm_type_function::unm_type_function,
    user_defined_type_function::user_defined_type_function,
    weakoptional_type_func::weakoptional_type_func,
  },
  records::{builtin_type_functions::BuiltinTypeFunctions, type_function::TypeFunction},
  type_aliases::reducer_function::ReducerFunction,
};
impl BuiltinTypeFunctions {
  /// C++ `BuiltinTypeFunctions::BuiltinTypeFunctions()`.
  pub fn new() -> Self {
    // Helper to build a `TypeFunction { name, reducer, can_reduce_generics }`.
    fn tf(name: &str, reducer: ReducerFunction) -> TypeFunction {
      TypeFunction {
        name: name.to_string(),
        reducer,
        can_reduce_generics: false,
      }
    }
    fn tf_gen(name: &str, reducer: ReducerFunction) -> TypeFunction {
      TypeFunction {
        name: name.to_string(),
        reducer,
        can_reduce_generics: true,
      }
    }

    BuiltinTypeFunctions {
      user_func: tf("user", user_defined_type_function),

      not_func: tf("not", not_type_function),
      len_func: tf("len", len_type_function),
      unm_func: tf("unm", unm_type_function),

      add_func: tf("add", add_type_function),
      sub_func: tf("sub", sub_type_function),
      mul_func: tf("mul", mul_type_function),
      div_func: tf("div", div_type_function),
      idiv_func: tf("idiv", idiv_type_function),
      pow_func: tf("pow", pow_type_function),
      mod_func: tf("mod", mod_type_function),

      concat_func: tf("concat", concat_type_function),

      and_func: tf_gen("and", and_type_function),
      or_func: tf_gen("or", or_type_function),

      lt_func: tf("lt", lt_type_function),
      le_func: tf("le", le_type_function),

      refine_func: tf_gen("refine", refine_type_function),
      singleton_func: tf("singleton", singleton_type_function),
      union_func: tf("union", union_type_function),
      intersect_func: tf("intersect", intersect_type_function),

      keyof_func: tf("keyof", keyof_type_function),
      rawkeyof_func: tf("rawkeyof", rawkeyof_type_function),
      index_func: tf("index", index_type_function),
      rawget_func: tf("rawget", rawget_type_function),

      setmetatable_func: tf("setmetatable", setmetatable_type_function),
      getmetatable_func: tf("getmetatable", getmetatable_type_function),

      objectof_func: tf("objectof", objectof_type_function),

      weakoptional_func: tf("weakoptional", weakoptional_type_func),
    }
  }
}

impl Default for BuiltinTypeFunctions {
  fn default() -> Self {
    Self::new()
  }
}
