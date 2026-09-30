//! 类语法解析规则用例：对照 cpp/tests/Parser.test.cpp 的
//! `classes_cannot_define_new_method`(:3339) / `classes_can_define_new_prefixed_method`(:3356) /
//! `classes_cannot_define_new_prop`(:3367) / `class_extends_not_a_class`(:3527) /
//! `class_extends_imported_class`(:3561)。
//!
//! `class` 语句入口按 `DebugLuauUserDefinedClasses` 分发（parser_parse_stat.rs:65），
//! 故每例经线程局部覆盖守卫开启（ulua-unit-test `ScopedFastFlag` 的本地同款，
//! ulua-ast 不能依赖该 crate）。

use ulua_ast::{
  records::{
    allocator::Allocator, ast_class_property::AstClassProperty,
    ast_expr_constant_string::AstExprConstantString, ast_expr_index_expr::AstExprIndexExpr,
    ast_expr_index_name::AstExprIndexName, ast_expr_local::AstExprLocal,
    ast_name_table::AstNameTable, ast_stat_block::AstStatBlock, ast_stat_class::AstStatClass,
    parse_options::ParseOptions, parse_result::ParseResult, parser::Parser,
  },
  rtti::{AstNodeView, ast_node_try_as, ast_node_try_as_ptr},
};
use ulua_common::{fflag::DebugLuauUserDefinedClasses, records::f_value::FValue};

/// 线程局部 FFlag 覆盖守卫。Drop 保证断言失败也不把覆盖泄漏给同线程复用的
/// 后续测试。
struct FlagGuard(&'static FValue<bool>);

impl FlagGuard {
  fn set(flag: &'static FValue<bool>, value: bool) -> Self {
    flag.push_test_override(value);
    Self(flag)
  }
}

impl Drop for FlagGuard {
  fn drop(&mut self) {
    self.0.pop_test_override();
  }
}

/// 开启类语法后解析 `src`，把结果交给闭包检查。`Box` 钉堆：`Parser` 捕获
/// `Allocator` 地址，宿主移动即悬垂；闭包返回前 arena 存活。
fn parse_class<R>(src: &str, f: impl FnOnce(&ParseResult, &AstStatBlock) -> R) -> R {
  let _flag = FlagGuard::set(&DebugLuauUserDefinedClasses, true);
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);
  let result = Parser::parse(src, &mut names, &mut allocator, ParseOptions::default());
  assert!(!result.root.is_null(), "解析应产出根块");
  // Safety: result.root 指向上方 arena 分配的存活根块，闭包借用期间无并发访问。
  f(&result, unsafe { &*result.root })
}

/// 取 body[i] 并下转为 `AstStatClass`。
fn class_at(block: &AstStatBlock, i: usize) -> &AstStatClass {
  ast_node_try_as::<AstStatClass>(block.body.get(i).unwrap().as_ast_node())
    .unwrap_or_else(|| panic!("body[{i}] 应为 AstStatClass"))
}

/// 取类成员 j 的属性形态。
fn prop_at(cls: &AstStatClass, j: usize) -> &AstClassProperty {
  cls
    .members
    .as_slice()
    .get(j)
    .and_then(|m| m.get_if_0())
    .unwrap_or_else(|| panic!("members[{j}] 应为 AstClassProperty"))
}

// cpp Parser.test.cpp:3339 —— 方法名 `new` 是保留构造器名，必须报错并提示
// 改名 `__init`（注意 cpp 消息里是两个空格）。
#[test]
fn classes_cannot_define_new_method() {
  parse_class(
    "class Point2\n    public x: number\n    public y: number\n\n    function new(x: number, y: number) end\nend\n",
    |result, _| {
      assert_eq!(result.errors.len(), 1, "应恰有一条解析错误");
      assert_eq!(
        result.errors[0].what(),
        "Class methods cannot be named 'new'.  Name it '__init' to define a constructor."
      );
    },
  );
}

// cpp Parser.test.cpp:3356 —— 前缀撞 `new` 的方法名（`newb`）不受构造器名限制。
#[test]
fn classes_can_define_new_prefixed_method() {
  parse_class(
    "class Point2\n    public x: number\n    public y: number\n\n    function newb(x: number, y: number) end\nend\n",
    |result, _| {
      assert!(result.errors.is_empty(), "不应有解析错误");
    },
  );
}

// cpp Parser.test.cpp:3367 —— 属性名 `new` 同样保留给构造器。
#[test]
fn classes_cannot_define_new_prop() {
  parse_class("class Point2\n    public new\nend\n", |result, _| {
    assert_eq!(result.errors.len(), 1, "应恰有一条解析错误");
    assert_eq!(
      result.errors[0].what(),
      "Class properties cannot be named 'new'. Define a method named '__init' to define a constructor."
    );
  });
}

// cpp Parser.test.cpp:3527 —— `extends` 后必须是标识符形态的类引用；字符串
// 与数字字面量各报一条错，且错误恢复不吞类体（成员仍可下转核对）。
#[test]
fn class_extends_not_a_class() {
  parse_class(
    "class Cat extends \"Animal\"\n    public meowMult: number\nend\n\nclass Dog extends 42\n    public barkMult: number\nend\n",
    |result, block| {
      assert_eq!(result.errors.len(), 2, "应恰有两条解析错误");
      assert_eq!(
        result.errors[0].what(),
        "Expected identifier when parsing class reference expression, got \"Animal\""
      );
      assert_eq!(
        result.errors[1].what(),
        "Expected identifier when parsing class reference expression, got '42'"
      );

      assert_eq!(block.body.len(), 2);
      assert_eq!(
        prop_at(class_at(block, 0), 0).name.as_str_or_empty(),
        "meowMult"
      );
      assert_eq!(
        prop_at(class_at(block, 1), 0).name.as_str_or_empty(),
        "barkMult"
      );
    },
  );
}

// cpp Parser.test.cpp:3561 —— `extends m.Animal`（IndexName）与
// `extends m["Animal"]`（IndexExpr）都是合法类引用；super 链下转核对到底。
#[test]
fn class_extends_imported_class() {
  parse_class(
    "local m = require(\"module\")\n\nclass Cat extends m.Animal\n    public meowMult: number\nend\n\nclass Dog extends m[\"Animal\"]\n    public barkMult: number\nend\n",
    |result, block| {
      assert!(result.errors.is_empty(), "不应有解析错误");
      assert_eq!(block.body.len(), 3);

      // Cat extends m.Animal → AstExprIndexName
      let cat = class_at(block, 1);
      let super_index = unsafe { ast_node_try_as_ptr::<AstExprIndexName>(cat.super_) }
        .expect("super 应为 IndexName");
      let super_local = ast_node_try_as::<AstExprLocal>(super_index.expr.get())
        .expect("IndexName 基表达式应为 AstExprLocal");
      assert_eq!(super_local.local.get().name.as_str_or_empty(), "m");
      assert_eq!(super_index.index.as_str_or_empty(), "Animal");

      // Dog extends m["Animal"] → AstExprIndexExpr
      let dog = class_at(block, 2);
      let super_index_expr = unsafe { ast_node_try_as_ptr::<AstExprIndexExpr>(dog.super_) }
        .expect("super 应为 IndexExpr");
      let super_local = ast_node_try_as::<AstExprLocal>(super_index_expr.expr.get())
        .expect("IndexExpr 基表达式应为 AstExprLocal");
      assert_eq!(super_local.local.get().name.as_str_or_empty(), "m");
      let index_str = ast_node_try_as::<AstExprConstantString>(super_index_expr.index.get())
        .expect("IndexExpr 索引应为字符串字面量");
      assert_eq!(index_str.value.as_slice(), b"Animal");
    },
  );
}
