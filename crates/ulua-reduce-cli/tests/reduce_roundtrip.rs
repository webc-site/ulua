//! reducer 往返测试：reduce 输出 → 再编译（解析无错）→ 再 reduce，应收敛到
//! 不动点且始终保持 bug 特征（search_text 可复现）。

use std::fs;

use tempfile::TempDir;
use ulua_ast::records::{
  allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions, parser::Parser,
};
use ulua_reduce_cli::run_main;

/// 命令模板：把脚本内容打到 stdout，供 run() 匹配 search_text
#[cfg(not(windows))]
const SHOW_CMD: &str = "cat {}";
#[cfg(windows)]
const SHOW_CMD: &str = "type {}";

const SEARCH_TEXT: &str = "REDUCE_ME";
const MAX_ROUNDS: usize = 8;

/// 单轮 reduce：写源文件 → run_main → 读回输出
fn reduce_once(dir: &TempDir, name: &str, source: &str) -> String {
  let path = dir.path().join(name);
  fs::write(&path, source).expect("write source");

  run_main(&[
    "ulua-reduce".to_string(),
    path.to_string_lossy().into_owned(),
    SHOW_CMD.to_string(),
    SEARCH_TEXT.to_string(),
  ]);

  fs::read_to_string(&path).expect("read reduced")
}

/// 再编译：解析必须无错
fn assert_parses(source: &str, context: &str) {
  let mut allocator = Box::new(Allocator::new());
  let mut name_table = AstNameTable::new(&mut allocator);
  let result = Parser::parse(
    source,
    source.len(),
    &mut name_table,
    &mut allocator,
    ParseOptions::default(),
  );
  assert!(
    result.errors.is_empty(),
    "{context}: parse errors: {:?}\nsource:\n{source}",
    result.errors
  );
}

/// 迭代 reduce 直到不动点（上限 MAX_ROUNDS 轮），返回每轮输出（含原始源码）
fn reduce_to_fixpoint(dir: &TempDir, name: &str, source: &str) -> Vec<String> {
  let mut rounds = vec![source.to_string()];

  for _ in 0..MAX_ROUNDS {
    let out = reduce_once(dir, name, rounds.last().expect("non-empty").as_str());
    let converged = rounds.last().expect("non-empty").as_str() == out.as_str();
    rounds.push(out);
    if converged {
      break;
    }
  }

  rounds
}

/// 公共断言：收敛（末两轮相同）、每轮输出均可编译、marker 保留、noise 删除
fn assert_roundtrip(rounds: &[String], context: &str) {
  assert!(
    rounds.len() <= MAX_ROUNDS + 1,
    "{context}: {MAX_ROUNDS} 轮内未收敛"
  );
  assert_eq!(
    rounds.last().expect("non-empty").as_str(),
    rounds[rounds.len() - 2].as_str(),
    "{context}: 末两轮输出应相同（不动点）"
  );

  for (i, round) in rounds.iter().enumerate().skip(1) {
    assert_parses(round, &format!("{context}: round {i}"));
    assert!(
      round.contains(SEARCH_TEXT),
      "{context}: round {i} 丢失 bug 特征:\n{round}"
    );
  }
}

#[test]
fn top_level_deletion_converges() {
  let dir = TempDir::new().expect("tempdir");
  let source = r#"
local a = 1
print("noise1")
local b = 2
print("REDUCE_ME")
if a then
	print("noise2")
end
for i = 1, 3 do
	print("noise3")
end
local c = a + b
print("REDUCE_ME tail")
"#;

  let rounds = reduce_to_fixpoint(&dir, "top_level.lua", source);
  assert_roundtrip(&rounds, "top-level");

  let final_out = rounds.last().expect("non-empty");
  assert!(!final_out.contains("noise"), "noise 应被删除:\n{final_out}");
}

#[test]
fn nested_block_promotion_converges() {
  let dir = TempDir::new().expect("tempdir");
  let source = r#"
do
	local x = 1
	do
		print("REDUCE_ME")
	end
end
print("noise")
"#;

  let rounds = reduce_to_fixpoint(&dir, "nested.lua", source);
  assert_roundtrip(&rounds, "nested");

  let final_out = rounds.last().expect("non-empty");
  assert!(!final_out.contains("noise"), "noise 应被删除:\n{final_out}");
  // 嵌套 do 块应被 promote 打平，仅剩顶层的 print
  assert!(
    !final_out.contains("do "),
    "do 块应被 promote 打平:\n{final_out}"
  );
}

#[test]
fn hotcomment_preserved_across_rounds() {
  let dir = TempDir::new().expect("tempdir");
  let source = "--!strict\nlocal a = 1\nprint(\"REDUCE_ME\")\nlocal b = 2\n";

  let rounds = reduce_to_fixpoint(&dir, "hotcomment.lua", source);
  assert_roundtrip(&rounds, "hotcomment");

  let final_out = rounds.last().expect("non-empty");
  assert!(
    final_out.starts_with("--!strict\n"),
    "hotcomment 应保留在文件头:\n{final_out}"
  );
  assert!(
    !final_out.contains("local b"),
    "无关局部变量应被删除:\n{final_out}"
  );
}
