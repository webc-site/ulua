use ulua_ast::records::{
  allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions, parser::Parser,
};
use ulua_reduce_cli::{Block, Stat, pruned_span};

/// 三条语句的块：`body[0..3]` 可被任意半开区间取用。返回的 arena 夹具
/// 与句柄同作用域存活（Box 钉堆：`AstNameTable` 捕获 `*mut Allocator`）。
fn block() -> (Box<Allocator>, Box<AstNameTable>, Block) {
  let mut allocator = Box::new(Allocator::new());
  let mut names = Box::new(AstNameTable::new(&mut allocator));

  let result = Parser::parse(
    "local a = 1\nlocal b = 2\nlocal c = 3\n",
    &mut names,
    &mut allocator,
    ParseOptions::default(),
  );
  assert!(result.errors.is_empty());
  let root = Block::new(result.root).expect("解析成功，根块非空");

  (allocator, names, root)
}

/// cpp `prunedSpan`：按顺序拼接两个半开区间（"hokey restriction" 是 span1 先于 span2）
#[test]
fn concatenates_two_half_open_ranges_in_order() {
  let (_arena, _names, root) = block();
  let block = root.get();
  let body = block.body.as_slice();

  let picked = pruned_span(block, (0, 1), (2, 3));
  assert_eq!(
    picked,
    vec![Stat::from_ref(body[0].get()), Stat::from_ref(body[2].get())]
  );
}

/// 空区间对 → 空结果（`deleteChildStatements` 用它表达「什么都不保留」）
#[test]
fn empty_spans_pick_nothing() {
  let (_arena, _names, root) = block();

  assert_eq!(pruned_span(root.get(), (0, 0), (3, 3)), Vec::new());
}

/// 整段区间 → 全部语句，顺序不变
#[test]
fn full_range_keeps_every_statement() {
  let (_arena, _names, root) = block();
  let block = root.get();

  let all: Vec<Stat> = block.body.iter().map(Stat::from_ref).collect();
  assert_eq!(pruned_span(block, (0, 3), (3, 3)), all);
}
