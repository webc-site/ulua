extern crate alloc;

use ulua_common::fflag::LuauSingleTypeOptionalPackReturnsAttributeParens;
use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_basic_escaping() {
  use crate::ast_json_encoder_support::*;

  let mut bytes = "hello \"world\"".as_bytes().to_vec();
  let mut s = AstExprConstantString::new(
    Location::default(),
    byte_array(&mut bytes),
    QuoteStyle::QuotedSimple,
  );

  assert_eq!(
    json_ref(&mut s),
    r#"{"type":"AstExprConstantString","location":"0,0 - 0,0","value":"hello \"world\""}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_annotation() {
  use crate::ast_json_encoder_support::*;

  // cpp:535 `ScopedFastFlag{LuauSingleTypeOptionalPackReturnsAttributeParens, true}`:开启后
  // 单类型返回 pack 不再产内层 AstTypeGroup。原 Rust 用例未置该 flag、断言的是 flag-off 的
  // group 包裹(cpp 用例不存在该配置下的断言),现与 cpp:540 期望逐字对齐。
  let _flag = ScopedFastFlag::new(&LuauSingleTypeOptionalPackReturnsAttributeParens, true);

  let mut fixture = JsonEncoderFixture::new();
  let statement =
    fixture.expect_parse_statement("type T = ((number) -> (string | nil)) & ((string) -> ())");

  assert_eq!(
    json(statement),
    r#"{"type":"AstStatTypeAlias","location":"0,0 - 0,56","name":"T","generics":[],"genericPacks":[],"value":{"type":"AstTypeIntersection","location":"0,9 - 0,56","types":[{"type":"AstTypeGroup","location":"0,9 - 0,37","inner":{"type":"AstTypeFunction","location":"0,10 - 0,36","attributes":[],"generics":[],"genericPacks":[],"argTypes":{"type":"AstTypeList","types":[{"type":"AstTypeReference","location":"0,11 - 0,17","name":"number","nameLocation":"0,11 - 0,17","parameters":[]}]},"argNames":[],"returnTypes":{"type":"AstTypePackExplicit","location":"0,22 - 0,36","typeList":{"type":"AstTypeList","types":[{"type":"AstTypeUnion","location":"0,23 - 0,35","types":[{"type":"AstTypeReference","location":"0,23 - 0,29","name":"string","nameLocation":"0,23 - 0,29","parameters":[]},{"type":"AstTypeReference","location":"0,32 - 0,35","name":"nil","nameLocation":"0,32 - 0,35","parameters":[]}]}]}}}},{"type":"AstTypeGroup","location":"0,40 - 0,56","inner":{"type":"AstTypeFunction","location":"0,41 - 0,55","attributes":[],"generics":[],"genericPacks":[],"argTypes":{"type":"AstTypeList","types":[{"type":"AstTypeReference","location":"0,42 - 0,48","name":"string","nameLocation":"0,42 - 0,48","parameters":[]}]},"argNames":[],"returnTypes":{"type":"AstTypePackExplicit","location":"0,53 - 0,55","typeList":{"type":"AstTypeList","types":[]}}}}]},"exported":false}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_attr() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let expr = fixture.expect_parse_statement("@checked function a(b) return c end");

  assert_eq!(
    json(expr),
    r#"{"type":"AstStatFunction","location":"0,0 - 0,35","name":{"type":"AstExprGlobal","location":"0,18 - 0,19","global":"a"},"func":{"type":"AstExprFunction","location":"0,0 - 0,35","attributes":[{"type":"AstAttr","location":"0,0 - 0,8","name":"checked"}],"generics":[],"genericPacks":[],"args":[{"luauType":null,"name":"b","isConst":false,"type":"AstLocal","location":"0,20 - 0,21"}],"vararg":false,"varargLocation":"0,0 - 0,0","body":{"type":"AstStatBlock","location":"0,22 - 0,32","hasEnd":true,"body":[{"type":"AstStatReturn","location":"0,23 - 0,31","list":[{"type":"AstExprGlobal","location":"0,30 - 0,31","global":"c"}]}]},"functionDepth":1,"debugname":"a"}}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_expr_binary() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let expr = fixture.expect_parse_expr("b + c");

  assert_eq!(
    json(expr),
    r#"{"type":"AstExprBinary","location":"0,4 - 0,9","op":"Add","left":{"type":"AstExprGlobal","location":"0,4 - 0,5","global":"b"},"right":{"type":"AstExprGlobal","location":"0,8 - 0,9","global":"c"}}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_expr_call() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let expr = fixture.expect_parse_expr("foo(1, 2, 3)");

  assert_eq!(
    json(expr),
    r#"{"type":"AstExprCall","location":"0,4 - 0,16","func":{"type":"AstExprGlobal","location":"0,4 - 0,7","global":"foo"},"args":[{"type":"AstExprConstantNumber","location":"0,8 - 0,9","value":1},{"type":"AstExprConstantNumber","location":"0,11 - 0,12","value":2},{"type":"AstExprConstantNumber","location":"0,14 - 0,15","value":3}],"self":false,"argLocation":"0,8 - 0,16"}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_expr_error() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  fixture
    .names
    .rebind_allocator(&mut fixture.allocator as *mut _);
  let src = "a = ";
  let parse_result = Parser::parse(
    src,
    &mut fixture.names,
    &mut fixture.allocator,
    ParseOptions::default(),
  );

  // root 经 `as_ref` 物化只读借用（fixture arena 保活至用例末），替代逐处 `(*root)` 裸解引用。
  let root = unsafe { parse_result.root.as_ref() }.expect("expected parse root block");
  assert_eq!(1, root.body.len());
  // `as_slice()[i]` 读 arena 元素指针，替代 `data.add(i)` 裸指针算术。
  let stat = root.body.as_slice()[0];
  // 判型 + 下转 + 判空一步折叠为 `Option`（cpp `stat->as<AstStatAssign>()`），
  // 未命中即上游不变量破坏，`expect` 承担原 `!is_null` 断言。
  let stat_assign =
    unsafe { ast_node_try_as_ptr::<AstStatAssign>(stat) }.expect("expected AstStatAssign");
  assert_eq!(1, stat_assign.values.size);
  let expr = stat_assign.values.as_slice()[0];

  assert_eq!(
    json(expr),
    r#"{"type":"AstExprError","location":"0,4 - 0,4","expressions":[],"messageIndex":0}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_expr_function() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let expr = fixture.expect_parse_expr("function (a) return a end");

  assert_eq!(
    json(expr),
    r#"{"type":"AstExprFunction","location":"0,4 - 0,29","attributes":[],"generics":[],"genericPacks":[],"args":[{"luauType":null,"name":"a","isConst":false,"type":"AstLocal","location":"0,14 - 0,15"}],"vararg":false,"varargLocation":"0,0 - 0,0","body":{"type":"AstStatBlock","location":"0,16 - 0,26","hasEnd":true,"body":[{"type":"AstStatReturn","location":"0,17 - 0,25","list":[{"type":"AstExprLocal","location":"0,24 - 0,25","local":{"luauType":null,"name":"a","isConst":false,"type":"AstLocal","location":"0,14 - 0,15"}}]}]},"functionDepth":1,"debugname":""}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_expr_global() {
  use crate::ast_json_encoder_support::*;

  let mut global = AstExprGlobal::new(Location::default(), ast_name(b"print"));

  assert_eq!(
    json_ref(&mut global),
    r#"{"type":"AstExprGlobal","location":"0,0 - 0,0","global":"print"}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_expr_group() {
  use crate::ast_json_encoder_support::*;

  let mut number =
    AstExprConstantNumber::new(Location::default(), 5.0, ConstantNumberParseResult::Ok);
  let mut group = AstExprGroup::new(Location::default(), Node::from_mut(&mut number.base));

  assert_eq!(
    json_ref(&mut group),
    r#"{"type":"AstExprGroup","location":"0,0 - 0,0","expr":{"type":"AstExprConstantNumber","location":"0,0 - 0,0","value":5}}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_expr_if_then() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let statement = fixture.expect_parse_statement("local a = if x then y else z");

  assert_eq!(
    json(statement),
    r#"{"type":"AstStatLocal","location":"0,0 - 0,28","vars":[{"luauType":null,"name":"a","isConst":false,"type":"AstLocal","location":"0,6 - 0,7"}],"values":[{"type":"AstExprIfElse","location":"0,10 - 0,28","condition":{"type":"AstExprGlobal","location":"0,13 - 0,14","global":"x"},"hasThen":true,"trueExpr":{"type":"AstExprGlobal","location":"0,20 - 0,21","global":"y"},"hasElse":true,"falseExpr":{"type":"AstExprGlobal","location":"0,27 - 0,28","global":"z"}}]}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_expr_index_expr() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let expr = fixture.expect_parse_expr("foo['bar']");

  assert_eq!(
    json(expr),
    r#"{"type":"AstExprIndexExpr","location":"0,4 - 0,14","expr":{"type":"AstExprGlobal","location":"0,4 - 0,7","global":"foo"},"index":{"type":"AstExprConstantString","location":"0,8 - 0,13","value":"bar"}}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_expr_index_name() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let expr = fixture.expect_parse_expr("foo.bar");

  assert_eq!(
    json(expr),
    r#"{"type":"AstExprIndexName","location":"0,4 - 0,11","expr":{"type":"AstExprGlobal","location":"0,4 - 0,7","global":"foo"},"index":"bar","indexLocation":"0,8 - 0,11","op":"."}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_expr_interp_string() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let statement = fixture.expect_parse_statement("local a = `var = {x}`");

  assert_eq!(
    json(statement),
    r#"{"type":"AstStatLocal","location":"0,0 - 0,21","vars":[{"luauType":null,"name":"a","isConst":false,"type":"AstLocal","location":"0,6 - 0,7"}],"values":[{"type":"AstExprInterpString","location":"0,10 - 0,21","strings":["var = ",""],"expressions":[{"type":"AstExprGlobal","location":"0,18 - 0,19","global":"x"}]}]}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_expr_local() {
  use crate::ast_json_encoder_support::*;

  let mut local = AstLocal::new(
    ast_name(b"foo"),
    Location::default(),
    null_mut(),
    0,
    0,
    null_mut(),
    false,
  );
  let mut expr_local = AstExprLocal::new(Location::default(), Node::from_mut(&mut local), false);

  assert_eq!(
    json_ref(&mut expr_local),
    r#"{"type":"AstExprLocal","location":"0,0 - 0,0","local":{"luauType":null,"name":"foo","isConst":false,"type":"AstLocal","location":"0,0 - 0,0"}}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_expr_table() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let expr = fixture.expect_parse_expr("{true, key=true, [key2]=true}");

  assert_eq!(
    json(expr),
    r#"{"type":"AstExprTable","location":"0,4 - 0,33","items":[{"type":"AstExprTableItem","kind":"item","value":{"type":"AstExprConstantBool","location":"0,5 - 0,9","value":true}},{"type":"AstExprTableItem","kind":"record","key":{"type":"AstExprConstantString","location":"0,11 - 0,14","value":"key"},"value":{"type":"AstExprConstantBool","location":"0,15 - 0,19","value":true}},{"type":"AstExprTableItem","kind":"general","key":{"type":"AstExprGlobal","location":"0,22 - 0,26","global":"key2"},"value":{"type":"AstExprConstantBool","location":"0,28 - 0,32","value":true}}]}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_expr_type_assertion() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let expr = fixture.expect_parse_expr("b :: any");

  assert_eq!(
    json(expr),
    r#"{"type":"AstExprTypeAssertion","location":"0,4 - 0,12","expr":{"type":"AstExprGlobal","location":"0,4 - 0,5","global":"b"},"annotation":{"type":"AstTypeReference","location":"0,9 - 0,12","name":"any","nameLocation":"0,9 - 0,12","parameters":[]}}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_expr_unary() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let expr = fixture.expect_parse_expr("-b");

  assert_eq!(
    json(expr),
    r#"{"type":"AstExprUnary","location":"0,4 - 0,6","op":"Minus","expr":{"type":"AstExprGlobal","location":"0,5 - 0,6","global":"b"}}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_expr_varargs() {
  use crate::ast_json_encoder_support::*;

  let mut varargs = AstExprVarargs::new(Location::default());

  assert_eq!(
    json_ref(&mut varargs),
    r#"{"type":"AstExprVarargs","location":"0,0 - 0,0"}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_generic_type() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let root = fixture.expect_parse(
    r#"
        a = function<b, c>()
        end
    "#,
  );

  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`varargs` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  assert_eq!(1, unsafe { (*root).body.len() });

  assert_eq!(
    // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`varargs` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
    json(unsafe { block_statement(root, 0) }),
    r#"{"type":"AstStatAssign","location":"1,8 - 2,11","vars":[{"type":"AstExprGlobal","location":"1,8 - 1,9","global":"a"}],"values":[{"type":"AstExprFunction","location":"1,12 - 2,11","attributes":[],"generics":[{"type":"AstGenericType","name":"b"},{"type":"AstGenericType","name":"c"}],"genericPacks":[],"args":[],"vararg":false,"varargLocation":"0,0 - 0,0","body":{"type":"AstStatBlock","location":"1,28 - 2,8","hasEnd":true,"body":[]},"functionDepth":1,"debugname":""}]}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_generic_type_pack() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let root = fixture.expect_parse(
    r#"
        a = function<b..., c...>()
        end
    "#,
  );

  assert_eq!(1, unsafe { (*root).body.len() });

  assert_eq!(
    json(unsafe { block_statement(root, 0) }),
    r#"{"type":"AstStatAssign","location":"1,8 - 2,11","vars":[{"type":"AstExprGlobal","location":"1,8 - 1,9","global":"a"}],"values":[{"type":"AstExprFunction","location":"1,12 - 2,11","attributes":[],"generics":[],"genericPacks":[{"type":"AstGenericTypePack","name":"b"},{"type":"AstGenericTypePack","name":"c"}],"args":[],"vararg":false,"varargLocation":"0,0 - 0,0","body":{"type":"AstStatBlock","location":"1,34 - 2,8","hasEnd":true,"body":[]},"functionDepth":1,"debugname":""}]}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_generic_type_pack_with_default() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let root = fixture.expect_parse(
    r#"
        type Foo<X... = ...string> = any
    "#,
  );

  assert_eq!(1, unsafe { (*root).body.len() });

  assert_eq!(
    json(unsafe { block_statement(root, 0) }),
    r#"{"type":"AstStatTypeAlias","location":"1,8 - 1,40","name":"Foo","generics":[],"genericPacks":[{"type":"AstGenericTypePack","name":"X","luauType":{"type":"AstTypePackVariadic","location":"1,24 - 1,33","variadicType":{"type":"AstTypeReference","location":"1,27 - 1,33","name":"string","nameLocation":"1,27 - 1,33","parameters":[]}}}],"value":{"type":"AstTypeReference","location":"1,37 - 1,40","name":"any","nameLocation":"1,37 - 1,40","parameters":[]},"exported":false}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_generic_type_with_default() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let root = fixture.expect_parse(
    r#"
        type Foo<X = string> = X
    "#,
  );

  assert_eq!(1, unsafe { (*root).body.len() });

  assert_eq!(
    json(unsafe { block_statement(root, 0) }),
    r#"{"type":"AstStatTypeAlias","location":"1,8 - 1,32","name":"Foo","generics":[{"type":"AstGenericType","name":"X","luauType":{"type":"AstTypeReference","location":"1,21 - 1,27","name":"string","nameLocation":"1,21 - 1,27","parameters":[]}}],"genericPacks":[],"value":{"type":"AstTypeReference","location":"1,31 - 1,32","name":"X","nameLocation":"1,31 - 1,32","parameters":[]},"exported":false}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_stat_block() {
  use crate::ast_json_encoder_support::*;

  let mut astlocal = AstLocal::new(
    ast_name(b"a_local"),
    Location::default(),
    null_mut(),
    0,
    0,
    null_mut(),
    false,
  );
  let mut vars = [from_mut(&mut astlocal)];
  let mut local = AstStatLocal::new(
    Location::default(),
    array(&mut vars),
    AstArray::default(),
    None,
    false,
  );
  let body = Nodes::from_vec(vec![Node::from_mut(&mut local.base)]);
  let mut block = AstStatBlock::new(Location::default(), body, true);

  assert_eq!(
    json_ref(&mut block),
    r#"{"type":"AstStatBlock","location":"0,0 - 0,0","hasEnd":true,"body":[{"type":"AstStatLocal","location":"0,0 - 0,0","vars":[{"luauType":null,"name":"a_local","isConst":false,"type":"AstLocal","location":"0,0 - 0,0"}],"values":[]}]}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_stat_break() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let statement = fixture.expect_parse_statement("while true do break end");

  assert_eq!(
    json(statement),
    r#"{"type":"AstStatWhile","location":"0,0 - 0,23","condition":{"type":"AstExprConstantBool","location":"0,6 - 0,10","value":true},"body":{"type":"AstStatBlock","location":"0,13 - 0,20","hasEnd":true,"body":[{"type":"AstStatBreak","location":"0,14 - 0,19"}]},"hasDo":true}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_stat_compound_assign() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let statement = fixture.expect_parse_statement("a += b");

  assert_eq!(
    json(statement),
    r#"{"type":"AstStatCompoundAssign","location":"0,0 - 0,6","op":"Add","var":{"type":"AstExprGlobal","location":"0,0 - 0,1","global":"a"},"value":{"type":"AstExprGlobal","location":"0,5 - 0,6","global":"b"}}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_stat_continue() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let statement = fixture.expect_parse_statement("while true do continue end");

  assert_eq!(
    json(statement),
    r#"{"type":"AstStatWhile","location":"0,0 - 0,26","condition":{"type":"AstExprConstantBool","location":"0,6 - 0,10","value":true},"body":{"type":"AstStatBlock","location":"0,13 - 0,23","hasEnd":true,"body":[{"type":"AstStatContinue","location":"0,14 - 0,22"}]},"hasDo":true}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_stat_declare_class() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let root = fixture.expect_parse(
    r#"
        declare extern type Foo with
            prop: number
            function method(self, foo: number): string
        end

        declare extern type Bar extends Foo with
            prop2: string
        end
    "#,
  );

  assert_eq!(2, unsafe { (*root).body.len() });

  assert_eq!(
    json(unsafe { block_statement(root, 0) }),
    r#"{"type":"AstStatDeclareClass","location":"1,28 - 4,11","name":"Foo","props":[{"name":"prop","nameLocation":"2,12 - 2,16","type":"AstDeclaredClassProp","luauType":{"type":"AstTypeReference","location":"2,18 - 2,24","name":"number","nameLocation":"2,18 - 2,24","parameters":[]},"location":"2,12 - 2,24"},{"name":"method","nameLocation":"3,21 - 3,27","type":"AstDeclaredClassProp","luauType":{"type":"AstTypeFunction","location":"3,12 - 3,54","attributes":[],"generics":[],"genericPacks":[],"argTypes":{"type":"AstTypeList","types":[{"type":"AstTypeReference","location":"3,39 - 3,45","name":"number","nameLocation":"3,39 - 3,45","parameters":[]}]},"argNames":[{"type":"AstArgumentName","name":"foo","location":"3,34 - 3,37"}],"returnTypes":{"type":"AstTypePackExplicit","location":"3,48 - 3,54","typeList":{"type":"AstTypeList","types":[{"type":"AstTypeReference","location":"3,48 - 3,54","name":"string","nameLocation":"3,48 - 3,54","parameters":[]}]}}},"location":"3,12 - 3,54"}],"indexer":null}"#
  );
  assert_eq!(
    json(unsafe { block_statement(root, 1) }),
    r#"{"type":"AstStatDeclareClass","location":"6,28 - 8,11","name":"Bar","superName":"Foo","props":[{"name":"prop2","nameLocation":"7,12 - 7,17","type":"AstDeclaredClassProp","luauType":{"type":"AstTypeReference","location":"7,19 - 7,25","name":"string","nameLocation":"7,19 - 7,25","parameters":[]},"location":"7,12 - 7,25"}],"indexer":null}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_stat_declare_function() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let statement = fixture.expect_parse_statement("declare function foo(x: number): string");

  assert_eq!(
    json(statement),
    r#"{"type":"AstStatDeclareFunction","location":"0,0 - 0,39","attributes":[],"name":"foo","nameLocation":"0,17 - 0,20","params":{"type":"AstTypeList","types":[{"type":"AstTypeReference","location":"0,24 - 0,30","name":"number","nameLocation":"0,24 - 0,30","parameters":[]}]},"paramNames":[{"type":"AstArgumentName","name":"x","location":"0,21 - 0,22"}],"vararg":false,"varargLocation":"0,0 - 0,0","retTypes":{"type":"AstTypePackExplicit","location":"0,33 - 0,39","typeList":{"type":"AstTypeList","types":[{"type":"AstTypeReference","location":"0,33 - 0,39","name":"string","nameLocation":"0,33 - 0,39","parameters":[]}]}},"generics":[],"genericPacks":[]}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_stat_declare_function2() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let statement =
    fixture.expect_parse_statement("declare function foo(x: number, ...: string): string");

  assert_eq!(
    json(statement),
    r#"{"type":"AstStatDeclareFunction","location":"0,0 - 0,52","attributes":[],"name":"foo","nameLocation":"0,17 - 0,20","params":{"type":"AstTypeList","types":[{"type":"AstTypeReference","location":"0,24 - 0,30","name":"number","nameLocation":"0,24 - 0,30","parameters":[]}],"tailType":{"type":"AstTypePackVariadic","location":"0,37 - 0,43","variadicType":{"type":"AstTypeReference","location":"0,37 - 0,43","name":"string","nameLocation":"0,37 - 0,43","parameters":[]}}},"paramNames":[{"type":"AstArgumentName","name":"x","location":"0,21 - 0,22"}],"vararg":true,"varargLocation":"0,32 - 0,35","retTypes":{"type":"AstTypePackExplicit","location":"0,46 - 0,52","typeList":{"type":"AstTypeList","types":[{"type":"AstTypeReference","location":"0,46 - 0,52","name":"string","nameLocation":"0,46 - 0,52","parameters":[]}]}},"generics":[],"genericPacks":[]}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_stat_for() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let statement = fixture.expect_parse_statement("for a=0,1 do end");

  assert_eq!(
    json(statement),
    r#"{"type":"AstStatFor","location":"0,0 - 0,16","var":{"luauType":null,"name":"a","isConst":false,"type":"AstLocal","location":"0,4 - 0,5"},"from":{"type":"AstExprConstantNumber","location":"0,6 - 0,7","value":0},"to":{"type":"AstExprConstantNumber","location":"0,8 - 0,9","value":1},"body":{"type":"AstStatBlock","location":"0,12 - 0,13","hasEnd":true,"body":[]},"hasDo":true}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_stat_for_in() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let statement = fixture.expect_parse_statement("for a in b do end");

  assert_eq!(
    json(statement),
    r#"{"type":"AstStatForIn","location":"0,0 - 0,17","vars":[{"luauType":null,"name":"a","isConst":false,"type":"AstLocal","location":"0,4 - 0,5"}],"values":[{"type":"AstExprGlobal","location":"0,9 - 0,10","global":"b"}],"body":{"type":"AstStatBlock","location":"0,13 - 0,14","hasEnd":true,"body":[]},"hasIn":true,"hasDo":true}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_stat_if() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let statement = fixture.expect_parse_statement("if true then else end");

  assert_eq!(
    json(statement),
    r#"{"type":"AstStatIf","location":"0,0 - 0,21","condition":{"type":"AstExprConstantBool","location":"0,3 - 0,7","value":true},"thenbody":{"type":"AstStatBlock","location":"0,12 - 0,13","hasEnd":true,"body":[]},"elsebody":{"type":"AstStatBlock","location":"0,17 - 0,18","hasEnd":true,"body":[]},"hasThen":true}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_stat_local_function() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let statement = fixture.expect_parse_statement("local function a(b) return end");

  assert_eq!(
    json(statement),
    r#"{"type":"AstStatLocalFunction","location":"0,0 - 0,30","name":{"luauType":null,"name":"a","isConst":false,"type":"AstLocal","location":"0,15 - 0,16"},"func":{"type":"AstExprFunction","location":"0,0 - 0,30","attributes":[],"generics":[],"genericPacks":[],"args":[{"luauType":null,"name":"b","isConst":false,"type":"AstLocal","location":"0,17 - 0,18"}],"vararg":false,"varargLocation":"0,0 - 0,0","body":{"type":"AstStatBlock","location":"0,19 - 0,27","hasEnd":true,"body":[{"type":"AstStatReturn","location":"0,20 - 0,26","list":[]}]},"functionDepth":1,"debugname":"a"}}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_stat_repeat() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let statement = fixture.expect_parse_statement("repeat until true");

  assert_eq!(
    json(statement),
    r#"{"type":"AstStatRepeat","location":"0,0 - 0,17","condition":{"type":"AstExprConstantBool","location":"0,13 - 0,17","value":true},"body":{"type":"AstStatBlock","location":"0,6 - 0,7","hasEnd":true,"body":[]}}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_stat_type_alias() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let statement = fixture.expect_parse_statement("type A = B");

  assert_eq!(
    json(statement),
    r#"{"type":"AstStatTypeAlias","location":"0,0 - 0,10","name":"A","generics":[],"genericPacks":[],"value":{"type":"AstTypeReference","location":"0,9 - 0,10","name":"B","nameLocation":"0,9 - 0,10","parameters":[]},"exported":false}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_stat_while() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let statement = fixture.expect_parse_statement("while true do end");

  assert_eq!(
    json(statement),
    r#"{"type":"AstStatWhile","location":"0,0 - 0,17","condition":{"type":"AstExprConstantBool","location":"0,6 - 0,10","value":true},"body":{"type":"AstStatBlock","location":"0,13 - 0,14","hasEnd":true,"body":[]},"hasDo":true}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_type_error() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let parse_result = fixture.parse("type T = ");
  assert_eq!(1, unsafe { (*parse_result.root).body.len() });

  let statement = unsafe { block_statement(parse_result.root, 0) };

  assert_eq!(
    json(statement),
    r#"{"type":"AstStatTypeAlias","location":"0,0 - 0,9","name":"T","generics":[],"genericPacks":[],"value":{"type":"AstTypeError","location":"0,8 - 0,9","types":[],"messageIndex":0},"exported":false}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_type_function() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let statement =
    fixture.expect_parse_statement(r#"type fun = (string, bool, named: number) -> ()"#);

  assert_eq!(
    json(statement),
    r#"{"type":"AstStatTypeAlias","location":"0,0 - 0,46","name":"fun","generics":[],"genericPacks":[],"value":{"type":"AstTypeFunction","location":"0,11 - 0,46","attributes":[],"generics":[],"genericPacks":[],"argTypes":{"type":"AstTypeList","types":[{"type":"AstTypeReference","location":"0,12 - 0,18","name":"string","nameLocation":"0,12 - 0,18","parameters":[]},{"type":"AstTypeReference","location":"0,20 - 0,24","name":"bool","nameLocation":"0,20 - 0,24","parameters":[]},{"type":"AstTypeReference","location":"0,33 - 0,39","name":"number","nameLocation":"0,33 - 0,39","parameters":[]}]},"argNames":[null,null,{"type":"AstArgumentName","name":"named","location":"0,26 - 0,31"}],"returnTypes":{"type":"AstTypePackExplicit","location":"0,44 - 0,46","typeList":{"type":"AstTypeList","types":[]}}},"exported":false}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_type_optional() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let root = fixture.expect_parse(
    r#"
            type Foo = string?
        "#,
  );

  assert_eq!(1, unsafe { (*root).body.len() });

  assert_eq!(
    json(unsafe { block_statement(root, 0) }),
    r#"{"type":"AstStatTypeAlias","location":"1,12 - 1,30","name":"Foo","generics":[],"genericPacks":[],"value":{"type":"AstTypeUnion","location":"1,23 - 1,30","types":[{"type":"AstTypeReference","location":"1,23 - 1,29","name":"string","nameLocation":"1,23 - 1,29","parameters":[]},{"type":"AstTypeOptional","location":"1,29 - 1,30"}]},"exported":false}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_ast_type_pack_explicit() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let root = fixture.expect_parse(
    r#"
        type A<T...> = () -> T...
        local a: A<(number, string)>
    "#,
  );

  assert_eq!(2, unsafe { (*root).body.len() });

  assert_eq!(
    json(unsafe { block_statement(root, 1) }),
    r#"{"type":"AstStatLocal","location":"2,8 - 2,36","vars":[{"luauType":{"type":"AstTypeReference","location":"2,17 - 2,36","name":"A","nameLocation":"2,17 - 2,18","parameters":[{"type":"AstTypePackExplicit","location":"2,19 - 2,20","typeList":{"type":"AstTypeList","types":[{"type":"AstTypeReference","location":"2,20 - 2,26","name":"number","nameLocation":"2,20 - 2,26","parameters":[]},{"type":"AstTypeReference","location":"2,28 - 2,34","name":"string","nameLocation":"2,28 - 2,34","parameters":[]}]}}]},"name":"a","isConst":false,"type":"AstLocal","location":"2,14 - 2,15"}],"values":[]}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_constants() {
  use crate::ast_json_encoder_support::*;

  let mut nil = AstExprConstantNil::new(Location::default());
  let mut b = AstExprConstantBool::new(Location::default(), true);
  let mut n = AstExprConstantNumber::new(Location::default(), 8.2, ConstantNumberParseResult::Ok);
  let mut big_num = AstExprConstantNumber::new(
    Location::default(),
    0.1677721600000003,
    ConstantNumberParseResult::Ok,
  );
  let mut positive_infinity = AstExprConstantNumber::new(
    Location::default(),
    f64::INFINITY,
    ConstantNumberParseResult::Ok,
  );
  let mut negative_infinity = AstExprConstantNumber::new(
    Location::default(),
    f64::NEG_INFINITY,
    ConstantNumberParseResult::Ok,
  );
  let mut nan =
    AstExprConstantNumber::new(Location::default(), f64::NAN, ConstantNumberParseResult::Ok);

  let mut char_string = [b'a', 0x1d, 0, b'\\', b'"', b'b'];
  let mut needs_escaping = AstExprConstantString::new(
    Location::default(),
    byte_array(&mut char_string),
    QuoteStyle::QuotedSimple,
  );

  assert_eq!(
    json_ref(&mut nil),
    r#"{"type":"AstExprConstantNil","location":"0,0 - 0,0"}"#
  );
  assert_eq!(
    json_ref(&mut b),
    r#"{"type":"AstExprConstantBool","location":"0,0 - 0,0","value":true}"#
  );
  assert_eq!(
    json_ref(&mut n),
    r#"{"type":"AstExprConstantNumber","location":"0,0 - 0,0","value":8.1999999999999993}"#
  );
  assert_eq!(
    json_ref(&mut big_num),
    r#"{"type":"AstExprConstantNumber","location":"0,0 - 0,0","value":0.16777216000000031}"#
  );
  assert_eq!(
    json_ref(&mut positive_infinity),
    r#"{"type":"AstExprConstantNumber","location":"0,0 - 0,0","value":Infinity}"#
  );
  assert_eq!(
    json_ref(&mut negative_infinity),
    r#"{"type":"AstExprConstantNumber","location":"0,0 - 0,0","value":-Infinity}"#
  );
  assert_eq!(
    json_ref(&mut nan),
    r#"{"type":"AstExprConstantNumber","location":"0,0 - 0,0","value":NaN}"#
  );
  assert_eq!(
    json_ref(&mut needs_escaping),
    r#"{"type":"AstExprConstantString","location":"0,0 - 0,0","value":"a\u001d\u0000\\\"b"}"#
  );

  // cpp:70-72,90-92 AstExprConstantInteger 三例:小正数/负数/i64::MAX 逐条锁精确 JSON。
  // (Rust 侧原缺失,整批补齐;表驱动。)
  for (value, literal) in [
    (42i64, "42"),                     // cpp:90
    (-1i64, "-1"),                     // cpp:91
    (i64::MAX, "9223372036854775807"), // cpp:92 0x7FFFFFFFFFFFFFFF
  ] {
    let mut int =
      AstExprConstantInteger::new(Location::default(), value, ConstantNumberParseResult::Ok);
    assert_eq!(
      json_ref(&mut int),
      format!(r#"{{"type":"AstExprConstantInteger","location":"0,0 - 0,0","value":{literal}}}"#),
      "cpp:90-92 integer {value}"
    );
  }

  // cpp:76-78,94 hasShorthands:退格/换页/换行/回车/制表符走单字符短转义而非 \u00xx。
  let mut shorthand_raw = [b'x', 0x08, 0x0C, b'\n', b'\r', b'\t', b'y'];
  let mut has_shorthands = AstExprConstantString::new(
    Location::default(),
    byte_array(&mut shorthand_raw),
    QuoteStyle::QuotedSimple,
  );
  assert_eq!(
    json_ref(&mut has_shorthands),
    r#"{"type":"AstExprConstantString","location":"0,0 - 0,0","value":"x\b\f\n\r\ty"}"#
  );

  // cpp:80-81,95 hasUtf8:≥0x80 的 UTF-8 字节原样透传(é=U+00E9、😀=U+1F600)。
  let mut utf8_raw = "e\u{00E9}\u{1F600}".as_bytes().to_vec();
  let mut has_utf8 = AstExprConstantString::new(
    Location::default(),
    byte_array(&mut utf8_raw),
    QuoteStyle::QuotedSimple,
  );
  assert_eq!(
    json_ref(&mut has_utf8),
    r#"{"type":"AstExprConstantString","location":"0,0 - 0,0","value":"eé😀"}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_indexed_type_literal() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let statement = fixture.expect_parse_statement(r#"type StringSet = { [string]: true }"#);

  assert_eq!(
    json(statement),
    r#"{"type":"AstStatTypeAlias","location":"0,0 - 0,35","name":"StringSet","generics":[],"genericPacks":[],"value":{"type":"AstTypeTable","location":"0,17 - 0,35","props":[],"indexer":{"location":"0,19 - 0,33","indexType":{"type":"AstTypeReference","location":"0,20 - 0,26","name":"string","nameLocation":"0,20 - 0,26","parameters":[]},"resultType":{"type":"AstTypeSingletonBool","location":"0,29 - 0,33","value":true}}},"exported":false}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_table_array() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let root = fixture.expect_parse(r#"type X = {string}"#);

  assert_eq!(
    json(root),
    r#"{"type":"AstStatBlock","location":"0,0 - 0,17","hasEnd":true,"body":[{"type":"AstStatTypeAlias","location":"0,0 - 0,17","name":"X","generics":[],"genericPacks":[],"value":{"type":"AstTypeTable","location":"0,9 - 0,17","props":[],"indexer":{"location":"0,10 - 0,16","indexType":{"type":"AstTypeReference","location":"0,9 - 0,9","name":"number","nameLocation":"0,9 - 0,9","parameters":[]},"resultType":{"type":"AstTypeReference","location":"0,10 - 0,16","name":"string","nameLocation":"0,10 - 0,16","parameters":[]}}},"exported":false}]}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_table_indexer() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let root = fixture.expect_parse(r#"type X = {string}"#);

  assert_eq!(
    json(root),
    r#"{"type":"AstStatBlock","location":"0,0 - 0,17","hasEnd":true,"body":[{"type":"AstStatTypeAlias","location":"0,0 - 0,17","name":"X","generics":[],"genericPacks":[],"value":{"type":"AstTypeTable","location":"0,9 - 0,17","props":[],"indexer":{"location":"0,10 - 0,16","indexType":{"type":"AstTypeReference","location":"0,9 - 0,9","name":"number","nameLocation":"0,9 - 0,9","parameters":[]},"resultType":{"type":"AstTypeReference","location":"0,10 - 0,16","name":"string","nameLocation":"0,10 - 0,16","parameters":[]}}},"exported":false}]}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_tables() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let src = r#"
        local x: {
            foo: number
        } = {
            foo = 123,
        }
    "#;
  let root = fixture.expect_parse(src);

  assert_eq!(
    json(root),
    r#"{"type":"AstStatBlock","location":"0,0 - 6,4","hasEnd":true,"body":[{"type":"AstStatLocal","location":"1,8 - 5,9","vars":[{"luauType":{"type":"AstTypeTable","location":"1,17 - 3,9","props":[{"name":"foo","type":"AstTableProp","location":"2,12 - 2,15","propType":{"type":"AstTypeReference","location":"2,17 - 2,23","name":"number","nameLocation":"2,17 - 2,23","parameters":[]}}],"indexer":null},"name":"x","isConst":false,"type":"AstLocal","location":"1,14 - 1,15"}],"values":[{"type":"AstExprTable","location":"3,12 - 5,9","items":[{"type":"AstExprTableItem","kind":"record","key":{"type":"AstExprConstantString","location":"4,12 - 4,15","value":"foo"},"value":{"type":"AstExprConstantNumber","location":"4,18 - 4,21","value":123}}]}]}]}"#
  );
}

// Source: `tests/AstJsonEncoder.test.cpp`
#[test]
fn ast_json_encoder_encode_type_literal() {
  use crate::ast_json_encoder_support::*;

  let mut fixture = JsonEncoderFixture::new();
  let statement = fixture.expect_parse_statement(
    r#"type Action = { strings: "A" | "B" | "C", mixed: "This" | "That" | true }"#,
  );

  assert_eq!(
    json(statement),
    r#"{"type":"AstStatTypeAlias","location":"0,0 - 0,73","name":"Action","generics":[],"genericPacks":[],"value":{"type":"AstTypeTable","location":"0,14 - 0,73","props":[{"name":"strings","type":"AstTableProp","location":"0,16 - 0,23","propType":{"type":"AstTypeUnion","location":"0,25 - 0,40","types":[{"type":"AstTypeSingletonString","location":"0,25 - 0,28","value":"A"},{"type":"AstTypeSingletonString","location":"0,31 - 0,34","value":"B"},{"type":"AstTypeSingletonString","location":"0,37 - 0,40","value":"C"}]}},{"name":"mixed","type":"AstTableProp","location":"0,42 - 0,47","propType":{"type":"AstTypeUnion","location":"0,49 - 0,71","types":[{"type":"AstTypeSingletonString","location":"0,49 - 0,55","value":"This"},{"type":"AstTypeSingletonString","location":"0,58 - 0,64","value":"That"},{"type":"AstTypeSingletonBool","location":"0,67 - 0,71","value":true}]}}],"indexer":null},"exported":false}"#
  );
}

// cpp 键名钉桩(camelCase):Rust 编码器四处键误写 snake_case——
// `vararg_location`(cpp `Analysis/src/AstJsonEncoder.cpp:471 PROP(varargLocation)`)、
// `prop_type`(cpp :1048 `write("propType")`)、`index_type`/`result_type`(cpp :1074-1075)。
// 期望串逐字取自 cpp 断言(`tests/AstJsonEncoder.test.cpp:268/143/156`)。
// 本用例刻意保持红:是实现缺陷证据,实现(src/)归并行会话修改,修好前不放宽断言;
// 存量用例中同样含 snake 键的旧期望(12 处)在实现修复后一并迁移。
#[test]
fn ast_json_encoder_keys_are_camel_case_per_cpp() {
  use crate::ast_json_encoder_support::*;

  // 表驱动:(cpp 出处行, 编码输入, cpp 期望 JSON)。expr 为 true 时走 a = <expr> 前缀。
  let cases: &[(u32, bool, &str, &str)] = &[
    // cpp:263-271 encode_AstExprFunction
    (
      268,
      true,
      "function (a) return a end",
      r#"{"type":"AstExprFunction","location":"0,4 - 0,29","attributes":[],"generics":[],"genericPacks":[],"args":[{"luauType":null,"name":"a","isConst":false,"type":"AstLocal","location":"0,14 - 0,15"}],"vararg":false,"varargLocation":"0,0 - 0,0","body":{"type":"AstStatBlock","location":"0,16 - 0,26","hasEnd":true,"body":[{"type":"AstStatReturn","location":"0,17 - 0,25","list":[{"type":"AstExprLocal","location":"0,24 - 0,25","local":{"luauType":null,"name":"a","isConst":false,"type":"AstLocal","location":"0,14 - 0,15"}}]}]},"functionDepth":1,"debugname":""}"#,
    ),
    // cpp:128-145 encode_tables(propType 键)
    (
      143,
      false,
      r#"
        local x: {
            foo: number
        } = {
            foo = 123,
        }
    "#,
      r#"{"type":"AstStatBlock","location":"0,0 - 6,4","hasEnd":true,"body":[{"type":"AstStatLocal","location":"1,8 - 5,9","vars":[{"luauType":{"type":"AstTypeTable","location":"1,17 - 3,9","props":[{"name":"foo","type":"AstTableProp","location":"2,12 - 2,15","propType":{"type":"AstTypeReference","location":"2,17 - 2,23","name":"number","nameLocation":"2,17 - 2,23","parameters":[]}}],"indexer":null},"name":"x","isConst":false,"type":"AstLocal","location":"1,14 - 1,15"}],"values":[{"type":"AstExprTable","location":"3,12 - 5,9","items":[{"type":"AstExprTableItem","kind":"record","key":{"type":"AstExprConstantString","location":"4,12 - 4,15","value":"foo"},"value":{"type":"AstExprConstantNumber","location":"4,18 - 4,21","value":123}}]}]}]}"#,
    ),
    // cpp:147-158 encode_table_array(indexType/resultType 键)
    (
      156,
      false,
      "type X = {string}",
      r#"{"type":"AstStatBlock","location":"0,0 - 0,17","hasEnd":true,"body":[{"type":"AstStatTypeAlias","location":"0,0 - 0,17","name":"X","generics":[],"genericPacks":[],"value":{"type":"AstTypeTable","location":"0,9 - 0,17","props":[],"indexer":{"location":"0,10 - 0,16","indexType":{"type":"AstTypeReference","location":"0,9 - 0,9","name":"number","nameLocation":"0,9 - 0,9","parameters":[]},"resultType":{"type":"AstTypeReference","location":"0,10 - 0,16","name":"string","nameLocation":"0,10 - 0,16","parameters":[]}}},"exported":false}]}"#,
    ),
  ];

  for (cpp_line, is_expr, input, expected) in cases {
    let mut fixture = JsonEncoderFixture::new();
    let json_text = if *is_expr {
      json(fixture.expect_parse_expr(input))
    } else {
      json(fixture.expect_parse(input))
    };
    assert_eq!(
      &json_text, expected,
      "cpp AstJsonEncoder.test.cpp:{cpp_line}"
    );
  }
}

pub(crate) mod ast_json_encoder_support {

  pub use core::ptr::{from_mut, null_mut};

  pub use ulua_ast::{
    enums::{constant_number_parse_result::ConstantNumberParseResult, quote_style_ast::QuoteStyle},
    records::{
      ast_array::AstArray,
      ast_expr_constant_bool::AstExprConstantBool,
      ast_expr_constant_integer::AstExprConstantInteger,
      ast_expr_constant_nil::AstExprConstantNil,
      ast_expr_constant_number::AstExprConstantNumber,
      ast_expr_constant_string::AstExprConstantString,
      ast_expr_global::AstExprGlobal,
      ast_expr_group::AstExprGroup,
      ast_expr_local::AstExprLocal,
      ast_expr_varargs::AstExprVarargs,
      ast_local::AstLocal,
      ast_stat_assign::AstStatAssign,
      ast_stat_block::AstStatBlock,
      ast_stat_local::AstStatLocal,
      location::Location,
      node_handle::{Node, Nodes},
      parse_options::ParseOptions,
      parser::Parser,
    },
    rtti::ast_node_try_as_ptr,
  };
  pub use ulua_unit_test::{
    functions::{
      ast_json_encoder_array::array, ast_json_encoder_ast_name::ast_name,
      ast_json_encoder_block_statement::block_statement, ast_json_encoder_byte_array::byte_array,
      ast_json_encoder_json::json, ast_json_encoder_json_ref::json_ref,
    },
    records::json_encoder_fixture::JsonEncoderFixture,
  };
}
// 缺口（未移植，对照 `tests/AstJsonEncoder.test.cpp`，共 4 例）：
// - encode_AstStatIf_if_local（:341）、encode_AstStatIf_if_const（:353）、
//   encode_AstExprIfElse_if_local（:365）、encode_AstExprIfElse_if_const
//   （:378）——依赖 FFlag `DebugLuauIfLocalSyntax`+`DebugLuauIfLocalAnalysis`
//   （均未同步）与 AstStatIf/AstExprIfElse 的 `conditionLocal`/
//   `conditionIsConst` 字段（parser 未接入 if local/const 语法）。同
//   pretty_print_cpp_cases.rs 登记的 if_local×12 阻清单（tst-r27）。
