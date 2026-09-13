use alloc::{string::String, sync::Arc, vec::Vec};

use crate::records::{scope::Scope, to_string_name_map::ToStringNameMap};

#[derive(Debug, Clone)]
pub struct ToStringOptions {
  pub exhaustive: bool,
  pub use_line_breaks: bool,
  pub function_type_arguments: bool,
  pub hide_table_kind: bool,
  pub hide_named_function_type_parameters: bool,
  pub hide_function_self_argument: bool,
  pub hide_table_alias_expansions: bool,
  pub use_question_marks: bool,
  pub ignore_synthetic_name: bool,
  pub max_table_length: usize,
  pub max_type_length: usize,
  pub composite_types_single_line_limit: usize,
  pub name_map: ToStringNameMap,
  pub scope: Option<Arc<Scope>>,
  pub named_function_override_arg_names: Vec<String>,
}
