//! CST 位置信息用例：对照 cpp/tests/Parser.test.cpp 的
//! `type_pack_explicit_with_cst`(:4169) 与
//! `optional_return_type_pack_with_cst_func_return`(:4199)。
//!
//! `store_cst_data=true` 时解析器把括号/逗号位置写入 `CstTypePackExplicit`；
//! `LuauSingleTypeOptionalPackReturnsAttributeParens` 决定「单一可选返回类型」
//! 的括号归属——`(number, ...string)` 的括号属于外层 type pack（记录位置），
//! `(string | number)?` 的括号属于内层 group（pack 侧记 `missing`）。两例把
//! 两种归属的 CST 位置逐字段钉住。

use ulua_ast::{
  records::{
    allocator::Allocator, ast_name_table::AstNameTable, ast_node::AstNode,
    ast_stat_block::AstStatBlock, ast_stat_type_alias::AstStatTypeAlias,
    ast_type_function::AstTypeFunction, ast_type_group::AstTypeGroup,
    ast_type_optional::AstTypeOptional, ast_type_pack::AstTypePack,
    ast_type_pack_explicit::AstTypePackExplicit, ast_type_union::AstTypeUnion,
    cst_type_pack_explicit::CstTypePackExplicit, parse_options::ParseOptions,
    parse_result::ParseResult, parser::Parser, position::Position,
  },
  rtti::{AstNodeView, CstNodeClass, ast_node_try_as, ast_node_try_as_ptr, cst_node_try_as},
};
use ulua_common::{
  fflag::LuauSingleTypeOptionalPackReturnsAttributeParens, records::f_value::FValue,
};

/// 线程局部 FFlag 覆盖守卫（ulua-unit-test `ScopedFastFlag` 的本地同款：
/// ulua-ast 不能依赖该 crate）。Drop 保证断言失败也不把覆盖泄漏给同线程
/// 复用的后续测试。
struct FlagGuard(&'static FValue<bool>);

impl FlagGuard {
  fn set(flag: &'static FValue<bool>, value: bool) -> Self {
    flag.push_test_override(value);
    Self(flag)
  }
}

/// 位置字面量简写（location.rs 同款）。
fn p(line: u32, column: u32) -> Position {
  Position { line, column }
}

impl Drop for FlagGuard {
  fn drop(&mut self) {
    self.0.pop_test_override();
  }
}

/// 解析 `src`（store_cst_data=true）并把结果交给闭包检查。`Box` 钉堆：
/// `AstNameTable`/`Parser` 捕获 `Allocator` 地址，宿主移动即悬垂；闭包返回前
/// arena 存活，节点与 CST 指针均有效。
fn parse_cst<R>(src: &str, f: impl FnOnce(&ParseResult, &AstStatBlock) -> R) -> R {
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);
  let result = Parser::parse(
    src,
    &mut names,
    &mut allocator,
    ParseOptions {
      store_cst_data: true,
      ..ParseOptions::default()
    },
  );
  assert!(!result.root.is_null(), "解析应产出根块");
  // Safety: result.root 指向上方 arena 分配的存活根块，闭包借用期间无并发访问。
  f(&result, unsafe { &*result.root })
}

/// 按 AST 节点地址在 `cst_node_map` 查 CST 节点并下转（cpp
/// `result.cstNodeMap.find(node)` + `->as<T>()`）。键地址与具体节点指针
/// repr(C) 首字段基址重合。
fn cst_of<T: CstNodeClass>(result: &ParseResult, node: *mut AstNode) -> Option<&T> {
  let raw = *result.cst_node_map.find(&node)?;
  if raw.is_null() {
    return None;
  }
  // Safety: CST 节点由 parser 与 AST 同 arena 构造，result 存续期内存活。
  cst_node_try_as::<T>(unsafe { &*raw })
}

// cpp Parser.test.cpp:4169 —— 显式返回 type pack 的括号/逗号位置逐字段核对。
#[test]
fn type_pack_explicit_with_cst() {
  let _flag = FlagGuard::set(&LuauSingleTypeOptionalPackReturnsAttributeParens, true);
  parse_cst("type T = () -> (number, ...string)", |result, block| {
    assert!(result.errors.is_empty(), "解析应无错误");
    assert_eq!(block.body.len(), 1);

    let alias = ast_node_try_as::<AstStatTypeAlias>(block.body.get(0).unwrap().as_ast_node())
      .expect("首条语句应为 AstStatTypeAlias");
    let fun = unsafe { ast_node_try_as_ptr::<AstTypeFunction>(alias.type_ptr) }
      .expect("别名类型应为 AstTypeFunction");

    let pack_ptr: *mut AstTypePack = fun.return_types;
    let pack = unsafe { ast_node_try_as_ptr::<AstTypePackExplicit>(pack_ptr) }
      .expect("返回类型应为显式 type pack");
    assert_eq!(pack.type_list.types.size, 1);
    assert!(pack.type_list.tail().is_some(), "应有 ...string 尾注");

    let cst =
      cst_of::<CstTypePackExplicit>(result, pack_ptr.cast()).expect("显式 type pack 应有 CST 节点");
    assert_eq!(cst.open_parentheses_position, p(0, 15));
    assert_eq!(cst.close_parentheses_position, p(0, 33));
    assert_eq!(cst.comma_positions.size, 1);
    assert_eq!(cst.comma_positions.as_slice()[0], p(0, 22));
  });
}

// cpp Parser.test.cpp:4199 —— `(string | number)?` 作为返回类型是单一可选类型：
// union( group( union(string, number) ), nil )。括号属于 group，外层返回
// type pack 隐式，不得记录任何括号位置。
#[test]
fn optional_return_type_pack_with_cst_func_return() {
  let _flag = FlagGuard::set(&LuauSingleTypeOptionalPackReturnsAttributeParens, true);
  parse_cst("type T = () -> (string | number)?", |result, block| {
    assert!(result.errors.is_empty(), "解析应无错误");
    assert_eq!(block.body.len(), 1);

    let alias = ast_node_try_as::<AstStatTypeAlias>(block.body.get(0).unwrap().as_ast_node())
      .expect("首条语句应为 AstStatTypeAlias");
    let fun = unsafe { ast_node_try_as_ptr::<AstTypeFunction>(alias.type_ptr) }
      .expect("别名类型应为 AstTypeFunction");

    let pack_ptr: *mut AstTypePack = fun.return_types;
    let pack = unsafe { ast_node_try_as_ptr::<AstTypePackExplicit>(pack_ptr) }
      .expect("返回类型应为显式 type pack");
    assert_eq!(pack.type_list.types.size, 1);
    assert!(pack.type_list.tail().is_none(), "不应有尾注");

    // 唯一返回类型是可选 union：`(string | number)?`
    let optional =
      unsafe { ast_node_try_as_ptr::<AstTypeUnion>(pack.type_list.types.as_slice()[0]) }
        .expect("返回类型应为 AstTypeUnion");
    assert_eq!(optional.types.size, 2);
    unsafe { ast_node_try_as_ptr::<AstTypeGroup>(optional.types.as_slice()[0]) }
      .expect("union[0] 应为 AstTypeGroup（(string | number)）");
    unsafe { ast_node_try_as_ptr::<AstTypeOptional>(optional.types.as_slice()[1]) }
      .expect("union[1] 应为 AstTypeOptional（?）");

    // 括号属于 group 而非返回 type pack：pack 侧 CST 位置缺失。
    let cst =
      cst_of::<CstTypePackExplicit>(result, pack_ptr.cast()).expect("显式 type pack 应有 CST 节点");
    assert_eq!(cst.open_parentheses_position, Position::missing());
    assert_eq!(cst.close_parentheses_position, Position::missing());
  });
}
