use ulua_ast::records::parser::Parser;
use ulua_reduce_cli::{Block, Reducer, Stat};

/// 把源码解析进 `Reducer` 自己的 arena（等价 `run_from_source` 的前半段，
/// 不跑外部命令），返回顶层第一条语句的句柄。
fn parse(reducer: &mut Reducer, source: &str) -> Stat {
  reducer.parse_result = Parser::parse(
    source,
    &mut reducer.name_table,
    &mut reducer.allocator,
    reducer.parse_options.clone(),
  );
  assert!(
    reducer.parse_result.errors.is_empty(),
    "{:?}",
    reducer.parse_result.errors
  );

  let root = Block::new(reducer.parse_result.root).expect("解析成功后根块恒非空");
  reducer.root = Some(root);
  Stat::from_ref(root.get().body.get(0).expect("顶层至少一条语句"))
}

/// cpp `getNestedStats` 的 `AstStatBlock` 分支（`do ... end`）
#[test]
fn plain_block_yields_its_body() {
  let mut reducer = Reducer::new();
  let stat = parse(&mut reducer, "do\n  print(1)\n  print(2)\nend\n");

  assert_eq!(reducer.get_nested_stats(stat).len(), 2);
}

/// `if`/`elseif`/`else` 三段的语句被拉平到同一个列表（else 链递归展开）
#[test]
fn if_elseif_else_flattens_all_branches() {
  let mut reducer = Reducer::new();
  let stat = parse(
    &mut reducer,
    "if x then\n  print(1)\n  print(2)\nelseif y then\n  print(3)\nelse\n  print(4)\nend\n",
  );

  assert_eq!(reducer.get_nested_stats(stat).len(), 4);
}

/// 无 else 的 `if`：`elsebody` 为可空句柄的空槽（cpp 的 nullptr），
/// `get` 即折叠为 `None`，只贡献 then 分支（cpp 的判空跳过语义）
#[test]
fn if_without_else_contributes_only_then_body() {
  let mut reducer = Reducer::new();
  let stat = parse(&mut reducer, "if x then\n  print(1)\nend\n");

  assert_eq!(reducer.get_nested_stats(stat).len(), 1);
}

/// 单 body 的循环类语句各取 own body
#[test]
fn loop_bodies_are_unwrapped() {
  for source in [
    "while x do\n  print(1)\n  print(2)\nend\n",
    "for i = 1, 2 do\n  print(1)\n  print(2)\nend\n",
    "for i in pairs(t) do\n  print(1)\n  print(2)\nend\n",
    "repeat\n  print(1)\n  print(2)\nuntil x\n",
  ] {
    let mut reducer = Reducer::new();
    let stat = parse(&mut reducer, source);

    assert_eq!(reducer.get_nested_stats(stat).len(), 2, "source: {source}");
  }
}

/// 函数语句取 `AstExprFunction` 的 body
#[test]
fn function_bodies_are_unwrapped() {
  for source in [
    "function f()\n  print(1)\n  print(2)\nend\n",
    "local function f()\n  print(1)\n  print(2)\nend\n",
  ] {
    let mut reducer = Reducer::new();
    let stat = parse(&mut reducer, source);

    assert_eq!(reducer.get_nested_stats(stat).len(), 2, "source: {source}");
  }
}

/// 叶子语句没有嵌套语句（cpp 走完 else-if 链后返回空表）
#[test]
fn leaf_statement_has_no_nested_stats() {
  let mut reducer = Reducer::new();
  let stat = parse(&mut reducer, "local x = 1\n");

  assert!(reducer.get_nested_stats(stat).is_empty());
}
