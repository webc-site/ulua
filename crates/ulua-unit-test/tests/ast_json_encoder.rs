extern crate alloc;

mod ast_json_encoder_basic_escaping {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:86:ast_json_encoder_basic_escaping`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record AstArray (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprConstantString (Ast/include/Luau/Ast.h)
  //!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item ast_json_encoder_basic_escaping

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_basic_escaping() {
    let mut bytes = "hello \"world\""
      .as_bytes()
      .iter()
      .map(|b| *b as c_char)
      .collect::<Vec<_>>();
    let mut s = AstExprConstantString::new(
      Location::default(),
      c_char_array(&mut bytes),
      QuoteStyle::QuotedSimple,
    );

    assert_eq!(
      json_ref(&mut s),
      r#"{"type":"AstExprConstantString","location":"0,0 - 0,0","value":"hello \"world\""}"#
    );
  }
}

mod ast_json_encoder_encode_annotation {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:524:ast_json_encoder_encode_annotation`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStat (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParseStatement (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record AstStatTypeAlias (Ast/include/Luau/Ast.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstTypeIntersection (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypeGroup (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypeFunction (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypeList (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypeReference (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypePackExplicit (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypeUnion (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_annotation

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_annotation() {
    let mut fixture = JsonEncoderFixture::new();
    let statement =
      fixture.expect_parse_statement("type T = ((number) -> (string | nil)) & ((string) -> ())");

    assert_eq!(
      json(statement),
      r#"{"type":"AstStatTypeAlias","location":"0,0 - 0,56","name":"T","generics":[],"genericPacks":[],"value":{"type":"AstTypeIntersection","location":"0,9 - 0,56","types":[{"type":"AstTypeGroup","location":"0,9 - 0,37","inner":{"type":"AstTypeFunction","location":"0,10 - 0,36","attributes":[],"generics":[],"genericPacks":[],"argTypes":{"type":"AstTypeList","types":[{"type":"AstTypeReference","location":"0,11 - 0,17","name":"number","nameLocation":"0,11 - 0,17","parameters":[]}]},"argNames":[],"returnTypes":{"type":"AstTypePackExplicit","location":"0,22 - 0,36","typeList":{"type":"AstTypeList","types":[{"type":"AstTypeGroup","location":"0,22 - 0,36","inner":{"type":"AstTypeUnion","location":"0,23 - 0,35","types":[{"type":"AstTypeReference","location":"0,23 - 0,29","name":"string","nameLocation":"0,23 - 0,29","parameters":[]},{"type":"AstTypeReference","location":"0,32 - 0,35","name":"nil","nameLocation":"0,32 - 0,35","parameters":[]}]}}]}}}},{"type":"AstTypeGroup","location":"0,40 - 0,56","inner":{"type":"AstTypeFunction","location":"0,41 - 0,55","attributes":[],"generics":[],"genericPacks":[],"argTypes":{"type":"AstTypeList","types":[{"type":"AstTypeReference","location":"0,42 - 0,48","name":"string","nameLocation":"0,42 - 0,48","parameters":[]}]},"argNames":[],"returnTypes":{"type":"AstTypePackExplicit","location":"0,53 - 0,55","typeList":{"type":"AstTypeList","types":[]}}}}]},"exported":false}"#
    );
  }
}

mod ast_json_encoder_encode_ast_attr {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:488:ast_json_encoder_encode_ast_attr`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStat (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParseStatement (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AstStatFunction (Ast/include/Luau/Ast.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstExprGlobal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprFunction (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstAttr (Ast/include/Luau/Ast.h)
  //!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
  //!   - type_ref -> record AstLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstStatReturn (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_attr

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_attr() {
    let mut fixture = JsonEncoderFixture::new();
    let expr = fixture.expect_parse_statement("@checked function a(b) return c end");

    assert_eq!(
      json(expr),
      r#"{"type":"AstStatFunction","location":"0,0 - 0,35","name":{"type":"AstExprGlobal","location":"0,18 - 0,19","global":"a"},"func":{"type":"AstExprFunction","location":"0,0 - 0,35","attributes":[{"type":"AstAttr","location":"0,0 - 0,8","name":"checked"}],"generics":[],"genericPacks":[],"args":[{"luauType":null,"name":"b","isConst":false,"type":"AstLocal","location":"0,20 - 0,21"}],"vararg":false,"vararg_location":"0,0 - 0,0","body":{"type":"AstStatBlock","location":"0,22 - 0,32","hasEnd":true,"body":[{"type":"AstStatReturn","location":"0,23 - 0,31","list":[{"type":"AstExprGlobal","location":"0,30 - 0,31","global":"c"}]}]},"functionDepth":1,"debugname":"a"}}"#
    );
  }
}

mod ast_json_encoder_encode_ast_expr_binary {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:325:ast_json_encoder_encode_ast_expr_binary`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstExpr (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParseExpr (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AstExprBinary (Ast/include/Luau/Ast.h)
  //!   - calls -> method BcInstHelper::op (Bytecode/include/Luau/BytecodeOps.h)
  //!   - type_ref -> record AstExprGlobal (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_expr_binary

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_expr_binary() {
    let mut fixture = JsonEncoderFixture::new();
    let expr = fixture.expect_parse_expr("b + c");

    assert_eq!(
      json(expr),
      r#"{"type":"AstExprBinary","location":"0,4 - 0,9","op":"Add","left":{"type":"AstExprGlobal","location":"0,4 - 0,5","global":"b"},"right":{"type":"AstExprGlobal","location":"0,8 - 0,9","global":"c"}}"#
    );
  }
}

mod ast_json_encoder_encode_ast_expr_call {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:264:ast_json_encoder_encode_ast_expr_call`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstExpr (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParseExpr (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AstExprCall (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprGlobal (Ast/include/Luau/Ast.h)
  //!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
  //!   - type_ref -> record AstExprConstantNumber (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_expr_call

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_expr_call() {
    let mut fixture = JsonEncoderFixture::new();
    let expr = fixture.expect_parse_expr("foo(1, 2, 3)");

    assert_eq!(
      json(expr),
      r#"{"type":"AstExprCall","location":"0,4 - 0,16","func":{"type":"AstExprGlobal","location":"0,4 - 0,7","global":"foo"},"args":[{"type":"AstExprConstantNumber","location":"0,8 - 0,9","value":1},{"type":"AstExprConstantNumber","location":"0,11 - 0,12","value":2},{"type":"AstExprConstantNumber","location":"0,14 - 0,15","value":3}],"self":false,"argLocation":"0,8 - 0,16"}"#
    );
  }
}

mod ast_json_encoder_encode_ast_expr_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:345:ast_json_encoder_encode_ast_expr_error`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record ParseResult (Ast/include/Luau/ParseResult.h)
  //!   - type_ref -> record Parser (Ast/include/Luau/Parser.h)
  //!   - calls -> method JsonEncoderFixture::parse (tests/AstJsonEncoder.test.cpp)
  //!   - type_ref -> record AstStatAssign (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExpr (Ast/include/Luau/Ast.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AstExprError (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_expr_error

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_expr_error() {
    let mut fixture = JsonEncoderFixture::new();
    fixture
      .names
      .rebind_allocator(&mut fixture.allocator as *mut _);
    let src = "a = ";
    let parse_result = Parser::parse(
      src,
      src.len(),
      &mut fixture.names,
      &mut fixture.allocator,
      ParseOptions::default(),
    );

    assert_eq!(1, unsafe { (*parse_result.root).body.size });
    let stat = unsafe { *(*parse_result.root).body.data.add(0) };
    let stat_assign = unsafe { ast_node_as::<AstStatAssign>(stat as *mut AstNode) };
    assert!(!stat_assign.is_null());
    assert_eq!(1, unsafe { (*stat_assign).values.size });
    let expr = unsafe { *(*stat_assign).values.data.add(0) };

    assert_eq!(
      json(expr),
      r#"{"type":"AstExprError","location":"0,4 - 0,4","expressions":[],"messageIndex":0}"#
    );
  }
}

mod ast_json_encoder_encode_ast_expr_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:293:ast_json_encoder_encode_ast_expr_function`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstExpr (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParseExpr (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AstExprFunction (Ast/include/Luau/Ast.h)
  //!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstStatReturn (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_expr_function

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_expr_function() {
    let mut fixture = JsonEncoderFixture::new();
    let expr = fixture.expect_parse_expr("function (a) return a end");

    assert_eq!(
      json(expr),
      r#"{"type":"AstExprFunction","location":"0,4 - 0,29","attributes":[],"generics":[],"genericPacks":[],"args":[{"luauType":null,"name":"a","isConst":false,"type":"AstLocal","location":"0,14 - 0,15"}],"vararg":false,"vararg_location":"0,0 - 0,0","body":{"type":"AstStatBlock","location":"0,16 - 0,26","hasEnd":true,"body":[{"type":"AstStatReturn","location":"0,17 - 0,25","list":[{"type":"AstExprLocal","location":"0,24 - 0,25","local":{"luauType":null,"name":"a","isConst":false,"type":"AstLocal","location":"0,14 - 0,15"}}]}]},"functionDepth":1,"debugname":""}"#
    );
  }
}

mod ast_json_encoder_encode_ast_expr_global {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:206:ast_json_encoder_encode_ast_expr_global`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstExprGlobal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstName (Ast/include/Luau/Ast.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_expr_global

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_expr_global() {
    let mut global = AstExprGlobal::new(Location::default(), ast_name(c"print"));

    assert_eq!(
      json_ref(&mut global),
      r#"{"type":"AstExprGlobal","location":"0,0 - 0,0","global":"print"}"#
    );
  }
}

mod ast_json_encoder_encode_ast_expr_group {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:193:ast_json_encoder_encode_ast_expr_group`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstExprConstantNumber (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstExprGroup (Ast/include/Luau/Ast.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_expr_group

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_expr_group() {
    let mut number =
      AstExprConstantNumber::new(Location::default(), 5.0, ConstantNumberParseResult::Ok);
    let mut group = AstExprGroup::new(
      Location::default(),
      &mut number as *mut AstExprConstantNumber as *mut AstExpr,
    );

    assert_eq!(
      json_ref(&mut group),
      r#"{"type":"AstExprGroup","location":"0,0 - 0,0","expr":{"type":"AstExprConstantNumber","location":"0,0 - 0,0","value":5}}"#
    );
  }
}

mod ast_json_encoder_encode_ast_expr_if_then {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:216:ast_json_encoder_encode_ast_expr_if_then`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStat (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParseStatement (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprIfElse (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprGlobal (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_expr_if_then

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_expr_if_then() {
    let mut fixture = JsonEncoderFixture::new();
    let statement = fixture.expect_parse_statement("local a = if x then y else z");

    assert_eq!(
      json(statement),
      r#"{"type":"AstStatLocal","location":"0,0 - 0,28","vars":[{"luauType":null,"name":"a","isConst":false,"type":"AstLocal","location":"0,6 - 0,7"}],"values":[{"type":"AstExprIfElse","location":"0,10 - 0,28","condition":{"type":"AstExprGlobal","location":"0,13 - 0,14","global":"x"},"hasThen":true,"trueExpr":{"type":"AstExprGlobal","location":"0,20 - 0,21","global":"y"},"hasElse":true,"falseExpr":{"type":"AstExprGlobal","location":"0,27 - 0,28","global":"z"}}]}"#
    );
  }
}

mod ast_json_encoder_encode_ast_expr_index_expr {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:283:ast_json_encoder_encode_ast_expr_index_expr`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstExpr (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParseExpr (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AstExprIndexExpr (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprGlobal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprConstantString (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_expr_index_expr

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_expr_index_expr() {
    let mut fixture = JsonEncoderFixture::new();
    let expr = fixture.expect_parse_expr("foo['bar']");

    assert_eq!(
      json(expr),
      r#"{"type":"AstExprIndexExpr","location":"0,4 - 0,14","expr":{"type":"AstExprGlobal","location":"0,4 - 0,7","global":"foo"},"index":{"type":"AstExprConstantString","location":"0,8 - 0,13","value":"bar"}}"#
    );
  }
}

mod ast_json_encoder_encode_ast_expr_index_name {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:273:ast_json_encoder_encode_ast_expr_index_name`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstExpr (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParseExpr (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AstExprIndexName (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprGlobal (Ast/include/Luau/Ast.h)
  //!   - calls -> method BcInstHelper::op (Bytecode/include/Luau/BytecodeOps.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_expr_index_name

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_expr_index_name() {
    let mut fixture = JsonEncoderFixture::new();
    let expr = fixture.expect_parse_expr("foo.bar");

    assert_eq!(
      json(expr),
      r#"{"type":"AstExprIndexName","location":"0,4 - 0,11","expr":{"type":"AstExprGlobal","location":"0,4 - 0,7","global":"foo"},"index":"bar","indexLocation":"0,8 - 0,11","op":"."}"#
    );
  }
}

mod ast_json_encoder_encode_ast_expr_interp_string {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:228:ast_json_encoder_encode_ast_expr_interp_string`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStat (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParseStatement (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprInterpString (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprGlobal (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_expr_interp_string

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_expr_interp_string() {
    let mut fixture = JsonEncoderFixture::new();
    let statement = fixture.expect_parse_statement("local a = `var = {x}`");

    assert_eq!(
      json(statement),
      r#"{"type":"AstStatLocal","location":"0,0 - 0,21","vars":[{"luauType":null,"name":"a","isConst":false,"type":"AstLocal","location":"0,6 - 0,7"}],"values":[{"type":"AstExprInterpString","location":"0,10 - 0,21","strings":["var = ",""],"expressions":[{"type":"AstExprGlobal","location":"0,18 - 0,19","global":"x"}]}]}"#
    );
  }
}

mod ast_json_encoder_encode_ast_expr_local {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:240:ast_json_encoder_encode_ast_expr_local`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstName (Ast/include/Luau/Ast.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_expr_local

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_expr_local() {
    let mut local = AstLocal::new(
      ast_name(c"foo"),
      Location::default(),
      null_mut(),
      0,
      0,
      null_mut(),
      false,
    );
    let mut expr_local = AstExprLocal::new(Location::default(), &mut local, false);

    assert_eq!(
      json_ref(&mut expr_local),
      r#"{"type":"AstExprLocal","location":"0,0 - 0,0","local":{"luauType":null,"name":"foo","isConst":false,"type":"AstLocal","location":"0,0 - 0,0"}}"#
    );
  }
}

mod ast_json_encoder_encode_ast_expr_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:305:ast_json_encoder_encode_ast_expr_table`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstExpr (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParseExpr (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AstExprTable (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprConstantBool (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprConstantString (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprGlobal (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_expr_table

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_expr_table() {
    let mut fixture = JsonEncoderFixture::new();
    let expr = fixture.expect_parse_expr("{true, key=true, [key2]=true}");

    assert_eq!(
      json(expr),
      r#"{"type":"AstExprTable","location":"0,4 - 0,33","items":[{"type":"AstExprTableItem","kind":"item","value":{"type":"AstExprConstantBool","location":"0,5 - 0,9","value":true}},{"type":"AstExprTableItem","kind":"record","key":{"type":"AstExprConstantString","location":"0,11 - 0,14","value":"key"},"value":{"type":"AstExprConstantBool","location":"0,15 - 0,19","value":true}},{"type":"AstExprTableItem","kind":"general","key":{"type":"AstExprGlobal","location":"0,22 - 0,26","global":"key2"},"value":{"type":"AstExprConstantBool","location":"0,28 - 0,32","value":true}}]}"#
    );
  }
}

mod ast_json_encoder_encode_ast_expr_type_assertion {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:335:ast_json_encoder_encode_ast_expr_type_assertion`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstExpr (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParseExpr (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AstExprTypeAssertion (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprGlobal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypeReference (Ast/include/Luau/Ast.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_expr_type_assertion

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_expr_type_assertion() {
    let mut fixture = JsonEncoderFixture::new();
    let expr = fixture.expect_parse_expr("b :: any");

    assert_eq!(
      json(expr),
      r#"{"type":"AstExprTypeAssertion","location":"0,4 - 0,12","expr":{"type":"AstExprGlobal","location":"0,4 - 0,5","global":"b"},"annotation":{"type":"AstTypeReference","location":"0,9 - 0,12","name":"any","nameLocation":"0,9 - 0,12","parameters":[]}}"#
    );
  }
}

mod ast_json_encoder_encode_ast_expr_unary {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:315:ast_json_encoder_encode_ast_expr_unary`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstExpr (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParseExpr (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AstExprUnary (Ast/include/Luau/Ast.h)
  //!   - calls -> method BcInstHelper::op (Bytecode/include/Luau/BytecodeOps.h)
  //!   - type_ref -> record AstExprGlobal (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_expr_unary

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_expr_unary() {
    let mut fixture = JsonEncoderFixture::new();
    let expr = fixture.expect_parse_expr("-b");

    assert_eq!(
      json(expr),
      r#"{"type":"AstExprUnary","location":"0,4 - 0,6","op":"Minus","expr":{"type":"AstExprGlobal","location":"0,5 - 0,6","global":"b"}}"#
    );
  }
}

mod ast_json_encoder_encode_ast_expr_varargs {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:257:ast_json_encoder_encode_ast_expr_varargs`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstExprVarargs (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_expr_varargs

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_expr_varargs() {
    let mut varargs = AstExprVarargs::new(Location::default());

    assert_eq!(
      json_ref(&mut varargs),
      r#"{"type":"AstExprVarargs","location":"0,0 - 0,0"}"#
    );
  }
}

mod ast_json_encoder_encode_ast_generic_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:594:ast_json_encoder_encode_ast_generic_type`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParse (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AstStatAssign (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprGlobal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprFunction (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstGenericType (Ast/include/Luau/Ast.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_generic_type

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_generic_type() {
    let mut fixture = JsonEncoderFixture::new();
    let root = fixture.expect_parse(
      r#"
        a = function<b, c>()
        end
    "#,
    );

    assert_eq!(1, unsafe { (*root).body.size });

    assert_eq!(
      json(unsafe { block_statement(root, 0) }),
      r#"{"type":"AstStatAssign","location":"1,8 - 2,11","vars":[{"type":"AstExprGlobal","location":"1,8 - 1,9","global":"a"}],"values":[{"type":"AstExprFunction","location":"1,12 - 2,11","attributes":[],"generics":[{"type":"AstGenericType","name":"b"},{"type":"AstGenericType","name":"c"}],"genericPacks":[],"args":[],"vararg":false,"vararg_location":"0,0 - 0,0","body":{"type":"AstStatBlock","location":"1,28 - 2,8","hasEnd":true,"body":[]},"functionDepth":1,"debugname":""}]}"#
    );
  }
}

mod ast_json_encoder_encode_ast_generic_type_pack {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:623:ast_json_encoder_encode_ast_generic_type_pack`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParse (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AstStatAssign (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprGlobal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprFunction (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstGenericTypePack (Ast/include/Luau/Ast.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_generic_type_pack

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_generic_type_pack() {
    let mut fixture = JsonEncoderFixture::new();
    let root = fixture.expect_parse(
      r#"
        a = function<b..., c...>()
        end
    "#,
    );

    assert_eq!(1, unsafe { (*root).body.size });

    assert_eq!(
      json(unsafe { block_statement(root, 0) }),
      r#"{"type":"AstStatAssign","location":"1,8 - 2,11","vars":[{"type":"AstExprGlobal","location":"1,8 - 1,9","global":"a"}],"values":[{"type":"AstExprFunction","location":"1,12 - 2,11","attributes":[],"generics":[],"genericPacks":[{"type":"AstGenericTypePack","name":"b"},{"type":"AstGenericTypePack","name":"c"}],"args":[],"vararg":false,"vararg_location":"0,0 - 0,0","body":{"type":"AstStatBlock","location":"1,34 - 2,8","hasEnd":true,"body":[]},"functionDepth":1,"debugname":""}]}"#
    );
  }
}

mod ast_json_encoder_encode_ast_generic_type_pack_with_default {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:638:ast_json_encoder_encode_ast_generic_type_pack_with_default`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParse (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record AstStatTypeAlias (Ast/include/Luau/Ast.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstGenericTypePack (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypePackVariadic (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypeReference (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_generic_type_pack_with_default

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_generic_type_pack_with_default() {
    let mut fixture = JsonEncoderFixture::new();
    let root = fixture.expect_parse(
      r#"
        type Foo<X... = ...string> = any
    "#,
    );

    assert_eq!(1, unsafe { (*root).body.size });

    assert_eq!(
      json(unsafe { block_statement(root, 0) }),
      r#"{"type":"AstStatTypeAlias","location":"1,8 - 1,40","name":"Foo","generics":[],"genericPacks":[{"type":"AstGenericTypePack","name":"X","luauType":{"type":"AstTypePackVariadic","location":"1,24 - 1,33","variadicType":{"type":"AstTypeReference","location":"1,27 - 1,33","name":"string","nameLocation":"1,27 - 1,33","parameters":[]}}}],"value":{"type":"AstTypeReference","location":"1,37 - 1,40","name":"any","nameLocation":"1,37 - 1,40","parameters":[]},"exported":false}"#
    );
  }
}

mod ast_json_encoder_encode_ast_generic_type_with_default {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:609:ast_json_encoder_encode_ast_generic_type_with_default`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParse (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record AstStatTypeAlias (Ast/include/Luau/Ast.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstGenericType (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypeReference (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_generic_type_with_default

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_generic_type_with_default() {
    let mut fixture = JsonEncoderFixture::new();
    let root = fixture.expect_parse(
      r#"
        type Foo<X = string> = X
    "#,
    );

    assert_eq!(1, unsafe { (*root).body.size });

    assert_eq!(
      json(unsafe { block_statement(root, 0) }),
      r#"{"type":"AstStatTypeAlias","location":"1,8 - 1,32","name":"Foo","generics":[{"type":"AstGenericType","name":"X","luauType":{"type":"AstTypeReference","location":"1,21 - 1,27","name":"string","nameLocation":"1,21 - 1,27","parameters":[]}}],"genericPacks":[],"value":{"type":"AstTypeReference","location":"1,31 - 1,32","name":"X","nameLocation":"1,31 - 1,32","parameters":[]},"exported":false}"#
    );
  }
}

mod ast_json_encoder_encode_ast_stat_block {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:96:ast_json_encoder_encode_ast_stat_block`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstName (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstArray (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExpr (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstStat (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_stat_block

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_stat_block() {
    let mut astlocal = AstLocal::new(
      ast_name(c"a_local"),
      Location::default(),
      null_mut(),
      0,
      0,
      null_mut(),
      false,
    );
    let mut vars = [&mut astlocal as *mut AstLocal];
    let mut local = AstStatLocal::new(
      Location::default(),
      array(&mut vars),
      AstArray::default(),
      None,
      false,
    );
    let mut body = [&mut local as *mut AstStatLocal as *mut AstStat];
    let mut block = AstStatBlock::new(Location::default(), array(&mut body), true);

    assert_eq!(
      json_ref(&mut block),
      r#"{"type":"AstStatBlock","location":"0,0 - 0,0","hasEnd":true,"body":[{"type":"AstStatLocal","location":"0,0 - 0,0","vars":[{"luauType":null,"name":"a_local","isConst":false,"type":"AstLocal","location":"0,0 - 0,0"}],"values":[]}]}"#
    );
  }
}

mod ast_json_encoder_encode_ast_stat_break {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:393:ast_json_encoder_encode_ast_stat_break`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStat (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParseStatement (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AstStatWhile (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprConstantBool (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstStatBreak (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_stat_break

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_stat_break() {
    let mut fixture = JsonEncoderFixture::new();
    let statement = fixture.expect_parse_statement("while true do break end");

    assert_eq!(
      json(statement),
      r#"{"type":"AstStatWhile","location":"0,0 - 0,23","condition":{"type":"AstExprConstantBool","location":"0,6 - 0,10","value":true},"body":{"type":"AstStatBlock","location":"0,13 - 0,20","hasEnd":true,"body":[{"type":"AstStatBreak","location":"0,14 - 0,19"}]},"hasDo":true}"#
    );
  }
}

mod ast_json_encoder_encode_ast_stat_compound_assign {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:437:ast_json_encoder_encode_ast_stat_compound_assign`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStat (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParseStatement (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AstStatCompoundAssign (Ast/include/Luau/Ast.h)
  //!   - calls -> method BcInstHelper::op (Bytecode/include/Luau/BytecodeOps.h)
  //!   - type_ref -> record AstExprGlobal (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_stat_compound_assign

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_stat_compound_assign() {
    let mut fixture = JsonEncoderFixture::new();
    let statement = fixture.expect_parse_statement("a += b");

    assert_eq!(
      json(statement),
      r#"{"type":"AstStatCompoundAssign","location":"0,0 - 0,6","op":"Add","var":{"type":"AstExprGlobal","location":"0,0 - 0,1","global":"a"},"value":{"type":"AstExprGlobal","location":"0,5 - 0,6","global":"b"}}"#
    );
  }
}

mod ast_json_encoder_encode_ast_stat_continue {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:403:ast_json_encoder_encode_ast_stat_continue`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStat (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParseStatement (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AstStatWhile (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprConstantBool (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstStatContinue (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_stat_continue

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_stat_continue() {
    let mut fixture = JsonEncoderFixture::new();
    let statement = fixture.expect_parse_statement("while true do continue end");

    assert_eq!(
      json(statement),
      r#"{"type":"AstStatWhile","location":"0,0 - 0,26","condition":{"type":"AstExprConstantBool","location":"0,6 - 0,10","value":true},"body":{"type":"AstStatBlock","location":"0,13 - 0,23","hasEnd":true,"body":[{"type":"AstStatContinue","location":"0,14 - 0,22"}]},"hasDo":true}"#
    );
  }
}

mod ast_json_encoder_encode_ast_stat_declare_class {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:500:ast_json_encoder_encode_ast_stat_declare_class`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParse (tests/AstJsonEncoder.test.cpp)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record Bar (tests/Variant.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstTypeReference (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypeFunction (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypeList (Ast/include/Luau/Ast.h)
  //!   - type_ref -> type_alias AstArgumentName (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypePackExplicit (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_stat_declare_class

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_stat_declare_class() {
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

    assert_eq!(2, unsafe { (*root).body.size });

    assert_eq!(
      json(unsafe { block_statement(root, 0) }),
      r#"{"type":"AstStatDeclareClass","location":"1,28 - 4,11","name":"Foo","props":[{"name":"prop","nameLocation":"2,12 - 2,16","type":"AstDeclaredClassProp","luauType":{"type":"AstTypeReference","location":"2,18 - 2,24","name":"number","nameLocation":"2,18 - 2,24","parameters":[]},"location":"2,12 - 2,24"},{"name":"method","nameLocation":"3,21 - 3,27","type":"AstDeclaredClassProp","luauType":{"type":"AstTypeFunction","location":"3,12 - 3,54","attributes":[],"generics":[],"genericPacks":[],"argTypes":{"type":"AstTypeList","types":[{"type":"AstTypeReference","location":"3,39 - 3,45","name":"number","nameLocation":"3,39 - 3,45","parameters":[]}]},"argNames":[{"type":"AstArgumentName","name":"foo","location":"3,34 - 3,37"}],"returnTypes":{"type":"AstTypePackExplicit","location":"3,48 - 3,54","typeList":{"type":"AstTypeList","types":[{"type":"AstTypeReference","location":"3,48 - 3,54","name":"string","nameLocation":"3,48 - 3,54","parameters":[]}]}}},"location":"3,12 - 3,54"}],"indexer":null}"#
    );
    assert_eq!(
      json(unsafe { block_statement(root, 1) }),
      r#"{"type":"AstStatDeclareClass","location":"6,28 - 8,11","name":"Bar","superName":"Foo","props":[{"name":"prop2","nameLocation":"7,12 - 7,17","type":"AstDeclaredClassProp","luauType":{"type":"AstTypeReference","location":"7,19 - 7,25","name":"string","nameLocation":"7,19 - 7,25","parameters":[]},"location":"7,12 - 7,25"}],"indexer":null}"#
    );
  }
}

mod ast_json_encoder_encode_ast_stat_declare_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:469:ast_json_encoder_encode_ast_stat_declare_function`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStat (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParseStatement (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AstStatDeclareFunction (Ast/include/Luau/Ast.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstTypeList (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypeReference (Ast/include/Luau/Ast.h)
  //!   - type_ref -> type_alias AstArgumentName (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypePackExplicit (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_stat_declare_function

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_stat_declare_function() {
    let mut fixture = JsonEncoderFixture::new();
    let statement = fixture.expect_parse_statement("declare function foo(x: number): string");

    assert_eq!(
      json(statement),
      r#"{"type":"AstStatDeclareFunction","location":"0,0 - 0,39","attributes":[],"name":"foo","nameLocation":"0,17 - 0,20","params":{"type":"AstTypeList","types":[{"type":"AstTypeReference","location":"0,24 - 0,30","name":"number","nameLocation":"0,24 - 0,30","parameters":[]}]},"paramNames":[{"type":"AstArgumentName","name":"x","location":"0,21 - 0,22"}],"vararg":false,"vararg_location":"0,0 - 0,0","retTypes":{"type":"AstTypePackExplicit","location":"0,33 - 0,39","typeList":{"type":"AstTypeList","types":[{"type":"AstTypeReference","location":"0,33 - 0,39","name":"string","nameLocation":"0,33 - 0,39","parameters":[]}]}},"generics":[],"genericPacks":[]}"#
    );
  }
}

mod ast_json_encoder_encode_ast_stat_declare_function_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:478:ast_json_encoder_encode_ast_stat_declare_function_2`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStat (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParseStatement (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AstStatDeclareFunction (Ast/include/Luau/Ast.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstTypeList (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypeReference (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypePackVariadic (Ast/include/Luau/Ast.h)
  //!   - type_ref -> type_alias AstArgumentName (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypePackExplicit (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_stat_declare_function_2

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_stat_declare_function2() {
    let mut fixture = JsonEncoderFixture::new();
    let statement =
      fixture.expect_parse_statement("declare function foo(x: number, ...: string): string");

    assert_eq!(
      json(statement),
      r#"{"type":"AstStatDeclareFunction","location":"0,0 - 0,52","attributes":[],"name":"foo","nameLocation":"0,17 - 0,20","params":{"type":"AstTypeList","types":[{"type":"AstTypeReference","location":"0,24 - 0,30","name":"number","nameLocation":"0,24 - 0,30","parameters":[]}],"tailType":{"type":"AstTypePackVariadic","location":"0,37 - 0,43","variadicType":{"type":"AstTypeReference","location":"0,37 - 0,43","name":"string","nameLocation":"0,37 - 0,43","parameters":[]}}},"paramNames":[{"type":"AstArgumentName","name":"x","location":"0,21 - 0,22"}],"vararg":true,"vararg_location":"0,32 - 0,35","retTypes":{"type":"AstTypePackExplicit","location":"0,46 - 0,52","typeList":{"type":"AstTypeList","types":[{"type":"AstTypeReference","location":"0,46 - 0,52","name":"string","nameLocation":"0,46 - 0,52","parameters":[]}]}},"generics":[],"genericPacks":[]}"#
    );
  }
}

mod ast_json_encoder_encode_ast_stat_for {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:413:ast_json_encoder_encode_ast_stat_for`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStat (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParseStatement (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AstStatFor (Ast/include/Luau/Ast.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstLocal (Ast/include/Luau/Ast.h)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - type_ref -> record AstExprConstantNumber (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_stat_for

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_stat_for() {
    let mut fixture = JsonEncoderFixture::new();
    let statement = fixture.expect_parse_statement("for a=0,1 do end");

    assert_eq!(
      json(statement),
      r#"{"type":"AstStatFor","location":"0,0 - 0,16","var":{"luauType":null,"name":"a","isConst":false,"type":"AstLocal","location":"0,4 - 0,5"},"from":{"type":"AstExprConstantNumber","location":"0,6 - 0,7","value":0},"to":{"type":"AstExprConstantNumber","location":"0,8 - 0,9","value":1},"body":{"type":"AstStatBlock","location":"0,12 - 0,13","hasEnd":true,"body":[]},"hasDo":true}"#
    );
  }
}

mod ast_json_encoder_encode_ast_stat_for_in {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:425:ast_json_encoder_encode_ast_stat_for_in`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStat (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParseStatement (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AstStatForIn (Ast/include/Luau/Ast.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprGlobal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_stat_for_in

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_stat_for_in() {
    let mut fixture = JsonEncoderFixture::new();
    let statement = fixture.expect_parse_statement("for a in b do end");

    assert_eq!(
      json(statement),
      r#"{"type":"AstStatForIn","location":"0,0 - 0,17","vars":[{"luauType":null,"name":"a","isConst":false,"type":"AstLocal","location":"0,4 - 0,5"}],"values":[{"type":"AstExprGlobal","location":"0,9 - 0,10","global":"b"}],"body":{"type":"AstStatBlock","location":"0,13 - 0,14","hasEnd":true,"body":[]},"hasIn":true,"hasDo":true}"#
    );
  }
}

mod ast_json_encoder_encode_ast_stat_if {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:363:ast_json_encoder_encode_ast_stat_if`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStat (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParseStatement (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AstStatIf (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprConstantBool (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_stat_if

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_stat_if() {
    let mut fixture = JsonEncoderFixture::new();
    let statement = fixture.expect_parse_statement("if true then else end");

    assert_eq!(
      json(statement),
      r#"{"type":"AstStatIf","location":"0,0 - 0,21","condition":{"type":"AstExprConstantBool","location":"0,3 - 0,7","value":true},"thenbody":{"type":"AstStatBlock","location":"0,12 - 0,13","hasEnd":true,"body":[]},"elsebody":{"type":"AstStatBlock","location":"0,17 - 0,18","hasEnd":true,"body":[]},"hasThen":true}"#
    );
  }
}

mod ast_json_encoder_encode_ast_stat_local_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:447:ast_json_encoder_encode_ast_stat_local_function`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStat (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParseStatement (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AstStatLocalFunction (Ast/include/Luau/Ast.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprFunction (Ast/include/Luau/Ast.h)
  //!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstStatReturn (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_stat_local_function

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_stat_local_function() {
    let mut fixture = JsonEncoderFixture::new();
    let statement = fixture.expect_parse_statement("local function a(b) return end");

    assert_eq!(
      json(statement),
      r#"{"type":"AstStatLocalFunction","location":"0,0 - 0,30","name":{"luauType":null,"name":"a","isConst":false,"type":"AstLocal","location":"0,15 - 0,16"},"func":{"type":"AstExprFunction","location":"0,0 - 0,30","attributes":[],"generics":[],"genericPacks":[],"args":[{"luauType":null,"name":"b","isConst":false,"type":"AstLocal","location":"0,17 - 0,18"}],"vararg":false,"vararg_location":"0,0 - 0,0","body":{"type":"AstStatBlock","location":"0,19 - 0,27","hasEnd":true,"body":[{"type":"AstStatReturn","location":"0,20 - 0,26","list":[]}]},"functionDepth":1,"debugname":"a"}}"#
    );
  }
}

mod ast_json_encoder_encode_ast_stat_repeat {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:383:ast_json_encoder_encode_ast_stat_repeat`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStat (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParseStatement (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AstStatRepeat (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprConstantBool (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_stat_repeat

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_stat_repeat() {
    let mut fixture = JsonEncoderFixture::new();
    let statement = fixture.expect_parse_statement("repeat until true");

    assert_eq!(
      json(statement),
      r#"{"type":"AstStatRepeat","location":"0,0 - 0,17","condition":{"type":"AstExprConstantBool","location":"0,13 - 0,17","value":true},"body":{"type":"AstStatBlock","location":"0,6 - 0,7","hasEnd":true,"body":[]}}"#
    );
  }
}

mod ast_json_encoder_encode_ast_stat_type_alias {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:459:ast_json_encoder_encode_ast_stat_type_alias`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStat (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParseStatement (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AstStatTypeAlias (Ast/include/Luau/Ast.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstTypeReference (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_stat_type_alias

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_stat_type_alias() {
    let mut fixture = JsonEncoderFixture::new();
    let statement = fixture.expect_parse_statement("type A = B");

    assert_eq!(
      json(statement),
      r#"{"type":"AstStatTypeAlias","location":"0,0 - 0,10","name":"A","generics":[],"genericPacks":[],"value":{"type":"AstTypeReference","location":"0,9 - 0,10","name":"B","nameLocation":"0,9 - 0,10","parameters":[]},"exported":false}"#
    );
  }
}

mod ast_json_encoder_encode_ast_stat_while {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:373:ast_json_encoder_encode_ast_stat_while`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStat (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParseStatement (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AstStatWhile (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprConstantBool (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_stat_while

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_stat_while() {
    let mut fixture = JsonEncoderFixture::new();
    let statement = fixture.expect_parse_statement("while true do end");

    assert_eq!(
      json(statement),
      r#"{"type":"AstStatWhile","location":"0,0 - 0,17","condition":{"type":"AstExprConstantBool","location":"0,6 - 0,10","value":true},"body":{"type":"AstStatBlock","location":"0,13 - 0,14","hasEnd":true,"body":[]},"hasDo":true}"#
    );
  }
}

mod ast_json_encoder_encode_ast_type_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:564:ast_json_encoder_encode_ast_type_error`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record ParseResult (Ast/include/Luau/ParseResult.h)
  //!   - calls -> method JsonEncoderFixture::parse (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AstStat (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstStatTypeAlias (Ast/include/Luau/Ast.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstTypeError (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_type_error

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_type_error() {
    let mut fixture = JsonEncoderFixture::new();
    let parse_result = fixture.parse("type T = ");
    assert_eq!(1, unsafe { (*parse_result.root).body.size });

    let statement = unsafe { block_statement(parse_result.root, 0) };

    assert_eq!(
      json(statement),
      r#"{"type":"AstStatTypeAlias","location":"0,0 - 0,9","name":"T","generics":[],"genericPacks":[],"value":{"type":"AstTypeError","location":"0,8 - 0,9","types":[],"messageIndex":0},"exported":false}"#
    );
  }
}

mod ast_json_encoder_encode_ast_type_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:555:ast_json_encoder_encode_ast_type_function`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStat (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParseStatement (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record AstStatTypeAlias (Ast/include/Luau/Ast.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstTypeFunction (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypeList (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypeReference (Ast/include/Luau/Ast.h)
  //!   - type_ref -> type_alias AstArgumentName (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypePackExplicit (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_type_function

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_type_function() {
    let mut fixture = JsonEncoderFixture::new();
    let statement =
      fixture.expect_parse_statement(r#"type fun = (string, bool, named: number) -> ()"#);

    assert_eq!(
      json(statement),
      r#"{"type":"AstStatTypeAlias","location":"0,0 - 0,46","name":"fun","generics":[],"genericPacks":[],"value":{"type":"AstTypeFunction","location":"0,11 - 0,46","attributes":[],"generics":[],"genericPacks":[],"argTypes":{"type":"AstTypeList","types":[{"type":"AstTypeReference","location":"0,12 - 0,18","name":"string","nameLocation":"0,12 - 0,18","parameters":[]},{"type":"AstTypeReference","location":"0,20 - 0,24","name":"bool","nameLocation":"0,20 - 0,24","parameters":[]},{"type":"AstTypeReference","location":"0,33 - 0,39","name":"number","nameLocation":"0,33 - 0,39","parameters":[]}]},"argNames":[null,null,{"type":"AstArgumentName","name":"named","location":"0,26 - 0,31"}],"returnTypes":{"type":"AstTypePackExplicit","location":"0,44 - 0,46","typeList":{"type":"AstTypeList","types":[]}}},"exported":false}"#
    );
  }
}

mod ast_json_encoder_encode_ast_type_optional {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:652:ast_json_encoder_encode_ast_type_optional`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParse (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record AstStatTypeAlias (Ast/include/Luau/Ast.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstTypeUnion (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypeReference (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypeOptional (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_type_optional

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_type_optional() {
    let mut fixture = JsonEncoderFixture::new();
    let root = fixture.expect_parse(
      r#"
            type Foo = string?
        "#,
    );

    assert_eq!(1, unsafe { (*root).body.size });

    assert_eq!(
      json(unsafe { block_statement(root, 0) }),
      r#"{"type":"AstStatTypeAlias","location":"1,12 - 1,30","name":"Foo","generics":[],"genericPacks":[],"value":{"type":"AstTypeUnion","location":"1,23 - 1,30","types":[{"type":"AstTypeReference","location":"1,23 - 1,29","name":"string","nameLocation":"1,23 - 1,29","parameters":[]},{"type":"AstTypeOptional","location":"1,29 - 1,30"}]},"exported":false}"#
    );
  }
}

mod ast_json_encoder_encode_ast_type_pack_explicit {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:577:ast_json_encoder_encode_ast_type_pack_explicit`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParse (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypeReference (Ast/include/Luau/Ast.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstTypePackExplicit (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypeList (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstLocal (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_ast_type_pack_explicit

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_ast_type_pack_explicit() {
    let mut fixture = JsonEncoderFixture::new();
    let root = fixture.expect_parse(
      r#"
        type A<T...> = () -> T...
        local a: A<(number, string)>
    "#,
    );

    assert_eq!(2, unsafe { (*root).body.size });

    assert_eq!(
      json(unsafe { block_statement(root, 1) }),
      r#"{"type":"AstStatLocal","location":"2,8 - 2,36","vars":[{"luauType":{"type":"AstTypeReference","location":"2,17 - 2,36","name":"A","nameLocation":"2,17 - 2,18","parameters":[{"type":"AstTypePackExplicit","location":"2,19 - 2,20","typeList":{"type":"AstTypeList","types":[{"type":"AstTypeReference","location":"2,20 - 2,26","name":"number","nameLocation":"2,20 - 2,26","parameters":[]},{"type":"AstTypeReference","location":"2,28 - 2,34","name":"string","nameLocation":"2,28 - 2,34","parameters":[]}]}}]},"name":"a","isConst":false,"type":"AstLocal","location":"2,14 - 2,15"}],"values":[]}"#
    );
  }
}

mod ast_json_encoder_encode_constants {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:60:ast_json_encoder_encode_constants`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstExprConstantNil (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstExprConstantBool (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprConstantNumber (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstArray (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprConstantString (Ast/include/Luau/Ast.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_constants

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_constants() {
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

    let mut char_string = [
      b'a' as c_char,
      0x1d as c_char,
      0 as c_char,
      b'\\' as c_char,
      b'"' as c_char,
      b'b' as c_char,
    ];
    let mut needs_escaping = AstExprConstantString::new(
      Location::default(),
      c_char_array(&mut char_string),
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
  }
}

mod ast_json_encoder_encode_indexed_type_literal {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:545:ast_json_encoder_encode_indexed_type_literal`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStat (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParseStatement (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record AstStatTypeAlias (Ast/include/Luau/Ast.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstTypeTable (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypeReference (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypeSingletonBool (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_indexed_type_literal

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_indexed_type_literal() {
    let mut fixture = JsonEncoderFixture::new();
    let statement = fixture.expect_parse_statement(r#"type StringSet = { [string]: true }"#);

    assert_eq!(
      json(statement),
      r#"{"type":"AstStatTypeAlias","location":"0,0 - 0,35","name":"StringSet","generics":[],"genericPacks":[],"value":{"type":"AstTypeTable","location":"0,17 - 0,35","props":[],"indexer":{"location":"0,19 - 0,33","index_type":{"type":"AstTypeReference","location":"0,20 - 0,26","name":"string","nameLocation":"0,20 - 0,26","parameters":[]},"result_type":{"type":"AstTypeSingletonBool","location":"0,29 - 0,33","value":true}}},"exported":false}"#
    );
  }
}

mod ast_json_encoder_encode_table_array {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:147:ast_json_encoder_encode_table_array`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParse (tests/AstJsonEncoder.test.cpp)
  //!   - type_ref -> record AstStatTypeAlias (Ast/include/Luau/Ast.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstTypeTable (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypeReference (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_table_array

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_table_array() {
    let mut fixture = JsonEncoderFixture::new();
    let root = fixture.expect_parse(r#"type X = {string}"#);

    assert_eq!(
      json(root),
      desugared_array_type_reference_is_empty(
        r#"{"type":"AstStatBlock","location":"0,0 - 0,17","hasEnd":true,"body":[{"type":"AstStatTypeAlias","location":"0,0 - 0,17","name":"X","generics":[],"genericPacks":[],"value":{"type":"AstTypeTable","location":"0,9 - 0,17","props":[],"indexer":{"location":"0,10 - 0,16","index_type":{"type":"AstTypeReference","location":"0,9 - 0,9","name":"number","nameLocation":"0,9 - 0,9","parameters":[]},"result_type":{"type":"AstTypeReference","location":"0,10 - 0,16","name":"string","nameLocation":"0,10 - 0,16","parameters":[]}}},"exported":false}]}"#,
        r#"{"type":"AstStatBlock","location":"0,0 - 0,17","hasEnd":true,"body":[{"type":"AstStatTypeAlias","location":"0,0 - 0,17","name":"X","generics":[],"genericPacks":[],"value":{"type":"AstTypeTable","location":"0,9 - 0,17","props":[],"indexer":{"location":"0,10 - 0,16","index_type":{"type":"AstTypeReference","location":"0,10 - 0,16","name":"number","nameLocation":"0,10 - 0,16","parameters":[]},"result_type":{"type":"AstTypeReference","location":"0,10 - 0,16","name":"string","nameLocation":"0,10 - 0,16","parameters":[]}}},"exported":false}]}"#,
      )
    );
  }
}

mod ast_json_encoder_encode_table_indexer {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:170:ast_json_encoder_encode_table_indexer`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParse (tests/AstJsonEncoder.test.cpp)
  //!   - type_ref -> record AstStatTypeAlias (Ast/include/Luau/Ast.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstTypeTable (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypeReference (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_table_indexer

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_table_indexer() {
    let mut fixture = JsonEncoderFixture::new();
    let root = fixture.expect_parse(r#"type X = {string}"#);

    assert_eq!(
      json(root),
      desugared_array_type_reference_is_empty(
        r#"{"type":"AstStatBlock","location":"0,0 - 0,17","hasEnd":true,"body":[{"type":"AstStatTypeAlias","location":"0,0 - 0,17","name":"X","generics":[],"genericPacks":[],"value":{"type":"AstTypeTable","location":"0,9 - 0,17","props":[],"indexer":{"location":"0,10 - 0,16","index_type":{"type":"AstTypeReference","location":"0,9 - 0,9","name":"number","nameLocation":"0,9 - 0,9","parameters":[]},"result_type":{"type":"AstTypeReference","location":"0,10 - 0,16","name":"string","nameLocation":"0,10 - 0,16","parameters":[]}}},"exported":false}]}"#,
        r#"{"type":"AstStatBlock","location":"0,0 - 0,17","hasEnd":true,"body":[{"type":"AstStatTypeAlias","location":"0,0 - 0,17","name":"X","generics":[],"genericPacks":[],"value":{"type":"AstTypeTable","location":"0,9 - 0,17","props":[],"indexer":{"location":"0,10 - 0,16","index_type":{"type":"AstTypeReference","location":"0,10 - 0,16","name":"number","nameLocation":"0,10 - 0,16","parameters":[]},"result_type":{"type":"AstTypeReference","location":"0,10 - 0,16","name":"string","nameLocation":"0,10 - 0,16","parameters":[]}}},"exported":false}]}"#,
      )
    );
  }
}

mod ast_json_encoder_encode_tables {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:122:ast_json_encoder_encode_tables`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParse (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypeTable (Ast/include/Luau/Ast.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstTableProp (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypeReference (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprTable (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprConstantString (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprConstantNumber (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_tables

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_tables() {
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
      r#"{"type":"AstStatBlock","location":"0,0 - 6,4","hasEnd":true,"body":[{"type":"AstStatLocal","location":"1,8 - 5,9","vars":[{"luauType":{"type":"AstTypeTable","location":"1,17 - 3,9","props":[{"name":"foo","type":"AstTableProp","location":"2,12 - 2,15","prop_type":{"type":"AstTypeReference","location":"2,17 - 2,23","name":"number","nameLocation":"2,17 - 2,23","parameters":[]}}],"indexer":null},"name":"x","isConst":false,"type":"AstLocal","location":"1,14 - 1,15"}],"values":[{"type":"AstExprTable","location":"3,12 - 5,9","items":[{"type":"AstExprTableItem","kind":"record","key":{"type":"AstExprConstantString","location":"4,12 - 4,15","value":"foo"},"value":{"type":"AstExprConstantNumber","location":"4,18 - 4,21","value":123}}]}]}]}"#
    );
  }
}

mod ast_json_encoder_encode_type_literal {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:533:ast_json_encoder_encode_type_literal`
  //! Source: `tests/AstJsonEncoder.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/AstJsonEncoder.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/AstJsonEncoder.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStat (Ast/include/Luau/Ast.h)
  //!   - calls -> method JsonEncoderFixture::expectParseStatement (tests/AstJsonEncoder.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AstStatTypeAlias (Ast/include/Luau/Ast.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstTypeTable (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTableProp (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypeUnion (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypeSingletonString (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypeSingletonBool (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item ast_json_encoder_encode_type_literal

  #[cfg(test)]
  use super::ast_json_encoder_support::*;

  #[cfg(test)]
  #[test]
  fn ast_json_encoder_encode_type_literal() {
    let mut fixture = JsonEncoderFixture::new();
    let statement = fixture.expect_parse_statement(
      r#"type Action = { strings: "A" | "B" | "C", mixed: "This" | "That" | true }"#,
    );

    assert_eq!(
      json(statement),
      r#"{"type":"AstStatTypeAlias","location":"0,0 - 0,73","name":"Action","generics":[],"genericPacks":[],"value":{"type":"AstTypeTable","location":"0,14 - 0,73","props":[{"name":"strings","type":"AstTableProp","location":"0,16 - 0,23","prop_type":{"type":"AstTypeUnion","location":"0,25 - 0,40","types":[{"type":"AstTypeSingletonString","location":"0,25 - 0,28","value":"A"},{"type":"AstTypeSingletonString","location":"0,31 - 0,34","value":"B"},{"type":"AstTypeSingletonString","location":"0,37 - 0,40","value":"C"}]}},{"name":"mixed","type":"AstTableProp","location":"0,42 - 0,47","prop_type":{"type":"AstTypeUnion","location":"0,49 - 0,71","types":[{"type":"AstTypeSingletonString","location":"0,49 - 0,55","value":"This"},{"type":"AstTypeSingletonString","location":"0,58 - 0,64","value":"That"},{"type":"AstTypeSingletonBool","location":"0,67 - 0,71","value":true}]}}],"indexer":null},"exported":false}"#
    );
  }
}

pub(crate) mod ast_json_encoder_support {

  pub use core::{ffi::c_char, ptr::null_mut};

  pub use ulua_ast::{
    enums::{constant_number_parse_result::ConstantNumberParseResult, quote_style_ast::QuoteStyle},
    records::{
      ast_array::AstArray, ast_expr::AstExpr, ast_expr_constant_bool::AstExprConstantBool,
      ast_expr_constant_nil::AstExprConstantNil, ast_expr_constant_number::AstExprConstantNumber,
      ast_expr_constant_string::AstExprConstantString, ast_expr_global::AstExprGlobal,
      ast_expr_group::AstExprGroup, ast_expr_local::AstExprLocal, ast_expr_varargs::AstExprVarargs,
      ast_local::AstLocal, ast_node::AstNode, ast_stat::AstStat, ast_stat_assign::AstStatAssign,
      ast_stat_block::AstStatBlock, ast_stat_local::AstStatLocal, location::Location,
      parse_options::ParseOptions, parser::Parser,
    },
    rtti::ast_node_as,
  };
  pub use ulua_unit_test::{
    functions::{
      ast_json_encoder_array::array, ast_json_encoder_ast_name::ast_name,
      ast_json_encoder_block_statement::block_statement,
      ast_json_encoder_c_char_array::c_char_array,
      ast_json_encoder_desugared_array_type_reference_is_empty::desugared_array_type_reference_is_empty,
      ast_json_encoder_json::json, ast_json_encoder_json_ref::json_ref,
    },
    records::json_encoder_fixture::JsonEncoderFixture,
  };
}
