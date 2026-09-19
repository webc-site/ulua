//! Source: `Analysis/include/Luau/BuiltinTypeFunctions.h`

use crate::records::type_function::TypeFunction;

// 27 builtin type-function entries (BuiltinTypeFunctions.h:9-55). The ctor
// and addToScope are their own method nodes; C++ deletes copy/assign.
#[derive(Debug)]
pub struct BuiltinTypeFunctions {
  pub user_func: TypeFunction,

  pub not_func: TypeFunction,
  pub len_func: TypeFunction,
  pub unm_func: TypeFunction,

  pub add_func: TypeFunction,
  pub sub_func: TypeFunction,
  pub mul_func: TypeFunction,
  pub div_func: TypeFunction,
  pub idiv_func: TypeFunction,
  pub pow_func: TypeFunction,
  pub mod_func: TypeFunction,

  pub concat_func: TypeFunction,

  pub and_func: TypeFunction,
  pub or_func: TypeFunction,

  pub lt_func: TypeFunction,
  pub le_func: TypeFunction,

  pub refine_func: TypeFunction,
  pub singleton_func: TypeFunction,
  pub union_func: TypeFunction,
  pub intersect_func: TypeFunction,

  pub keyof_func: TypeFunction,
  pub rawkeyof_func: TypeFunction,
  pub index_func: TypeFunction,
  pub rawget_func: TypeFunction,

  pub setmetatable_func: TypeFunction,
  pub getmetatable_func: TypeFunction,

  pub objectof_func: TypeFunction,

  pub weakoptional_func: TypeFunction,
}
