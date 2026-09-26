use ulua_ast::{
  records::{ast_stat_local::AstStatLocal, position::Position},
  rtti::ast_node_try_as_ptr,
};

use crate::{
  functions::find_ast_ancestry_of_position_ast_query::find_ast_ancestry_of_position_source_module_position_bool,
  records::{binding::Binding, source_module::SourceModule},
};

pub(crate) fn find_binding_local_statement(
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

  nodes.into_iter().rev().find_map(|node| {
    // Safety: node 来自 ancestry 向量，指向 parser arena 存活节点；
    // ast_node_try_as_ptr 按 class_index 判型，命中即合法下转。
    let stat_local = unsafe { ast_node_try_as_ptr::<AstStatLocal>(node) }?;
    Some((stat_local as *const AstStatLocal).cast_mut())
  })
}
