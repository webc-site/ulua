use alloc::vec::Vec;

use ulua_ast::{
  records::{
    allocator::Allocator, ast_local::AstLocal, ast_name_table::AstNameTable,
    ast_stat_function::AstStatFunction, parse_options::ParseOptions, parser::Parser,
  },
  rtti::{AstNodePtr, ast_node_try_as_ptr},
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

  let first = unsafe { (&*result.root).body[0].as_ptr() };
  // 门面一步下转+判型（原 `ast_node_as + is_null 断言 + &*` 三步样板）：
  // Safety: first 是 body 数组内存活语句指针（夹具 arena 只读存活至本帧末）。
  let func: &'static AstStatFunction =
    unsafe { ast_node_try_as_ptr::<AstStatFunction>(first) }.expect("首条语句应为函数声明");
  let function = func.func;

  // `func.func` 已句柄化为 Node：Deref 即安全只读视图，vars 抽取不再需要 unsafe。
  let vars: Vec<*mut AstLocal> = function.args.iter_nodes().map(|n| n.as_ptr()).collect();

  // Safety: function 由 parser 布线为非空 AstExprFunction 指针（AST 子指针不变式），
  // body 为非空块句柄；基类上转收口在 `AstNodePtr::as_ast_node` 门面（repr(C)
  // 基址重合），model_cost 按只读遍历 AST 计成本，不写不逃逸。
  unsafe { model_cost_ast_node_ast_local_usize(&mut *function.body.as_ast_node(), &vars) }
}
