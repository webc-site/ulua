use alloc::vec::Vec;

use ulua_ast::{
  records::{
    allocator::Allocator, ast_local::AstLocal, ast_name_table::AstNameTable,
    ast_stat_function::AstStatFunction, node_handle::OptNode, parse_options::ParseOptions,
    parser::Parser,
  },
  rtti::AstNodeViewMut,
};
use ulua_compiler::functions::model_cost_cost_model::model_cost_ast_node_ast_local_usize;

pub fn model_function(source: &str) -> u64 {
  // Box 钉堆：AstNameTable/Parser 捕获宿主地址，宿主移动即悬垂。
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);
  let result = Parser::parse(source, &mut names, &mut allocator, ParseOptions::default());
  assert!(
    result.errors.is_empty(),
    "unexpected parse error(s): {:?}",
    result.errors
  );
  assert!(!result.root.is_null());

  // `body[0]` 已是 Node 句柄（root 经 OptNode 判空物化为 fixture arena 存活根），
  // Node 为 Copy 值，下转走句柄上生命周期正确的 `try_as`，零 unsafe。
  let root = OptNode::from_ptr(result.root);
  let first = root.get().expect("上方已断言根块非空").body[0];
  let func = first
    .try_as::<AstStatFunction>()
    .expect("首条语句应为函数声明");
  let mut function = func.func;

  // `func.func` 已句柄化为 Node：Deref 即安全只读视图，vars 抽取不再需要 unsafe。
  let vars: Vec<*mut AstLocal> = function.args.iter_nodes().map(|n| n.as_ptr()).collect();

  // 独占视图经 `Node::get_mut` + `AstNodeViewMut::as_ast_node_mut` 安全上转
  // （repr(C) 基址重合），model_cost 按只读遍历 AST 计成本，不写不逃逸。
  model_cost_ast_node_ast_local_usize(function.body.get_mut().as_ast_node_mut(), &vars)
}
