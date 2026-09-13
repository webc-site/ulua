use ulua_ast::{
  records::{ast_stat_local::AstStatLocal, position::Position},
  rtti::ast_node_as,
};

use crate::{
  functions::find_ast_ancestry_of_position_ast_query::find_ast_ancestry_of_position_source_module_position_bool,
  records::{binding::Binding, source_module::SourceModule},
};

pub fn find_binding_local_statement(
  source: &SourceModule,
  binding: &Binding,
) -> Option<*mut AstStatLocal> {
  if binding.location.begin == Position::new(0, 0) && binding.location.end == Position::new(0, 0) {
    return None;
  }

  let nodes = find_ast_ancestry_of_position_source_module_position_bool(
    source,
    binding.location.begin,
    false,
  );

  let iter = nodes.iter().rev();
  for &node in iter {
    let stat_local = unsafe { ast_node_as::<AstStatLocal>(node) };
    if !stat_local.is_null() {
      return Some(stat_local);
    }
  }

  None
}
