//! RTTI 索引唯一性测试（原型：cpp `Ast.h` / `Cst.h` 的 `LUAU_RTTI` 注册表）。
//!
//! C++ 用 `++gAstRttiIndex` 自增，天然唯一；移植换成对名字做哈希
//! （`ast_rtti_index`），唯一性就从「构造保证」变成「需要验证的性质」——
//! 一旦两个节点类撞到同一个索引，`is<AstExprGroup>()` 这类运行期类型判定会
//! 静默误判。故这里把全量类名清单钉住。若将来新增节点撞车，在该名字上加盐
//! （并同步其 `impl AstNodeClass`）。

use std::collections::BTreeMap;

use ulua_ast::rtti::ast_rtti_index;

/// 对一组类名计算 RTTI 索引并收集碰撞（名字对 + 相同索引）。
fn collect_collisions(names: &[&'static str]) -> Vec<(&'static str, &'static str, i32)> {
  let mut seen: BTreeMap<i32, &str> = BTreeMap::new();
  let mut collisions = Vec::new();
  for &name in names {
    let idx = ast_rtti_index(name);
    if let Some(prev) = seen.insert(idx, name) {
      collisions.push((prev, name, idx));
    }
  }
  collisions
}

/// `Ast.h` 中全部 `LUAU_RTTI(Class)` 类名。
const RTTI_NAMES: &[&str] = &[
  "AstAttr",
  "AstGenericType",
  "AstGenericTypePack",
  "AstExprGroup",
  "AstExprConstantNil",
  "AstExprConstantBool",
  "AstExprConstantNumber",
  "AstExprConstantInteger",
  "AstExprConstantString",
  "AstExprLocal",
  "AstExprGlobal",
  "AstExprVarargs",
  "AstExprCall",
  "AstExprIndexName",
  "AstExprIndexExpr",
  "AstExprFunction",
  "AstExprTable",
  "AstExprUnary",
  "AstExprBinary",
  "AstExprTypeAssertion",
  "AstExprIfElse",
  "AstExprInterpString",
  "AstExprInstantiate",
  "AstExprError",
  "AstStatBlock",
  "AstStatIf",
  "AstStatWhile",
  "AstStatRepeat",
  "AstStatBreak",
  "AstStatContinue",
  "AstStatReturn",
  "AstStatExpr",
  "AstStatLocal",
  "AstStatFor",
  "AstStatForIn",
  "AstStatAssign",
  "AstStatCompoundAssign",
  "AstStatFunction",
  "AstStatLocalFunction",
  "AstStatTypeAlias",
  "AstStatTypeFunction",
  "AstStatDeclareGlobal",
  "AstStatDeclareFunction",
  "AstStatClass",
  "AstStatDeclareExternType",
  "AstStatError",
  "AstTypeReference",
  "AstTypeTable",
  "AstTypeFunction",
  "AstTypeTypeof",
  "AstTypeOptional",
  "AstTypeUnion",
  "AstTypeIntersection",
  "AstTypeSingletonBool",
  "AstTypeSingletonString",
  "AstTypeGroup",
  "AstTypeError",
  "AstTypePackExplicit",
  "AstTypePackVariadic",
  "AstTypePackGeneric",
  "Class",
];

/// `Cst.h` 中全部 `LUAU_CST_RTTI(Class)` 类名。CST 与 AST 共用索引函数但各自
/// 一套索引空间，故只需保证 CST 内部互不碰撞。
const CST_RTTI_NAMES: &[&str] = &[
  "CstAttr",
  "CstExprGroup",
  "CstExprConstantNumber",
  "CstExprConstantInteger",
  "CstExprConstantString",
  "CstExprCall",
  "CstExprIndexExpr",
  "CstExprFunction",
  "CstExprTable",
  "CstExprOp",
  "CstExprTypeAssertion",
  "CstExprIfElse",
  "CstExprInterpString",
  "CstExprExplicitTypeInstantiation",
  "CstStatDo",
  "CstStatIf",
  "CstStatRepeat",
  "CstStatReturn",
  "CstStatLocal",
  "CstStatFor",
  "CstStatForIn",
  "CstStatAssign",
  "CstStatCompoundAssign",
  "CstStatFunction",
  "CstStatLocalFunction",
  "CstGenericType",
  "CstGenericTypePack",
  "CstStatTypeAlias",
  "CstStatTypeFunction",
  "CstTypeReference",
  "CstTypeTable",
  "CstTypeFunction",
  "CstTypeTypeof",
  "CstTypeUnion",
  "CstTypeIntersection",
  "CstTypeSingletonString",
  "CstTypeGroup",
  "CstTypePackExplicit",
  "CstTypePackGeneric",
  "CstParametrizedAttr",
  "Class",
];

#[test]
fn rtti_indices_unique() {
  let collisions = collect_collisions(RTTI_NAMES);
  assert!(
    collisions.is_empty(),
    "AST RTTI index collisions: {collisions:?}"
  );
}

#[test]
fn cst_rtti_indices_unique() {
  let collisions = collect_collisions(CST_RTTI_NAMES);
  assert!(
    collisions.is_empty(),
    "CST RTTI index collisions: {collisions:?}"
  );
}

#[test]
fn rtti_index_is_stable_and_positive() {
  // 稳定性：索引是名字的纯函数（编译期 const fn，不参与迭代顺序）。
  assert_eq!(
    ast_rtti_index("AstExprGroup"),
    ast_rtti_index("AstExprGroup")
  );
  // 非负：给负数哨兵留位置。
  assert!(ast_rtti_index("AstExprGroup") >= 0);
}
