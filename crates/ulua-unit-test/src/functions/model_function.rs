use ulua_ast::{
  records::{
    allocator::Allocator, ast_name_table::AstNameTable, ast_node::AstNode,
    ast_stat_function::AstStatFunction, parse_options::ParseOptions, parser::Parser,
  },
  rtti::ast_node_as,
};
use ulua_compiler::functions::model_cost_cost_model_alt_b::model_cost_ast_node_ast_local_usize;

pub fn model_function(source: &str) -> u64 {
  let mut allocator = Allocator::new();
  let mut names = AstNameTable::new(&mut allocator);
  let result = Parser::parse(
    source,
    source.len(),
    &mut names,
    &mut allocator,
    ParseOptions::default(),
  );
  assert!(
    result.errors.is_empty(),
    "unexpected parse error(s): {:?}",
    result.errors
  );
  assert!(!result.root.is_null());

  let first = unsafe { *(*result.root).body.data };
  let func = unsafe { ast_node_as::<AstStatFunction>(first as *mut AstNode) };
  assert!(!func.is_null());

  let function = unsafe { (*func).func };
  unsafe {
    model_cost_ast_node_ast_local_usize(
      (*function).body as *mut AstNode,
      (*function).args.data,
      (*function).args.size,
    )
  }
}
