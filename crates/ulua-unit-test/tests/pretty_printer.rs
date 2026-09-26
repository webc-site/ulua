use ulua_analysis::type_aliases::module_name_type::ModuleName;
extern crate alloc;

// 196 个用例共用导入：原 319 处 test fn 体内逐例重复的 use 统一上提至此
// （源头 cpp `tests/PrettyPrinter.test.cpp`，cpp 侧即为整文件共享的 using 声明）。
use alloc::string::String;

use ulua_ast::{
  functions::{
    pretty_print_pretty_printer::{
      pretty_print_ast_stat_block, pretty_print_string_view_parse_options_bool_bool as pp,
    },
    pretty_print_with_types_pretty_printer::pretty_print_with_types_ast_stat_block,
    to_string_pretty_printer::to_string_ast_node,
  },
  records::{
    allocator::Allocator, ast_name_table::AstNameTable, ast_stat_local::AstStatLocal,
    parse_options::ParseOptions, parser::Parser,
  },
  rtti::ast_node_try_as_ptr,
};
use ulua_common::fflag;
use ulua_unit_test::{
  functions::ast_node_ref::PtrRef, records::fixture::Fixture,
  type_aliases::scoped_fast_flag::ScopedFastFlag,
};

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_a_table_key_can_be_the_empty_string() {
  let code = "local T = {[''] = true}";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_always_emit_a_space_after_local_keyword() {
  let code = "do local aZZZZ = Workspace.P1.Shape local bZZZZ = Enum.PartType.Cylinder end";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Ported from upstream Luau doctest.
// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_attach_types() {
  let mut fixture = Fixture::default();
  let code = String::from(
    r#"
        local s='str'
        local t={a=1,b=false}
        local function fn()
            return 10
        end
    "#,
  );
  let expected = String::from(
    r#"
        local s:string='str'
        local t:{a:number,b:boolean}={a=1,b=false}
        local function fn(): number
            return 10
        end
    "#,
  );

  assert_eq!(expected, fixture.decorate_with_types(&code));
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_binary_keywords() {
  let code = "local c = a0 ._ or b0 ._";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_binary_numbers() {
  let code = " local a = 0b0101 ";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_binary_spaces_around_tokens() {
  let _fixture = Fixture::default();
  let code = r#"
local _ =    1+1
local _ = 1   +1
local _ = 1+   1
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_compound_assignment_spaces_around_tokens() {
  let _fixture = Fixture::default();
  for code in [" a   += 1 ", " a +=   1 "] {
    assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_do_block_ending_with_semicolon() {
  let _fixture = Fixture::default();
  let code = r#"
        do
            return;
        end;
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_do_blocks() {
  let code = r#"
        foo()

        do
            local bar=baz()
            quux()
        end

        foo2()
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_double_quoted_strings() {
  let code = r#" local a = "hello world" "#;
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_elseif_chains_indent_sensibly() {
  let code = r#"
        if This then
            Once()
        elseif That then
            Another()
        elseif SecondLast then
            Third()
        else
            IfAllElseFails()
        end
    "#;

  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_emit_a_do_block_in_cases_of_potentially_ambiguous_syntax() {
  let code = r#"
        f();
        (g or f)()
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_escaped_strings() {
  let code = r#" local s='\\b\\t\\n\\\\' "#;
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_escaped_strings_2() {
  let code = r#" local s="\a\b\f\n\r\t\v\'\"\\" "#;
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_escaped_strings_newline() {
  let code = r#"
    print("foo \
        bar")
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_escaped_strings_raw() {
  let code = r#" local x = [=[\v<((do|load)file|require)\s*\(?['"]\zs[^'"]+\ze['"]]=] "#;
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

#[test]
fn pretty_printer_export() {
  let _export_value = ScopedFastFlag::new(&fflag::LuauExportValueSyntax, true);

  let mut code = r#"
export                      local version = "1.0.0"
export           const tabbed = ...
export const TAU = math.pi * 2
export local settings: Settings = getSettings()
export local a, b, c = 1, 2, 3
export local d
    "#;
  let result = pp(code, ParseOptions::default(), true, false);
  assert_eq!(code, result.code);

  code = r#"
export function add(a: number, b: number): number
    return a + b
end

export function greet(name: string): string
    return "Hello, " .. name
end

export function noop()
end

export        function tabbed(): number
    return 1
end
    "#;
  let result = pp(code, ParseOptions::default(), true, false);
  assert_eq!(code, result.code);

  code = r#"
@native
export function foo()
end

@native
export                 function tabbed_attribute()
end
    "#;
  let result = pp(code, ParseOptions::default(), true, false);
  assert_eq!(code, result.code);

  code = r#"
export local f, g

function f()
    return g()
end

function g()
    return 42
end
    "#;
  let result = pp(code, ParseOptions::default(), true, false);
  assert_eq!(code, result.code);

  code = r#"
export type Config = {
    debug: boolean,
    timeout: number,
}

export local currentConfig: Config

export function createConfig(debug: boolean, timeout: number): Config
    return {
        debug = debug,
        timeout = timeout,
    }
end
    "#;
  let result = pp(code, ParseOptions::default(), true, false);
  assert_eq!(code, result.code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_for_in_loop() {
  let code = " for k, v in ipairs(x)do end ";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_for_in_loop_spaces_around_tokens() {
  for code in [
    " for k, v in ipairs(x)   do end ",
    " for k, v    in    ipairs(x) do end ",
    " for k  ,  v in ipairs(x) do end ",
    " for k, v in next  , t  do end ",
    " for k, v in ipairs(x) do   end ",
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_for_in_single_variable() {
  let code = " for key in pairs(x) do end ";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_for_loop() {
  for code in [" for i=1,10 do end ", " for i=5,6,7 do end "] {
    assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_for_loop_spaces_around_tokens() {
  for code in [
    " for index = 1, 10 do call(index) end ",
    " for index = 1  , 10 do call(index) end ",
    " for index = 1, 10  ,  3 do call(index) end ",
    " for index = 1, 10    do call(index) end ",
    " for index = 1, 10 do call(index)    end ",
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_for_loop_stmt_semicolon() {
  let _fixture = Fixture::default();
  let code = r#"
        for i,v in ... do
        end;
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_function() {
  for code in [
    " function p(o, m, g) return 77 end ",
    " function p(o, m, g,...)  return 77 end ",
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_function_call_parentheses_multiple_args() {
  let code = " call(arg1, arg3, arg3) ";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_function_call_parentheses_multiple_args_no_space() {
  let code = " call(arg1,arg3,arg3) ";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_function_call_parentheses_multiple_args_space_before_commas() {
  let code = " call(arg1 ,arg3 ,arg3) ";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_function_call_parentheses_no_args() {
  let code = " call() ";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_function_call_parentheses_one_arg() {
  let code = " call(arg) ";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_function_call_spaces_before_parentheses() {
  let code = " call () ";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_function_call_spaces_within_parentheses() {
  let code = " call(  ) ";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_function_call_string_double_quotes() {
  let code = r#" call "string" "#;
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_function_call_string_no_space() {
  let code = " call'string' ";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_function_call_string_single_quotes() {
  let code = " call 'string' ";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_function_call_table_literal() {
  let code = r#" call { x = 1 } "#;
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_function_call_table_literal_no_space() {
  let code = r#" call{x=1} "#;
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_function_definition_semicolon() {
  let _fixture = Fixture::default();
  let code = r#"
        function foo()
        end;
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_function_spaces_around_tokens() {
  for code in [
    " function     p(o, m, ...) end ",
    " function p(   o, m, ...) end ",
    " function p(o   , m, ...) end ",
    " function p(o,   m, ...) end ",
    " function p(o, m   , ...) end ",
    " function p(o, m,   ...) end ",
    " function p(o, m, ...   ) end ",
    " function p(o, m, ...)   end ",
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_function_taking_ellipsis() {
  let code = " function F(...) end ";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Ported from upstream Luau doctest.
// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_function_type_location() {
  let mut fixture = Fixture::default();
  let code = String::from(
    r#"
        local function foo(x: number): number
         return x
        end
        local g: (number)->number = foo
    "#,
  );
  let expected = String::from(
    r#"
        local function foo(x: number): number
         return x
        end
        local g: (number)->(number)=foo
    "#,
  );

  let actual = fixture.decorate_with_types(&code);

  assert_eq!(expected, actual);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_function_with_types_spaces_around_tokens() {
  for code in [
    " function p<X, Y, Z...>(o: string, m: number, ...: any): string end ",
    " function p   <X, Y, Z...>(o: string, m: number, ...: any): string end ",
    " function p<X   , Y, Z...>(o: string, m: number, ...: any): string end ",
    " function p<X,   Y, Z...>(o: string, m: number, ...: any): string end ",
    " function p<X, Y,   Z...>(o: string, m: number, ...: any): string end ",
    " function p<X, Y, Z  ...>(o: string, m: number, ...: any): string end ",
    " function p<X, Y, Z...  >(o: string, m: number, ...: any): string end ",
    " function p<X, Y, Z...>  (o: string, m: number, ...: any): string end ",
    " function p<X, Y, Z...>(o  : string, m: number, ...: any): string end ",
    " function p<X, Y, Z...>(o:   string, m: number, ...: any): string end ",
    " function p<X, Y, Z...>(o: string  , m: number, ...: any): string end ",
    " function p<X, Y, Z...>(o: string,   m: number, ...: any): string end ",
    " function p<X, Y, Z...>(o: string, m: number,   ...: any): string end ",
    " function p<X, Y, Z...>(o: string, m: number, ...  : any): string end ",
    " function p<X, Y, Z...>(o: string, m: number, ...:   any): string end ",
    " function p<X, Y, Z...>(o: string, m: number, ...: any  ): string end ",
    " function p<X, Y, Z...>(o: string, m: number, ...: any)   :string end ",
    " function p<X, Y, Z...>(o: string, m: number, ...: any):    string end ",
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
  }
}

#[test]
fn pretty_printer_fuzzer_class() {
  let _fflag = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);
  let code = r#" class l0 end "#;

  let _result = pp(code, ParseOptions::default(), true, false);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_fuzzer_nil_optional() {
  let code = r#" local x: nil? "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

#[test]
fn pretty_printer_fuzzer_pretty_print_with_zero_location() {
  let example = r#"
if _ then
elseif _ then
elseif l0 then
else
local function l0<t0>(...):(t0<t0...>,(any)|(<t0>((any)|(<t0>(""[[[[[[[[[[[[[[[[[[[[[[[[!*t")->()))->()))
end
end
"#;

  let parse_options = ParseOptions {
    capture_comments: true,
    ..Default::default()
  };
  // Box 钉堆：AstNameTable/Lexer/Parser 捕获宿主地址，宿主移动即悬垂。
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);
  let parse_result = Parser::parse(example, &mut names, &mut allocator, parse_options);

  assert!(!parse_result.root.is_null());
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`parse_result` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  let _ = unsafe { pretty_print_with_types_ast_stat_block(&mut *parse_result.root) };
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_hexadecimal_numbers() {
  let code = " local a = 0xFFFF ";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_if_stmt_semicolon() {
  let _fixture = Fixture::default();
  let code = r#"
        if init then
            x = string.sub(x, utf8.offset(x, init));
        end;
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_if_stmt_semicolon_2() {
  let _fixture = Fixture::default();
  let code = r#"
        if (t < 1) then return c/2*t*t + b end;
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_if_stmt_spaces_around_tokens() {
  for code in [
    " if     This then Once() end",
    " if This     then Once() end",
    " if This then     Once() end",
    " if This then Once()     end",
    " if This then Once()   else Other() end",
    " if This then Once() else    Other() end",
    " if This then Once()    elseif true then Other() end",
    " if This then Once() elseif     true then Other() end",
    " if This then Once() elseif true    then Other() end",
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_if_then_else_spaces_around_tokens() {
  let _fixture = Fixture::default();
  for code in [
    "local a = if   1 then 2 else 3",
    "local a = if 1   then 2 else 3",
    "local a = if 1 then   2 else 3",
    "local a = if 1 then 2   else 3",
    "local a = if 1 then 2 else   3",
    "local a = if 1 then 2   elseif 3 then 4 else 5",
    "local a = if 1 then 2 elseif   3 then 4 else 5",
    "local a = if 1 then 2 elseif 3   then 4 else 5",
    "local a = if 1 then 2 elseif 3 then   4 else 5",
    "local a = if 1 then 2 elseif 3 then 4   else 5",
    "local a = if 1 then 2 elseif 3 then 4 else   5",
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_if_then_else_spaces_between_else_if() {
  let _fixture = Fixture::default();
  let code = r#"
    return
        if a then "was a" else
        if b then "was b" else
        if c then "was c" else
        "was nothing!"
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_index_expr_spaces_around_tokens() {
  let _fixture = Fixture::default();
  for code in [
    "local _ = a[2]",
    "local _ = a   [2]",
    "local _ = a[   2]",
    "local _ = a[2   ]",
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_index_name_ends_with_digit() {
  let _fixture = Fixture::default();
  let code = "sparkles.Color = Color3.new()";
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_index_name_spaces_around_tokens() {
  let _fixture = Fixture::default();
  for code in [
    "local _ = a.name",
    "local _ = a   .name",
    "local _ = a.   name",
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_infinity() {
  let code = " local a = 1e500    local b = 1e400 ";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_lambda() {
  for code in [
    " local p=function(o, m, g) return 77 end ",
    " local p=function(o, m, g,...)  return 77 end ",
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_local_assignment() {
  for code in [" local x = 1 ", " local x, y, z = 1, 2, 3 ", " local x "] {
    assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_local_assignment_spaces_around_tokens() {
  for code in [
    " local    x = 1 ",
    " local x    = 1 ",
    " local x =    1 ",
    " local x   , y = 1, 2 ",
    " local x,    y = 1, 2 ",
    " local x, y = 1   , 2 ",
    " local x, y = 1,    2 ",
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_local_function() {
  for code in [
    " local function p(o, m, g) return 77 end ",
    " local function p(o, m, g,...)  return 77 end ",
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_local_function_spaces_around_tokens() {
  for code in [
    " local     function p(o, m, ...) end ",
    " local function    p(o, m, ...) end ",
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_method_calls() {
  let code = " foo.bar.baz:quux() ";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_method_definitions() {
  let code = " function foo.bar.baz:quux() end ";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_more_table_literals() {
  let code = r#" local t={['Content-Type']='text/plain'} "#;
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_need_a_space_between_number_literals_and_dots() {
  let code = r#" return point and math.ceil(point* 100000* 100)/ 100000 .. '%'or '' "#;
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_nested_do_block() {
  let code = r#"
        do
            do
                local x = 1
            end
        end
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_numbers() {
  let code = " local a=2510238627 ";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_numbers_with_separators() {
  let code = " local a = 123_456_789 ";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_omit_decimal_place_for_integers() {
  let code = " local a=5, 6, 7, 3.141, 1.1290000000000002e+45 ";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_parentheses_multiline() {
  let _fixture = Fixture::default();
  let code = r#"
local test = (
    x
)
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_position_correctly_updated_when_writing_multiline_string() {
  let code = r#"
    call([[
        testing
    ]]) "#;
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_array_types() {
  let _fixture = Fixture::default();
  let code = r#"
type t1 = {number}
type t2 = {[string]: number}
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_assign_multiple() {
  let _fixture = Fixture::default();
  let code = "a, b, c = 1, 2, 3";
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_assign_spaces_around_tokens() {
  let _fixture = Fixture::default();
  for code in [
    "a = 1",
    "a    = 1",
    "a =    1",
    "a   , b = 1, 2",
    "a,    b = 1, 2",
    "a, b = 1   , 2",
    "a, b = 1,    2",
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_ast_stat_block_overload() {
  let code = "local a = 1";
  // Box 钉堆：AstNameTable/Lexer/Parser 捕获宿主地址，宿主移动即悬垂。
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);
  let result = Parser::parse(code, &mut names, &mut allocator, ParseOptions::default());

  assert!(!result.root.is_null());

  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`result` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  let printed = unsafe { pretty_print_ast_stat_block(&mut *result.root) };
  assert_eq!("local a = 1", printed);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_break_continue() {
  let _fixture = Fixture::default();
  let code = r#"
local a, b, c
repeat
    if a then break end
    if b then continue end
until c
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_chained_function_types() {
  for code in [
    r#" type Foo = () -> () -> () "#,
    r#" type Foo = () -> ()   -> () "#,
    r#" type Foo = () -> () ->   () "#,
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_compound_assignment() {
  let _fixture = Fixture::default();
  let code = r#"
local a = 1
a += 2
a -= 3
a *= 4
a /= 5
a //= 5
a %= 6
a ^= 7
a ..= ' - result'
    "#;
  let result = pp(code, ParseOptions::default(), true, false);
  assert_eq!(code, result.code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_declare_global_stat() {
  let _fixture = Fixture::default();
  let code = "declare _G: any";

  let options = ParseOptions {
    allow_declaration_syntax: true,
    ..Default::default()
  };
  assert_eq!(code, pp(code, options, true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_double_quoted_string_types() {
  let code = r#" type a = "hello world" "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

#[test]
fn pretty_printer_pretty_print_error_expr() {
  let _fixture = Fixture::default();
  let code = "local a = f:-";

  // Box 钉堆：AstNameTable/Lexer/Parser 捕获宿主地址，宿主移动即悬垂。
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);
  let parse_result = Parser::parse(code, &mut names, &mut allocator, ParseOptions::default());

  assert!(!parse_result.root.is_null());
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`parse_result` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  let actual = unsafe { pretty_print_with_types_ast_stat_block(&mut *parse_result.root) };
  assert_eq!("local a = (error-expr: f:%error-id%)-(error-expr)", actual);
}

#[test]
fn pretty_printer_pretty_print_error_stat() {
  let _fixture = Fixture::default();
  let code = "-";

  // Box 钉堆：AstNameTable/Lexer/Parser 捕获宿主地址，宿主移动即悬垂。
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);
  let parse_result = Parser::parse(code, &mut names, &mut allocator, ParseOptions::default());

  assert!(!parse_result.root.is_null());
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`parse_result` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  let actual = unsafe { pretty_print_with_types_ast_stat_block(&mut *parse_result.root) };
  assert_eq!("(error-stat: (error-expr))", actual);
}

#[test]
fn pretty_printer_pretty_print_error_type() {
  let _fixture = Fixture::default();
  let code = "local a: ";

  // Box 钉堆：AstNameTable/Lexer/Parser 捕获宿主地址，宿主移动即悬垂。
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);
  let parse_result = Parser::parse(code, &mut names, &mut allocator, ParseOptions::default());

  assert!(!parse_result.root.is_null());
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`parse_result` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  let actual = unsafe { pretty_print_with_types_ast_stat_block(&mut *parse_result.root) };
  assert_eq!("local a:%error-type%", actual);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_escaped_string_types() {
  let code = r#" type a = "\\b\\t\\n\\\\" "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

#[test]
fn pretty_printer_pretty_print_explicit_type_instantiations() {
  let mut code = "f<<A, B, C...>>() t.f<<A, B, C...>>() t:f<<A, B, C>>()" as &str;
  let result = pp(code, ParseOptions::default(), true, false);
  assert_eq!(code, result.code);

  // Box 钉堆：AstNameTable/Lexer/Parser 捕获宿主地址，宿主移动即悬垂。
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);
  let parse_result = Parser::parse(code, &mut names, &mut allocator, ParseOptions::default());
  assert!(parse_result.errors.is_empty(), "{:?}", parse_result.errors);
  assert!(!parse_result.root.is_null());
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`parse_result` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  let actual = unsafe { pretty_print_with_types_ast_stat_block(&mut *parse_result.root) };
  assert_eq!(code, actual);

  let result = pp(code, ParseOptions::default(), false, false);
  assert_eq!(
    "f              () t.f              () t:f           ()",
    result.code
  );

  code = "f < < A , B , C... > >( ) t.f < < A, B, C... > >  ( )  t:f< < A, B, C > > ( )";
  let result = pp(code, ParseOptions::default(), true, false);
  assert_eq!(code, result.code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_for_in_multiple() {
  let _fixture = Fixture::default();
  let code = "for k,v in next,{}do print(k,v) end";
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_for_in_multiple_types() {
  let _fixture = Fixture::default();
  let code = "for k:string,v:boolean in next,{}do end";
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_for_loop_annotation_spaces_around_tokens() {
  let _fixture = Fixture::default();
  for code in [
    r#" for i: number = 1, 10 do end "#,
    r#" for i   : number = 1, 10 do end "#,
    r#" for i:    number = 1, 10 do end "#,
    r#" for x: number, y: number in ... do end "#,
    r#" for x   : number, y: number in ... do end "#,
    r#" for x:    number, y: number in ... do end "#,
    r#" for x: number, y   : number in ... do end "#,
    r#" for x: number, y:    number in ... do end "#,
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
  }
}

#[test]
fn pretty_printer_pretty_print_function_attributes() {
  let mut code = r#"
        @native
        function foo()
        end
    "#;
  let result = pp(code, ParseOptions::default(), true, false);
  assert_eq!(code, result.code);

  code = r#"
        @native
        local function foo()
        end
    "#;
  let result = pp(code, ParseOptions::default(), true, false);
  assert_eq!(code, result.code);

  code = r#"
        @checked local function foo()
        end
    "#;
  let result = pp(code, ParseOptions::default(), true, false);
  assert_eq!(code, result.code);

  code = r#"
        local foo = @native function() end
    "#;
  let result = pp(code, ParseOptions::default(), true, false);
  assert_eq!(code, result.code);

  code = r#"
        @native
        function foo:bar()
        end
    "#;
  let result = pp(code, ParseOptions::default(), true, false);
  assert_eq!(code, result.code);

  code = r#"
        @native   @checked
        function foo:bar()
        end
    "#;
  let result = pp(code, ParseOptions::default(), true, false);
  assert_eq!(code, result.code);

  {
    let _no_inline = ScopedFastFlag::new(&fflag::DebugLuauNoInline, true);
    code = r#"
        @debugnoinline
        local function t() end
        "#;
    let result = pp(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_generic_function() {
  let _fixture = Fixture::default();
  let code = r#"
local function foo<T,S...>(a: T, ...: S...) return 1 end
local f: <T,S...>(T, S...)->(number) = foo
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_if_then_else() {
  let _fixture = Fixture::default();
  let code = "local a = if 1 then 2 else 3";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_if_then_else_multiple_conditions() {
  let _fixture = Fixture::default();
  let code = "local a = if 1 then 2 elseif 3 then 4 else 5";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_if_then_else_multiple_conditions_2() {
  let _fixture = Fixture::default();
  let code = r#"
        local x = if yes
            then nil
            else if no
                then if this
                    then that
                    else other
                else nil
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

#[test]
fn pretty_printer_pretty_print_incomplete_assign_stat() {
  let _fixture = Fixture::default();
  let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);
  let code = r#"
x , y 1, 2
"#;

  assert_eq!(code, pp(code, ParseOptions::default(), true, true).code);
}

#[test]
fn pretty_printer_pretty_print_incomplete_do_stat() {
  let _fixture = Fixture::default();
  let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);
  let code = r#"
do
    print("hello world")
"#;

  assert_eq!(code, pp(code, ParseOptions::default(), true, true).code);
}

#[test]
fn pretty_printer_pretty_print_incomplete_explicit_type_instantiations() {
  let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);

  let mut code = "f<<A, B, C...>() t.f<<A, B, C...>() t:f<<A, B, C>()";
  let result = pp(code, ParseOptions::default(), true, true);
  assert_eq!(code, result.code);

  code = "f < < A , B , C...  >( ) t.f < < A, B, C...  >  ( )  t:f< < A, B, C  > ( )";
  let result = pp(code, ParseOptions::default(), true, true);
  assert_eq!(code, result.code);
}

#[test]
fn pretty_printer_pretty_print_incomplete_expr_group() {
  let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);
  let _cst_expr_group = ScopedFastFlag::new(&fflag::LuauCstExprGroup, true);

  let mut code = "local x = (1 + 2";
  let result = pp(code, ParseOptions::default(), true, true);
  assert_eq!(code, result.code);

  code = "local x = (1 + 2                 )";
  let result = pp(code, ParseOptions::default(), true, true);
  assert_eq!(code, result.code);
}

#[test]
fn pretty_printer_pretty_print_incomplete_for_stat() {
  let _fixture = Fixture::default();
  let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);
  let code = r#"
for i : number = 1 10 do
    print(i)
end
"#;

  assert_eq!(code, pp(code, ParseOptions::default(), true, true).code);
}

#[test]
fn pretty_printer_pretty_print_incomplete_function_call() {
  let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);

  let mut code = "print('hello world'";
  let result = pp(code, ParseOptions::default(), true, true);
  assert_eq!(code, result.code);

  code = "t:hello('world'";
  let result = pp(code, ParseOptions::default(), true, true);
  assert_eq!(code, result.code);
}

#[test]
fn pretty_printer_pretty_print_incomplete_function_expr() {
  let _fixture = Fixture::default();
  let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);
  let code = r#"
local a = function<T(x : T, y: string, ... : number)
    return x
end"#;

  assert_eq!(code, pp(code, ParseOptions::default(), true, true).code);
}

#[test]
fn pretty_printer_pretty_print_incomplete_function_type() {
  let _fixture = Fixture::default();
  let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);

  let mut code = r#"
local function foo() : (number, string -> ()
end
"#;
  let result = pp(code, ParseOptions::default(), true, true);
  assert_eq!(code, result.code);

  code = "type foo = <A(number) -> string";
  let result = pp(code, ParseOptions::default(), true, true);
  assert_eq!(code, result.code);

  code = "type foo = <A>number) -> string";
  let result = pp(code, ParseOptions::default(), true, true);
  assert_eq!(code, result.code);

  code = "type foo = <A>(number -> string";
  let result = pp(code, ParseOptions::default(), true, true);
  assert_eq!(code, result.code);
}

#[test]
fn pretty_printer_pretty_print_incomplete_generic_typepack() {
  let _fixture = Fixture::default();
  let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);
  let code = "type foo<T, U..., V> = bar<T, U..., V>";

  assert_eq!(code, pp(code, ParseOptions::default(), true, true).code);
}

#[test]
fn pretty_printer_pretty_print_incomplete_if_else_expr() {
  let _fixture = Fixture::default();
  let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);
  let code = r#"local a = if true 1 else 2"#;

  assert_eq!(code, pp(code, ParseOptions::default(), true, true).code);
}

#[test]
fn pretty_printer_pretty_print_incomplete_index_expr() {
  let _fixture = Fixture::default();
  let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);
  let code = "local a = {1, 2, 3} local b = a[2";

  assert_eq!(code, pp(code, ParseOptions::default(), true, true).code);
}

#[test]
fn pretty_printer_pretty_print_incomplete_repeat_stat() {
  let _fixture = Fixture::default();
  let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);
  let code = r#"
repeat
    print("hello world")
"#;
  let expected = r#"
repeat
    print("hello world")
(error-expr)"#;

  assert_eq!(expected, pp(code, ParseOptions::default(), true, true).code);
}

#[test]
fn pretty_printer_pretty_print_incomplete_table_expr() {
  let _fixture = Fixture::default();
  let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);
  let _table_indent = ScopedFastFlag::new(&fflag::LuauTableEntriesDontNeedToMatchIndent, true);

  let mut code = r#"local a = { a = 1 ["b"] = 2 }"#;
  let result = pp(code, ParseOptions::default(), true, true);
  assert_eq!(code, result.code);

  code = r#"local a = { ["b" = 2, ["c"] 3, ["d" 4 }"#;
  let result = pp(code, ParseOptions::default(), true, true);
  assert_eq!(code, result.code);
}

#[test]
fn pretty_printer_pretty_print_incomplete_table_type() {
  let _fixture = Fixture::default();
  let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);

  let mut code = r#"type foo = { ["hello" : number }"#;
  let result = pp(code, ParseOptions::default(), true, true);
  assert_eq!(code, result.code);

  code = r#"type foo = { ["hello"] number }"#;
  let result = pp(code, ParseOptions::default(), true, true);
  assert_eq!(code, result.code);

  code = r#"type foo = { ["hello" number }"#;
  let result = pp(code, ParseOptions::default(), true, true);
  assert_eq!(code, result.code);

  code = r#"type foo = { ["hello"] number, ["world"] number, ["i" : "rule" }"#;
  let result = pp(code, ParseOptions::default(), true, true);
  assert_eq!(code, result.code);

  code = r#"type foo = { [number] number }"#;
  let result = pp(code, ParseOptions::default(), true, true);
  assert_eq!(code, result.code);

  code = r#"type foo = { [number : number }"#;
  let result = pp(code, ParseOptions::default(), true, true);
  assert_eq!(code, result.code);

  code = r#"type foo = { [number  number }"#;
  let result = pp(code, ParseOptions::default(), true, true);
  assert_eq!(code, result.code);
}

#[test]
fn pretty_printer_pretty_print_incomplete_type_alias() {
  let _fixture = Fixture::default();
  let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);
  let code = "type foo number";

  assert_eq!(code, pp(code, ParseOptions::default(), true, true).code);
}

#[test]
fn pretty_printer_pretty_print_incomplete_type_group() {
  let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);
  let _cst_type_group = ScopedFastFlag::new(&fflag::LuauCstTypeGroup, true);

  let mut code = "type t = (number";
  let result = pp(code, ParseOptions::default(), true, true);
  assert_eq!(code, result.code);

  code = "type t = (number           )";
  let result = pp(code, ParseOptions::default(), true, true);
  assert_eq!(code, result.code);
}

#[test]
fn pretty_printer_pretty_print_incomplete_type_reference() {
  let _fixture = Fixture::default();
  let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);
  let code = "type foo = Bar<number";

  assert_eq!(code, pp(code, ParseOptions::default(), true, true).code);
}

#[test]
fn pretty_printer_pretty_print_incomplete_typeof_type() {
  let _fixture = Fixture::default();
  let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);

  let mut code = "type foo = typeof x)";
  let result = pp(code, ParseOptions::default(), true, true);
  assert_eq!(code, result.code);

  code = "type foo = typeof(x";
  let result = pp(code, ParseOptions::default(), true, true);
  assert_eq!(code, result.code);

  code = "type foo = typeof x";
  let result = pp(code, ParseOptions::default(), true, true);
  assert_eq!(code, result.code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_index_expr() {
  let _fixture = Fixture::default();
  let code = "local a = {1, 2, 3} local b = a[2]";
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_intersection_spaces_around_tokens() {
  let _fixture = Fixture::default();
  for code in ["local a: string   & number", "local a: string &   number"] {
    assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_intersection_type_nested() {
  let _fixture = Fixture::default();
  let code = "local a: ((number)->(string))&((string)->(string))";
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_intersection_type_nested_2() {
  let _fixture = Fixture::default();
  let code = "local a: (number|string)&(string|boolean)";
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_intersection_type_with_function() {
  let _fixture = Fixture::default();
  let code = "type FnB<U...> = () -> U... & T";
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_leading_intersection_ampersand() {
  let _fixture = Fixture::default();
  for code in ["local a: & string & number", "local a: & string"] {
    assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_leading_union_pipe() {
  let _fixture = Fixture::default();
  for code in ["local a: | string | number", "local a: | string"] {
    assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_mixed_union_intersection() {
  let _fixture = Fixture::default();
  for code in [
    "local a: string | (Foo & Bar)",
    "local a: string |   (Foo & Bar)",
    "local a: string | (  Foo & Bar)",
    "local a: string | (Foo & Bar  )",
    "local a: string &   (Foo | Bar)",
    "local a: string & (  Foo | Bar)",
    "local a: string & (Foo | Bar  )",
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_parse_error() {
  let _fixture = Fixture::default();
  let code = "local a = -";
  let result = pp(code, ParseOptions::default(), false, false);

  assert_eq!("", result.code);
  assert_eq!(
    "Expected identifier when parsing expression, got <eof>",
    result.parse_error
  );
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_preserve_union_optional_style() {
  let _fixture = Fixture::default();
  for code in [
    "local a: string | nil",
    "local a: string?",
    "local a: string???",
    "local a: string? | nil",
    "local a: string | nil | number",
    "local a: string | nil | number?",
    "local a: string? | number?",
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_raw_string_types() {
  for code in [
    r#" type a = [[ hello world ]] "#,
    r#" type a = [==[ hello world ]==] "#,
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
// cpp:2786 pretty_print_readonly_indexer
// 只读索引器（`{ read number }` / `{ read [string]: boolean }`）回环。
// 注：cpp 侧以 ScopedFastFlag 打开 LuauPrettyPrintVisualizeIndexerAccess；该旗标
// 本端口 HEAD 未定义（cpp 门控的 printer 分支在本 Rust 打印器中无条件生效），
// 故不加 SFF，直接以回环精确串锁定 cpp 期望输出。
#[test]
fn pretty_printer_pretty_print_readonly_indexer() {
  let code = r#"
        local _t: { read number } = {}
        local _u: { read [string]: boolean }
    "#;

  assert_eq!(code, pp(code, ParseOptions::default(), true, true).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_single_quoted_string_types() {
  let code = r#" type a = 'hello world' "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_singleton_types() {
  let _fixture = Fixture::default();
  let code = r#"
type t1 = 'hello'
type t2 = true
type t3 = ''
type t4 = false
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_string_interp() {
  let _fixture = Fixture::default();
  let code = r#" local _ = `hello {name}` "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_string_interp_multiline() {
  let _fixture = Fixture::default();
  let code = r#" local _ = `hello {
        name
    }!` "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_string_interp_multiline_escape() {
  let _fixture = Fixture::default();
  let code = r#" local _ = `hello \
        world!` "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_string_interp_on_new_line() {
  let _fixture = Fixture::default();
  let code = r#"
        error(
            `a {b} c`
        )
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_string_literal_escape() {
  let _fixture = Fixture::default();
  let code = r#" local _ = ` bracket = \{, backtick = \` = {'ok'} ` "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

#[test]
fn pretty_printer_pretty_print_to_string() {
  let _fixture = Fixture::default();
  let code = "local a: string = 'hello'";

  // Box 钉堆：AstNameTable/Lexer/Parser 捕获宿主地址，宿主移动即悬垂。
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);
  let parse_result = Parser::parse(code, &mut names, &mut allocator, ParseOptions::default());

  assert!(!parse_result.root.is_null());
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`names` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  let root = unsafe { &*parse_result.root };
  assert_eq!(1, root.body.len());

  let stat = root.body.at(0).as_ptr();
  // 门面一步下转+判型+物化（原 `ast_node_as + is_null 断言 + &*` 三步样板）：
  // stat 为 body 数组内存活语句指针（夹具 arena 单线程只读存活至用例结束）。
  let stat_local =
    unsafe { ast_node_try_as_ptr::<AstStatLocal>(stat) }.expect("首条语句应为 AstStatLocal");
  assert_eq!(
    "local a: string = 'hello'",
    to_string_ast_node(&stat_local.base.base)
  );

  assert_eq!(1, stat_local.vars.len());
  let local = stat_local.vars[0];
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`names` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  let annotation = unsafe { (*local).annotation };
  assert!(!annotation.is_null());
  // 指针槽读法走夹具门面 PtrRef（null → None），只读借用交给 to_string_ast_node。
  assert_eq!(
    "string",
    to_string_ast_node(
      &PtrRef::as_ref_opt(&annotation)
        .expect("annotation 指向 arena 存活 AstType")
        .base
    )
  );

  assert_eq!(1, stat_local.values.len());
  let expr = stat_local.values[0];
  assert_eq!(
    "'hello'",
    to_string_ast_node(
      &PtrRef::as_ref_opt(&expr)
        .expect("expr 是 body 元素的 arena 存活表达式节点")
        .base
    )
  );
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_type_alias_default_type_parameters() {
  let _fixture = Fixture::default();
  let code = r#"
type Packed<T = string, U = T, V... = ...boolean, W... = (T, U, V...)> = (T, U, V...)->(W...)
local a: Packed<number>
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_type_annotation_spaces_around_tokens() {
  let _fixture = Fixture::default();
  for code in [
    r#" local _: Type "#,
    r#" local _  : Type "#,
    r#" local _:   Type "#,
    r#" local x: Type, y = 1 "#,
    r#" local x  : Type, y = 1 "#,
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_type_assertion() {
  let _fixture = Fixture::default();
  let code = "local a = 5 :: number";
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_type_function_generics() {
  for code in [
    r#" type Foo = <X, Y, Z...>() -> () "#,
    r#" type Foo =   <X, Y, Z...>() -> () "#,
    r#" type Foo = <  X, Y, Z...>() -> () "#,
    r#" type Foo = <X  , Y, Z...>() -> () "#,
    r#" type Foo = <X,   Y, Z...>() -> () "#,
    r#" type Foo = <X, Y  , Z...>() -> () "#,
    r#" type Foo = <X, Y,   Z...>() -> () "#,
    r#" type Foo = <X, Y, Z  ...>() -> () "#,
    r#" type Foo = <X, Y, Z...  >() -> () "#,
    r#" type Foo = <X, Y, Z...>  () -> () "#,
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_type_function_named_arguments() {
  for code in [
    r#" type Foo = (x: string) -> () "#,
    r#" type Foo = (x: string, y: number) -> ()  "#,
    r#" type Foo = (  x: string, y: number) -> () "#,
    r#" type Foo = (x  : string, y: number) -> () "#,
    r#" type Foo = (x:   string, y: number) -> () "#,
    r#" type Foo = (x: string,   y: number) -> () "#,
    r#" type Foo = (number, info: string) -> () "#,
    r#" type Foo = (first: string, second: string, ...string) -> () "#,
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_type_function_return_types() {
  for code in [
    r#" type Foo = () ->   () "#,
    r#" type Foo = () -> (  ) "#,
    r#" type Foo = () -> string "#,
    r#" type Foo = () ->   string "#,
    r#" type Foo = () -> (string) "#,
    r#" type Foo = () ->   (string) "#,
    r#" type Foo = () -> ...any "#,
    r#" type Foo = () ->   ...any "#,
    r#" type Foo = () -> ...  any "#,
    r#" type Foo = () -> (...any) "#,
    r#" type Foo = () -> (  string, number) "#,
    r#" type Foo = () -> (string  , number) "#,
    r#" type Foo = () -> (string,   number) "#,
    r#" type Foo = () -> (string, number  ) "#,
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_type_function_unnamed_arguments() {
  for code in [
    r#" type Foo = () -> () "#,
    r#" type Foo =   () -> () "#,
    r#" type Foo = (string) -> () "#,
    r#" type Foo = (string, number) -> () "#,
    r#" type Foo = (  string, number) -> () "#,
    r#" type Foo = (string  , number) -> () "#,
    r#" type Foo = (string,   number) -> () "#,
    r#" type Foo = (string, number  ) -> () "#,
    r#" type Foo = (string, number)   -> () "#,
    r#" type Foo = (string, number) ->   ()  "#,
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_type_functions() {
  let _fixture = Fixture::default();
  let code =
    r#" type function foo(arg1, arg2) if arg1 == arg2 then return arg1 end return arg2 end "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_type_functions_spaces_around_tokens() {
  let _fixture = Fixture::default();
  for code in [
    r#" type   function foo() end "#,
    r#" type function   foo() end "#,
    r#" type function foo  () end "#,
    r#" export   type function foo() end "#,
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_type_packs() {
  let _fixture = Fixture::default();
  let code = r#"
type Packed<T...> = (T...)->(T...)
local a: Packed<>
local b: Packed<(number, string)>
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_type_reference_import() {
  let fixture = Fixture::default();
  fixture.file_resolver.source.insert(
    ModuleName::from("game/A"),
    r#"
export type Type = { a: number }
return {}
    "#
    .to_string(),
  );

  let code = r#"
local Import = require(game.A)
local a: Import.Type
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_type_reference_spaces_around_tokens() {
  let _fixture = Fixture::default();
  for code in [
    r#" local _: Foo.Type "#,
    r#" local _: Foo   .Type "#,
    r#" local _: Foo.   Type "#,
    r#" local _: Type  <> "#,
    r#" local _: Type<  > "#,
    r#" local _: Type<  number> "#,
    r#" local _: Type<number  ,string> "#,
    r#" local _: Type<number,  string  > "#,
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_type_table_access_modifiers() {
  for code in [
    r#"
        type Foo = {
            read  bar: number,
              write baz: number,
        }
    "#,
    r#" type Foo = { read string } "#,
    r#" type Foo = {
        read [string]: number,
        read ["property"]: number
    } "#,
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_type_table_preserve_indexer_location() {
  for code in [
    r#"
        type Foo = {
            [number]: string,
            property: number,
        }
    "#,
    r#"
        type Foo = {
            property: number,
            [number]: string,
        }
    "#,
    r#"
        type Foo = {
            property: number,
            [number]: string,
            property2: number,
        }
    "#,
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_type_table_preserve_original_indexer_style() {
  for code in [
    r#"
        type Foo = {
            [number]: string
        }
    "#,
    r#"
        type Foo = { { number } }
    "#,
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_type_table_preserve_property_definition_style() {
  let code = r#"
        type Foo = {
            ["$$typeof1"]: string,
            ['$$typeof2']: string,
        }
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_type_table_semicolon_separators() {
  let code = r#"
        type Foo = {
            bar: number;
            baz: number;
        }
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_type_table_spaces_between_tokens() {
  for code in [
    r#" type Foo = { bar: number, } "#,
    r#" type Foo = {   bar: number, } "#,
    r#" type Foo = { bar  : number, } "#,
    r#" type Foo = { bar:   number, } "#,
    r#" type Foo = { bar: number  , } "#,
    r#" type Foo = { bar: number,   } "#,
    r#" type Foo = { bar: number   } "#,
    r#" type Foo = { [string]: number } "#,
    r#" type Foo = {    [string]: number } "#,
    r#" type Foo = { [   string]: number } "#,
    r#" type Foo = { [string   ]: number } "#,
    r#" type Foo = { [string]   : number } "#,
    r#" type Foo = { [string]:   number } "#,
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_type_table_string_properties_spaces_between_tokens() {
  let code = r#"
        type Foo = {
            [  "$$typeof1"]: string,
            ['$$typeof2'  ]: string,
            ['$$typeof2'  ]: string,
        }
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_typeof_spaces_around_tokens() {
  let _fixture = Fixture::default();
  for code in [
    r#" type X = typeof(x) "#,
    r#" type X =    typeof(x) "#,
    r#" type X = typeof   (x) "#,
    r#" type X = typeof(   x) "#,
    r#" type X = typeof(x   ) "#,
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_types_preserve_parentheses_style() {
  for code in [
    r#" type Foo = number "#,
    r#" type Foo = (number) "#,
    r#" type Foo = ((number)) "#,
    r#" type Foo = (  (number)  ) "#,
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_unary() {
  let _fixture = Fixture::default();
  let code = r#"
local a = 1
local b = -1
local c = true
local d = not c
local e = 'hello'
local d = #e
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_union_reverse() {
  let _fixture = Fixture::default();
  let code = "local a: nil | number";
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_union_spaces_around_tokens() {
  let _fixture = Fixture::default();
  for code in ["local a: string   | number", "local a: string |   number"] {
    assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_union_type_nested() {
  let _fixture = Fixture::default();
  let code = "local a: ((number)->(string))|((string)->(string))";
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_union_type_nested_2() {
  let _fixture = Fixture::default();
  let code = "local a: (number&string)|(string&boolean)";
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_union_type_nested_3() {
  let _fixture = Fixture::default();
  let code = "local a: nil | (string & number)";
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_pretty_print_varargs() {
  let _fixture = Fixture::default();
  let code = "local function f(...) return ... end";
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_raw_strings() {
  let code = " local a = [[ hello world ]] ";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_raw_strings_with_blocks() {
  let code = " local a = [==[ hello world ]==] ";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

#[test]
fn pretty_printer_remixed_simple_class() {
  let _fflag = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let code = r#"
class Point
    function length(self)
        return 100
    end
    public x
    function create(): Point
        return Point { x = 0, y = 0 }
    end
    public y
end
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_repeat_until_loop() {
  let code = " repeat print() until f(x) ";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_repeat_until_loop_condition_on_new_line() {
  let code = r#"
    repeat
        print()
    until
        f(x) "#;
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_returns_spaces_around_tokens() {
  for code in [" return    1 ", " return 1   , 2 ", " return 1,  2 "] {
    assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
  }
}

#[test]
fn pretty_printer_roundtrip_generic_types() {
  let code = r#"
        export type A<T> = {v:T, next:A<T>}
    "#;
  // Box 钉堆：AstNameTable/Lexer/Parser 捕获宿主地址，宿主移动即悬垂。
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);

  let parse_result = Parser::parse(code, &mut names, &mut allocator, ParseOptions::default());

  assert!(parse_result.errors.is_empty());
  assert!(!parse_result.root.is_null());
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`parse_result` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  let actual = unsafe { pretty_print_with_types_ast_stat_block(&mut *parse_result.root) };
  assert_eq!(code, actual);
}

#[test]
fn pretty_printer_roundtrip_types() {
  let code = r#"
        local s:string='str'
        local t:{a:string,b:number,[string]:number}
        local fn:(string,string)->(number,number)
        local s2:typeof(s)='foo'
        local os:string?
        local sn:string|number
        local it:{x:number}&{y:number}
    "#;
  // Box 钉堆：AstNameTable/Lexer/Parser 捕获宿主地址，宿主移动即悬垂。
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);

  let parse_result = Parser::parse(code, &mut names, &mut allocator, ParseOptions::default());

  assert!(parse_result.errors.is_empty());
  assert!(!parse_result.root.is_null());
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`parse_result` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  let actual = unsafe { pretty_print_with_types_ast_stat_block(&mut *parse_result.root) };
  assert_eq!(code, actual);
}

#[test]
fn pretty_printer_simple_class_example() {
  let _fflag = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let code = r#"
class Point
    public x: number
    public y: number
    function length(self)
        return 100
    end
    function create()
        return Point { x = 0, y = 0 }
    end
end
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// cpp: PrettyPrinter.test.cpp "simple_class_inheritance" —— 带父类的 class 定义回环，
// 守护 b3-H2（AstStatClass 可视化丢 extends/super）。
#[test]
fn pretty_printer_simple_class_inheritance() {
  let _fflag = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let code = r#"
class Animal
    public species: string
end

class Cat extends Animal
    public meowMult: number
end
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

#[test]
fn pretty_printer_simple_class_with_public_functions() {
  let _fflag = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let code = r#"
class Point
    public function length(self)
        return 100
    end
    public x
    public function create(): Point
        return Point { x = 0, y = 0 }
    end
    public y
end
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_simple_interp_string() {
  let code = " local a = `hello world` ";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_single_quoted_strings() {
  let code = " local a = 'hello world' ";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_spaces_between_keywords_even_if_it_pushes_the_line_estimation_off() {
  let code = " if math.abs(raySlope) < .01 then return 0 end ";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_stmt_semicolon() {
  let _fixture = Fixture::default();
  for code in [" local test = 1; ", " local test = 1  ; "] {
    assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_string_literals() {
  let code = r#" local S='abcdef\n\f\a\020' "#;
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_string_literals_containing_utf8() {
  let code = " local S='lalala こんにちは' ";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_strips_type_annotations() {
  let code = " local s: string= 'hello there' ";
  let expected = " local s        = 'hello there' ";
  assert_eq!(
    expected,
    pp(code, ParseOptions::default(), false, false).code
  );
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_strips_type_assertion_expressions() {
  let code = " local s= some_function() :: any+ something_else() :: number ";
  let expected = " local s= some_function()       + something_else()           ";
  assert_eq!(
    expected,
    pp(code, ParseOptions::default(), false, false).code
  );
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_table_literal_closing_brace_at_correct_position() {
  let code = r#"
        local t={
            eggs='Tasty',
            avocado='more like awesomecavo amirite'
        }
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_table_literal_multiline_with_indexers() {
  let code = r#"
        local t = {
            ["my first value"] = "x";
            ["my second value"] = "y";
        }
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_table_literal_preserves_record_vs_general() {
  let code = r#" local t={['foo']='bar',quux=42} "#;
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_table_literal_with_keyword_key() {
  let code = r#" local t={['nil']=nil,['true']=true} "#;
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_table_literal_with_numeric_key() {
  let code = r#" local t={[5]='five',[6]='six'} "#;
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_table_literal_with_semicolon_separators() {
  let code = r#"
        local t = { x = 1; y = 2 }
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_table_literal_with_spaces_around_equals() {
  let code = r#"
        local t = { x    =   1  }
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_table_literal_with_spaces_around_separator() {
  let code = r#"
        local t = { x = 1  , y = 2 }
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_table_literal_with_trailing_separators() {
  let code = r#"
        local t = { x = 1, y = 2, }
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_table_literals() {
  let code = r#" local t={1, 2, 3, foo='bar', baz=99,[5.5]='five point five', 'end'} "#;
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_test_1() {
  let example = r#"
local function isPortal(element)
    if type(element)~='table'then
        return false
    end

    return element.component == Core.Portal
end
"#;

  let result = pp(example, ParseOptions::default(), false, false);
  assert_eq!(example, result.code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_type_alias_spaces_around_tokens() {
  let _fixture = Fixture::default();
  for code in [
    r#" type Foo = string "#,
    r#" type    Foo = string "#,
    r#" type Foo    = string "#,
    r#" type Foo =    string "#,
    r#" export type Foo = string "#,
    r#" export    type Foo = string "#,
    r#" type Foo<X, Y, Z...> = string "#,
    r#" type Foo  <X, Y, Z...> = string "#,
    r#" type Foo<  X, Y, Z...> = string "#,
    r#" type Foo<X  , Y, Z...> = string "#,
    r#" type Foo<X,   Y, Z...> = string "#,
    r#" type Foo<X, Y  , Z...> = string "#,
    r#" type Foo<X, Y,   Z...> = string "#,
    r#" type Foo<X, Y, Z  ...> = string "#,
    r#" type Foo<X, Y, Z...  > = string "#,
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_type_alias_with_defaults_spaces_around_tokens() {
  let _fixture = Fixture::default();
  for code in [
    r#" type Foo<X = string, Z... = ...any> = string "#,
    r#" type Foo<X   = string, Z... = ...any> = string "#,
    r#" type Foo<X =   string, Z... = ...any> = string "#,
    r#" type Foo<X = string, Z...   = ...any> = string "#,
    r#" type Foo<X = string, Z... =   ...any> = string "#,
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
  }
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_type_assertion_spaces_around_tokens() {
  let _fixture = Fixture::default();
  let code = "local a = 5   :: number";
  let result = pp(code, ParseOptions::default(), true, false);
  assert_eq!(code, result.code);

  let code = "local a = 5 ::   number";
  let result = pp(code, ParseOptions::default(), true, false);
  assert_eq!(code, result.code);
}

// Ported from upstream Luau doctest.
// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_type_lists_should_be_emitted_correctly() {
  let mut fixture = Fixture::default();
  let code = String::from(
    r#"
        local a = function(a: string, b: number, ...: string): (string, ...number)
        end

        local b = function(...: string): ...number
        end

        local c = function()
        end
    "#,
  );
  let expected = String::from(
    r#"
        local a:(a:string,b:number,...string)->(string,...number)=function(a:string,b:number,...:string): (string,...number)
        end

        local b:(...string)->(...number)=function(...:string): ...number
        end

        local c:()->()=function(): ()
        end
    "#,
  );

  let actual = fixture.decorate_with_types(&code);

  assert_eq!(expected, actual);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_type_packs_spaces_around_tokens() {
  let _fixture = Fixture::default();
  for code in [
    r#" type _ = Packed<  T...> "#,
    r#" type _ = Packed<T  ...> "#,
    r#" type _ = Packed<   ...T> "#,
    r#" type _ = Packed<...   T> "#,
    r#" type _ = Packed<  ()> "#,
    r#" type _ = Packed<  (string, number)> "#,
    r#" type _ = Packed<(  string, number)> "#,
    r#" type _ = Packed<(string  , number)> "#,
    r#" type _ = Packed<(string,   number)> "#,
    r#" type _ = Packed<(string, number  )> "#,
    r#" type _ = Packed<(string, number)  > "#,
    r#" type _ = Packed<(  )> "#,
    r#" type _ = Packed<()  > "#,
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
  }
}

// Ported from upstream Luau doctest.
// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_types_should_not_be_considered_cyclic_if_they_are_not_recursive() {
  let mut fixture = Fixture::default();
  let code = String::from(
    r#"
        local common: {foo:string} = {foo = 'foo'}

        local t = {}
        t.x = common
        t.y = common
    "#,
  );
  let expected = String::from(
    r#"
        local common: {foo:string} = {foo = 'foo'}

        local t:{x:{foo:string},y:{foo:string}}={}
        t.x = common
        t.y = common
    "#,
  );

  assert_eq!(expected, fixture.decorate_with_types(&code));
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_unary_spaces_around_tokens() {
  let _fixture = Fixture::default();
  let code = r#"
local _ =   -1
local _ = -  1
local _ =   not true
local _ = not   true
local _ =   #e
local _ = #  e
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_while_do_semicolon() {
  let _fixture = Fixture::default();
  let code = r#"
        while true do
        end;
    "#;
  assert_eq!(code, pp(code, ParseOptions::default(), true, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_while_loop() {
  let code = " while f(x)do print() end ";
  assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
}

// Source: `tests/PrettyPrinter.test.cpp`
#[test]
fn pretty_printer_while_loop_spaces_around_tokens() {
  for code in [
    " while     f(x) do print() end ",
    " while f(x)    do print() end ",
    " while f(x) do    print() end ",
    " while f(x) do print()    end ",
  ] {
    assert_eq!(code, pp(code, ParseOptions::default(), false, false).code);
  }
}
