use ulua_ast::{
  records::{ast_stat_local::AstStatLocal, position::Position},
  rtti::ast_node_try_cast_ptr,
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
    // ast_node_try_cast_ptr 按 class_index 判型，命中只交出类型化 NonNull
    // 指针（本函数纯指针传递，不解引用），不进借用系统、不锻造假 'static。
    let stat_local = unsafe { ast_node_try_cast_ptr::<AstStatLocal>(node) }?;
    Some(stat_local.as_ptr())
  })
}
