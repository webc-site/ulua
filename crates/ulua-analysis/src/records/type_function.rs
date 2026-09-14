use alloc::string::String;

use crate::type_aliases::reducer_function::ReducerFunction;
#[derive(Debug, Clone)]
pub struct TypeFunction {
  pub name: String,
  pub reducer: ReducerFunction,
  pub can_reduce_generics: bool,
}
