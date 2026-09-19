use ulua_ast::rtti::ast_node_as;
extern crate alloc;

mod pretty_printer_a_table_key_can_be_the_empty_string {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_a_table_key_can_be_the_empty_string() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = "local T = {[''] = true}";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_always_emit_a_space_after_local_keyword {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_always_emit_a_space_after_local_keyword() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = "do local aZZZZ = Workspace.P1.Shape local bZZZZ = Enum.PartType.Cylinder end";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_attach_types {
  //! Ported from upstream Luau doctest.
  //! Source: `tests/PrettyPrinter.test.cpp`
  use super::*;

  #[test]
  fn pretty_printer_attach_types() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

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
}

mod pretty_printer_binary_keywords {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_binary_keywords() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = "local c = a0 ._ or b0 ._";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_binary_numbers {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_binary_numbers() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = " local a = 0b0101 ";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_binary_spaces_around_tokens {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_binary_spaces_around_tokens() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = r#"
local _ =    1+1
local _ = 1   +1
local _ = 1+   1
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_compound_assignment_spaces_around_tokens {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_compound_assignment_spaces_around_tokens() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    for code in [" a   += 1 ", " a +=   1 "] {
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_do_block_ending_with_semicolon {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_do_block_ending_with_semicolon() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = r#"
        do
            return;
        end;
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_do_blocks {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_do_blocks() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#"
        foo()

        do
            local bar=baz()
            quux()
        end

        foo2()
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_double_quoted_strings {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_double_quoted_strings() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#" local a = "hello world" "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_elseif_chains_indent_sensibly {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_elseif_chains_indent_sensibly() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

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

    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_emit_a_do_block_in_cases_of_potentially_ambiguous_syntax {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_emit_a_do_block_in_cases_of_potentially_ambiguous_syntax() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#"
        f();
        (g or f)()
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_escaped_strings {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_escaped_strings() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#" local s='\\b\\t\\n\\\\' "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_escaped_strings_2 {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_escaped_strings_2() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#" local s="\a\b\f\n\r\t\v\'\"\\" "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_escaped_strings_newline {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_escaped_strings_newline() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#"
    print("foo \
        bar")
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_escaped_strings_raw {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_escaped_strings_raw() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#" local x = [=[\v<((do|load)file|require)\s*\(?['"]\zs[^'"]+\ze['"]]=] "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_export {

  #[test]
  fn pretty_printer_export() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_common::fflag;
    use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _export_value = ScopedFastFlag::new(&fflag::LuauExportValueSyntax, true);

    let mut code = r#"
export                      local version = "1.0.0"
export           const tabbed = ...
export const TAU = math.pi * 2
export local settings: Settings = getSettings()
export local a, b, c = 1, 2, 3
export local d
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
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
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);

    code = r#"
@native
export function foo()
end

@native
export                 function tabbed_attribute()
end
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
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
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
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
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_for_in_loop {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_for_in_loop() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = " for k, v in ipairs(x)do end ";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_for_in_loop_spaces_around_tokens {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_for_in_loop_spaces_around_tokens() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    for code in [
      " for k, v in ipairs(x)   do end ",
      " for k, v    in    ipairs(x) do end ",
      " for k  ,  v in ipairs(x) do end ",
      " for k, v in next  , t  do end ",
      " for k, v in ipairs(x) do   end ",
    ] {
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        false,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_for_in_single_variable {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_for_in_single_variable() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = " for key in pairs(x) do end ";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_for_loop {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_for_loop() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    for code in [" for i=1,10 do end ", " for i=5,6,7 do end "] {
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        false,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_for_loop_spaces_around_tokens {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_for_loop_spaces_around_tokens() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    for code in [
      " for index = 1, 10 do call(index) end ",
      " for index = 1  , 10 do call(index) end ",
      " for index = 1, 10  ,  3 do call(index) end ",
      " for index = 1, 10    do call(index) end ",
      " for index = 1, 10 do call(index)    end ",
    ] {
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        false,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_for_loop_stmt_semicolon {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_for_loop_stmt_semicolon() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = r#"
        for i,v in ... do
        end;
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_function {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_function() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    for code in [
      " function p(o, m, g) return 77 end ",
      " function p(o, m, g,...)  return 77 end ",
    ] {
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        false,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_function_call_parentheses_multiple_args {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_function_call_parentheses_multiple_args() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = " call(arg1, arg3, arg3) ";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_function_call_parentheses_multiple_args_no_space {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_function_call_parentheses_multiple_args_no_space() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = " call(arg1,arg3,arg3) ";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_function_call_parentheses_multiple_args_space_before_commas {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_function_call_parentheses_multiple_args_space_before_commas() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = " call(arg1 ,arg3 ,arg3) ";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_function_call_parentheses_no_args {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_function_call_parentheses_no_args() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = " call() ";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_function_call_parentheses_one_arg {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_function_call_parentheses_one_arg() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = " call(arg) ";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_function_call_spaces_before_parentheses {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_function_call_spaces_before_parentheses() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = " call () ";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_function_call_spaces_within_parentheses {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_function_call_spaces_within_parentheses() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = " call(  ) ";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_function_call_string_double_quotes {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_function_call_string_double_quotes() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#" call "string" "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_function_call_string_no_space {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_function_call_string_no_space() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = " call'string' ";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_function_call_string_single_quotes {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_function_call_string_single_quotes() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = " call 'string' ";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_function_call_table_literal {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_function_call_table_literal() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#" call { x = 1 } "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_function_call_table_literal_no_space {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_function_call_table_literal_no_space() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#" call{x=1} "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_function_definition_semicolon {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_function_definition_semicolon() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = r#"
        function foo()
        end;
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_function_spaces_around_tokens {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_function_spaces_around_tokens() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

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
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        false,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_function_taking_ellipsis {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_function_taking_ellipsis() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = " function F(...) end ";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_function_type_location {
  //! Ported from upstream Luau doctest.
  //! Source: `tests/PrettyPrinter.test.cpp`
  use super::*;

  #[test]
  fn pretty_printer_function_type_location() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

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
}

mod pretty_printer_function_with_types_spaces_around_tokens {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_function_with_types_spaces_around_tokens() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

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
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_fuzzer_class {

  use ulua_common::fflag;
  #[test]
  fn pretty_printer_fuzzer_class() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _fflag = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);
    let code = r#" class l0 end "#;

    let _result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
  }
}

mod pretty_printer_fuzzer_nil_optional {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_fuzzer_nil_optional() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#" local x: nil? "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_fuzzer_pretty_print_with_zero_location {

  #[test]
  fn pretty_printer_fuzzer_pretty_print_with_zero_location() {
    use ulua_ast::{
      functions::pretty_print_with_types_pretty_printer_alt_b::pretty_print_with_types_ast_stat_block,
      records::{
        allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions,
        parser::Parser,
      },
    };

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
    let parse_result = Parser::parse(
      example,
      &mut names,
      &mut allocator,
      parse_options,
    );

    assert!(!parse_result.root.is_null());
    let _ = unsafe { pretty_print_with_types_ast_stat_block(&mut *parse_result.root) };
  }
}

mod pretty_printer_hexadecimal_numbers {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_hexadecimal_numbers() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = " local a = 0xFFFF ";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_if_stmt_semicolon {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_if_stmt_semicolon() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = r#"
        if init then
            x = string.sub(x, utf8.offset(x, init));
        end;
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_if_stmt_semicolon_2 {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_if_stmt_semicolon_2() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = r#"
        if (t < 1) then return c/2*t*t + b end;
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_if_stmt_spaces_around_tokens {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_if_stmt_spaces_around_tokens() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

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
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        false,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_if_then_else_spaces_around_tokens {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_if_then_else_spaces_around_tokens() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

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
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        false,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_if_then_else_spaces_between_else_if {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_if_then_else_spaces_between_else_if() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = r#"
    return
        if a then "was a" else
        if b then "was b" else
        if c then "was c" else
        "was nothing!"
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_index_expr_spaces_around_tokens {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_index_expr_spaces_around_tokens() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    for code in [
      "local _ = a[2]",
      "local _ = a   [2]",
      "local _ = a[   2]",
      "local _ = a[2   ]",
    ] {
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_index_name_ends_with_digit {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_index_name_ends_with_digit() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = "sparkles.Color = Color3.new()";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_index_name_spaces_around_tokens {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_index_name_spaces_around_tokens() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    for code in [
      "local _ = a.name",
      "local _ = a   .name",
      "local _ = a.   name",
    ] {
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_infinity {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_infinity() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = " local a = 1e500    local b = 1e400 ";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_lambda {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_lambda() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    for code in [
      " local p=function(o, m, g) return 77 end ",
      " local p=function(o, m, g,...)  return 77 end ",
    ] {
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        false,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_local_assignment {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_local_assignment() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    for code in [" local x = 1 ", " local x, y, z = 1, 2, 3 ", " local x "] {
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        false,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_local_assignment_spaces_around_tokens {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_local_assignment_spaces_around_tokens() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    for code in [
      " local    x = 1 ",
      " local x    = 1 ",
      " local x =    1 ",
      " local x   , y = 1, 2 ",
      " local x,    y = 1, 2 ",
      " local x, y = 1   , 2 ",
      " local x, y = 1,    2 ",
    ] {
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        false,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_local_function {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_local_function() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    for code in [
      " local function p(o, m, g) return 77 end ",
      " local function p(o, m, g,...)  return 77 end ",
    ] {
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        false,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_local_function_spaces_around_tokens {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_local_function_spaces_around_tokens() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    for code in [
      " local     function p(o, m, ...) end ",
      " local function    p(o, m, ...) end ",
    ] {
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        false,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_method_calls {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_method_calls() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = " foo.bar.baz:quux() ";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_method_definitions {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_method_definitions() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = " function foo.bar.baz:quux() end ";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_more_table_literals {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_more_table_literals() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#" local t={['Content-Type']='text/plain'} "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_need_a_space_between_number_literals_and_dots {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_need_a_space_between_number_literals_and_dots() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#" return point and math.ceil(point* 100000* 100)/ 100000 .. '%'or '' "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_nested_do_block {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_nested_do_block() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#"
        do
            do
                local x = 1
            end
        end
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_numbers {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_numbers() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = " local a=2510238627 ";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_numbers_with_separators {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_numbers_with_separators() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = " local a = 123_456_789 ";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_omit_decimal_place_for_integers {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_omit_decimal_place_for_integers() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = " local a=5, 6, 7, 3.141, 1.1290000000000002e+45 ";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_parentheses_multiline {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_parentheses_multiline() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = r#"
local test = (
    x
)
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_position_correctly_updated_when_writing_multiline_string {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_position_correctly_updated_when_writing_multiline_string() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#"
    call([[
        testing
    ]]) "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_array_types {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_array_types() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = r#"
type t1 = {number}
type t2 = {[string]: number}
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_assign_multiple {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_assign_multiple() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = "a, b, c = 1, 2, 3";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_assign_spaces_around_tokens {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_assign_spaces_around_tokens() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

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
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        false,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_pretty_print_ast_stat_block_overload {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_ast_stat_block_overload() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_b::pretty_print_ast_stat_block,
      records::{
        allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions,
        parser::Parser,
      },
    };

    let code = "local a = 1";
    // Box 钉堆：AstNameTable/Lexer/Parser 捕获宿主地址，宿主移动即悬垂。
    let mut allocator = Box::new(Allocator::new());
    let mut names = AstNameTable::new(&mut allocator);
    let result = Parser::parse(
      code,
      &mut names,
      &mut allocator,
      ParseOptions::default(),
    );

    assert!(!result.root.is_null());

    let printed = unsafe { pretty_print_ast_stat_block(&mut *result.root) };
    assert_eq!("local a = 1", printed);
  }
}

mod pretty_printer_pretty_print_break_continue {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_break_continue() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = r#"
local a, b, c
repeat
    if a then break end
    if b then continue end
until c
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_chained_function_types {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_chained_function_types() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    for code in [
      r#" type Foo = () -> () -> () "#,
      r#" type Foo = () -> ()   -> () "#,
      r#" type Foo = () -> () ->   () "#,
    ] {
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_pretty_print_compound_assignment {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_compound_assignment() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

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
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_declare_global_stat {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_declare_global_stat() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = "declare _G: any";

    let options = ParseOptions {
      allow_declaration_syntax: true,
      ..Default::default()
    };
    let result = pretty_print_string_view_parse_options_bool_bool(code, options, true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_double_quoted_string_types {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_double_quoted_string_types() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#" type a = "hello world" "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_error_expr {

  #[test]
  fn pretty_printer_pretty_print_error_expr() {
    use ulua_ast::{
      functions::pretty_print_with_types_pretty_printer_alt_b::pretty_print_with_types_ast_stat_block,
      records::{
        allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions,
        parser::Parser,
      },
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = "local a = f:-";

    // Box 钉堆：AstNameTable/Lexer/Parser 捕获宿主地址，宿主移动即悬垂。
    let mut allocator = Box::new(Allocator::new());
    let mut names = AstNameTable::new(&mut allocator);
    let parse_result = Parser::parse(
      code,
      &mut names,
      &mut allocator,
      ParseOptions::default(),
    );

    assert!(!parse_result.root.is_null());
    let actual = unsafe { pretty_print_with_types_ast_stat_block(&mut *parse_result.root) };
    assert_eq!("local a = (error-expr: f:%error-id%)-(error-expr)", actual);
  }
}

mod pretty_printer_pretty_print_error_stat {

  #[test]
  fn pretty_printer_pretty_print_error_stat() {
    use ulua_ast::{
      functions::pretty_print_with_types_pretty_printer_alt_b::pretty_print_with_types_ast_stat_block,
      records::{
        allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions,
        parser::Parser,
      },
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = "-";

    // Box 钉堆：AstNameTable/Lexer/Parser 捕获宿主地址，宿主移动即悬垂。
    let mut allocator = Box::new(Allocator::new());
    let mut names = AstNameTable::new(&mut allocator);
    let parse_result = Parser::parse(
      code,
      &mut names,
      &mut allocator,
      ParseOptions::default(),
    );

    assert!(!parse_result.root.is_null());
    let actual = unsafe { pretty_print_with_types_ast_stat_block(&mut *parse_result.root) };
    assert_eq!("(error-stat: (error-expr))", actual);
  }
}

mod pretty_printer_pretty_print_error_type {

  #[test]
  fn pretty_printer_pretty_print_error_type() {
    use ulua_ast::{
      functions::pretty_print_with_types_pretty_printer_alt_b::pretty_print_with_types_ast_stat_block,
      records::{
        allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions,
        parser::Parser,
      },
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = "local a: ";

    // Box 钉堆：AstNameTable/Lexer/Parser 捕获宿主地址，宿主移动即悬垂。
    let mut allocator = Box::new(Allocator::new());
    let mut names = AstNameTable::new(&mut allocator);
    let parse_result = Parser::parse(
      code,
      &mut names,
      &mut allocator,
      ParseOptions::default(),
    );

    assert!(!parse_result.root.is_null());
    let actual = unsafe { pretty_print_with_types_ast_stat_block(&mut *parse_result.root) };
    assert_eq!("local a:%error-type%", actual);
  }
}

mod pretty_printer_pretty_print_escaped_string_types {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_escaped_string_types() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#" type a = "\\b\\t\\n\\\\" "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_explicit_type_instantiations {

  #[test]
  fn pretty_printer_pretty_print_explicit_type_instantiations() {
    use ulua_ast::{
      functions::{
        pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
        pretty_print_with_types_pretty_printer_alt_b::pretty_print_with_types_ast_stat_block,
      },
      records::{
        allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions,
        parser::Parser,
      },
    };

    let mut code = "f<<A, B, C...>>() t.f<<A, B, C...>>() t:f<<A, B, C>>()" as &str;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);

    // Box 钉堆：AstNameTable/Lexer/Parser 捕获宿主地址，宿主移动即悬垂。
    let mut allocator = Box::new(Allocator::new());
    let mut names = AstNameTable::new(&mut allocator);
    let parse_result = Parser::parse(
      code,
      &mut names,
      &mut allocator,
      ParseOptions::default(),
    );
    assert!(parse_result.errors.is_empty(), "{:?}", parse_result.errors);
    assert!(!parse_result.root.is_null());
    let actual = unsafe { pretty_print_with_types_ast_stat_block(&mut *parse_result.root) };
    assert_eq!(code, actual);

    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(
      "f              () t.f              () t:f           ()",
      result.code
    );

    code = "f < < A , B , C... > >( ) t.f < < A, B, C... > >  ( )  t:f< < A, B, C > > ( )";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_for_in_multiple {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_for_in_multiple() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = "for k,v in next,{}do print(k,v) end";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_for_in_multiple_types {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_for_in_multiple_types() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = "for k:string,v:boolean in next,{}do end";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_for_loop_annotation_spaces_around_tokens {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_for_loop_annotation_spaces_around_tokens() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

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
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_pretty_print_function_attributes {

  use ulua_common::fflag;
  #[test]
  fn pretty_printer_pretty_print_function_attributes() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let mut code = r#"
        @native
        function foo()
        end
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);

    code = r#"
        @native
        local function foo()
        end
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);

    code = r#"
        @checked local function foo()
        end
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);

    code = r#"
        local foo = @native function() end
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);

    code = r#"
        @native
        function foo:bar()
        end
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);

    code = r#"
        @native   @checked
        function foo:bar()
        end
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);

    {
      let _no_inline = ScopedFastFlag::new(&fflag::DebugLuauNoInline, true);
      code = r#"
        @debugnoinline
        local function t() end
        "#;
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_pretty_print_generic_function {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_generic_function() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = r#"
local function foo<T,S...>(a: T, ...: S...) return 1 end
local f: <T,S...>(T, S...)->(number) = foo
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_if_then_else {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_if_then_else() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = "local a = if 1 then 2 else 3";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_if_then_else_multiple_conditions {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_if_then_else_multiple_conditions() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = "local a = if 1 then 2 elseif 3 then 4 else 5";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_if_then_else_multiple_conditions_2 {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_if_then_else_multiple_conditions_2() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

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
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_incomplete_assign_stat {

  #[test]
  fn pretty_printer_pretty_print_incomplete_assign_stat() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_common::fflag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _fixture = Fixture::default();
    let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);
    let code = r#"
x , y 1, 2
"#;

    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_incomplete_do_stat {

  #[test]
  fn pretty_printer_pretty_print_incomplete_do_stat() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_common::fflag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _fixture = Fixture::default();
    let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);
    let code = r#"
do
    print("hello world")
"#;

    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_incomplete_explicit_type_instantiations {

  #[test]
  fn pretty_printer_pretty_print_incomplete_explicit_type_instantiations() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_common::fflag;
    use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);

    let mut code = "f<<A, B, C...>() t.f<<A, B, C...>() t:f<<A, B, C>()";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);

    code = "f < < A , B , C...  >( ) t.f < < A, B, C...  >  ( )  t:f< < A, B, C  > ( )";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_incomplete_expr_group {

  #[test]
  fn pretty_printer_pretty_print_incomplete_expr_group() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_common::fflag;
    use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);
    let _cst_expr_group = ScopedFastFlag::new(&fflag::LuauCstExprGroup, true);

    let mut code = "local x = (1 + 2";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);

    code = "local x = (1 + 2                 )";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_incomplete_for_stat {

  #[test]
  fn pretty_printer_pretty_print_incomplete_for_stat() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_common::fflag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _fixture = Fixture::default();
    let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);
    let code = r#"
for i : number = 1 10 do
    print(i)
end
"#;

    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_incomplete_function_call {

  #[test]
  fn pretty_printer_pretty_print_incomplete_function_call() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_common::fflag;
    use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);

    let mut code = "print('hello world'";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);

    code = "t:hello('world'";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_incomplete_function_expr {

  #[test]
  fn pretty_printer_pretty_print_incomplete_function_expr() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_common::fflag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _fixture = Fixture::default();
    let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);
    let code = r#"
local a = function<T(x : T, y: string, ... : number)
    return x
end"#;

    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_incomplete_function_type {

  #[test]
  fn pretty_printer_pretty_print_incomplete_function_type() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_common::fflag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _fixture = Fixture::default();
    let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);

    let mut code = r#"
local function foo() : (number, string -> ()
end
"#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);

    code = "type foo = <A(number) -> string";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);

    code = "type foo = <A>number) -> string";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);

    code = "type foo = <A>(number -> string";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_incomplete_generic_typepack {

  #[test]
  fn pretty_printer_pretty_print_incomplete_generic_typepack() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_common::fflag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _fixture = Fixture::default();
    let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);
    let code = "type foo<T, U..., V> = bar<T, U..., V>";

    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_incomplete_if_else_expr {

  #[test]
  fn pretty_printer_pretty_print_incomplete_if_else_expr() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_common::fflag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _fixture = Fixture::default();
    let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);
    let code = r#"local a = if true 1 else 2"#;

    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_incomplete_index_expr {

  #[test]
  fn pretty_printer_pretty_print_incomplete_index_expr() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_common::fflag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _fixture = Fixture::default();
    let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);
    let code = "local a = {1, 2, 3} local b = a[2";

    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_incomplete_repeat_stat {

  #[test]
  fn pretty_printer_pretty_print_incomplete_repeat_stat() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_common::fflag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

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

    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(expected, result.code);
  }
}

mod pretty_printer_pretty_print_incomplete_table_expr {

  #[test]
  fn pretty_printer_pretty_print_incomplete_table_expr() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_common::fflag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _fixture = Fixture::default();
    let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);
    let _table_indent = ScopedFastFlag::new(&fflag::LuauTableEntriesDontNeedToMatchIndent, true);

    let mut code = r#"local a = { a = 1 ["b"] = 2 }"#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);

    code = r#"local a = { ["b" = 2, ["c"] 3, ["d" 4 }"#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_incomplete_table_type {

  #[test]
  fn pretty_printer_pretty_print_incomplete_table_type() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_common::fflag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _fixture = Fixture::default();
    let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);

    let mut code = r#"type foo = { ["hello" : number }"#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);

    code = r#"type foo = { ["hello"] number }"#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);

    code = r#"type foo = { ["hello" number }"#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);

    code = r#"type foo = { ["hello"] number, ["world"] number, ["i" : "rule" }"#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);

    code = r#"type foo = { [number] number }"#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);

    code = r#"type foo = { [number : number }"#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);

    code = r#"type foo = { [number  number }"#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_incomplete_type_alias {

  #[test]
  fn pretty_printer_pretty_print_incomplete_type_alias() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_common::fflag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _fixture = Fixture::default();
    let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);
    let code = "type foo number";

    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_incomplete_type_group {

  #[test]
  fn pretty_printer_pretty_print_incomplete_type_group() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_common::fflag;
    use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);
    let _cst_type_group = ScopedFastFlag::new(&fflag::LuauCstTypeGroup, true);

    let mut code = "type t = (number";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);

    code = "type t = (number           )";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_incomplete_type_reference {

  #[test]
  fn pretty_printer_pretty_print_incomplete_type_reference() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_common::fflag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _fixture = Fixture::default();
    let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);
    let code = "type foo = Bar<number";

    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_incomplete_typeof_type {

  #[test]
  fn pretty_printer_pretty_print_incomplete_typeof_type() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_common::fflag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _fixture = Fixture::default();
    let _error_tolerant = ScopedFastFlag::new(&fflag::LuauErrorTolerantPrettyPrinting, true);

    let mut code = "type foo = typeof x)";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);

    code = "type foo = typeof(x";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);

    code = "type foo = typeof x";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_index_expr {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_index_expr() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = "local a = {1, 2, 3} local b = a[2]";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_intersection_spaces_around_tokens {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_intersection_spaces_around_tokens() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    for code in ["local a: string   & number", "local a: string &   number"] {
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_pretty_print_intersection_type_nested {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_intersection_type_nested() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = "local a: ((number)->(string))&((string)->(string))";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_intersection_type_nested_2 {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_intersection_type_nested_2() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = "local a: (number|string)&(string|boolean)";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_intersection_type_with_function {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_intersection_type_with_function() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = "type FnB<U...> = () -> U... & T";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_leading_intersection_ampersand {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_leading_intersection_ampersand() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    for code in ["local a: & string & number", "local a: & string"] {
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_pretty_print_leading_union_pipe {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_leading_union_pipe() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    for code in ["local a: | string | number", "local a: | string"] {
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_pretty_print_mixed_union_intersection {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_mixed_union_intersection() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

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
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_pretty_print_parse_error {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_parse_error() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = "local a = -";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);

    assert_eq!("", result.code);
    assert_eq!(
      "Expected identifier when parsing expression, got <eof>",
      result.parse_error
    );
  }
}

mod pretty_printer_pretty_print_preserve_union_optional_style {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_preserve_union_optional_style() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

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
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_pretty_print_raw_string_types {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_raw_string_types() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    for code in [
      r#" type a = [[ hello world ]] "#,
      r#" type a = [==[ hello world ]==] "#,
    ] {
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_pretty_print_single_quoted_string_types {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_single_quoted_string_types() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#" type a = 'hello world' "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_singleton_types {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_singleton_types() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = r#"
type t1 = 'hello'
type t2 = true
type t3 = ''
type t4 = false
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_string_interp {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_string_interp() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = r#" local _ = `hello {name}` "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_string_interp_multiline {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_string_interp_multiline() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = r#" local _ = `hello {
        name
    }!` "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_string_interp_multiline_escape {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_string_interp_multiline_escape() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = r#" local _ = `hello \
        world!` "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_string_interp_on_new_line {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_string_interp_on_new_line() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = r#"
        error(
            `a {b} c`
        )
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_string_literal_escape {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_string_literal_escape() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = r#" local _ = ` bracket = \{, backtick = \` = {'ok'} ` "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_to_string {
  use super::*;
  #[test]
  fn pretty_printer_pretty_print_to_string() {
    use ulua_ast::{
      functions::to_string_pretty_printer::to_string_ast_node,
      records::{
        allocator::Allocator, ast_name_table::AstNameTable, ast_node::AstNode,
        ast_stat_local::AstStatLocal, parse_options::ParseOptions, parser::Parser,
      },
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = "local a: string = 'hello'";

    // Box 钉堆：AstNameTable/Lexer/Parser 捕获宿主地址，宿主移动即悬垂。
    let mut allocator = Box::new(Allocator::new());
    let mut names = AstNameTable::new(&mut allocator);
    let parse_result = Parser::parse(
      code,
      &mut names,
      &mut allocator,
      ParseOptions::default(),
    );

    assert!(!parse_result.root.is_null());
    let root = unsafe { &*parse_result.root };
    assert_eq!(1, root.body.size);

    let stat = unsafe { *root.body.data.add(0) };
    let stat_local = unsafe { ast_node_as::<AstStatLocal>(stat as *mut AstNode) };
    assert!(!stat_local.is_null());
    assert_eq!("local a: string = 'hello'", unsafe {
      to_string_ast_node(stat_local as *mut AstNode)
    });

    let stat_local = unsafe { &*stat_local };
    assert_eq!(1, stat_local.vars.size);
    let local = unsafe { *stat_local.vars.data.add(0) };
    let annotation = unsafe { (*local).annotation };
    assert!(!annotation.is_null());
    assert_eq!("string", unsafe {
      to_string_ast_node(annotation as *mut AstNode)
    });

    assert_eq!(1, stat_local.values.size);
    let expr = unsafe { *stat_local.values.data.add(0) };
    assert_eq!("'hello'", unsafe {
      to_string_ast_node(expr as *mut AstNode)
    });
  }
}

mod pretty_printer_pretty_print_type_alias_default_type_parameters {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_type_alias_default_type_parameters() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = r#"
type Packed<T = string, U = T, V... = ...boolean, W... = (T, U, V...)> = (T, U, V...)->(W...)
local a: Packed<number>
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_type_annotation_spaces_around_tokens {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_type_annotation_spaces_around_tokens() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    for code in [
      r#" local _: Type "#,
      r#" local _  : Type "#,
      r#" local _:   Type "#,
      r#" local x: Type, y = 1 "#,
      r#" local x  : Type, y = 1 "#,
    ] {
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_pretty_print_type_assertion {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_type_assertion() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = "local a = 5 :: number";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_type_function_generics {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_type_function_generics() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

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
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_pretty_print_type_function_named_arguments {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_type_function_named_arguments() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

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
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_pretty_print_type_function_return_types {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_type_function_return_types() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

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
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_pretty_print_type_function_unnamed_arguments {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_type_function_unnamed_arguments() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

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
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_pretty_print_type_functions {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_type_functions() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code =
      r#" type function foo(arg1, arg2) if arg1 == arg2 then return arg1 end return arg2 end "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_type_functions_spaces_around_tokens {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_type_functions_spaces_around_tokens() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    for code in [
      r#" type   function foo() end "#,
      r#" type function   foo() end "#,
      r#" type function foo  () end "#,
      r#" export   type function foo() end "#,
    ] {
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_pretty_print_type_packs {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_type_packs() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = r#"
type Packed<T...> = (T...)->(T...)
local a: Packed<>
local b: Packed<(number, string)>
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_type_reference_import {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_type_reference_import() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.file_resolver.source.insert(
      "game/A".to_string(),
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
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_type_reference_spaces_around_tokens {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_type_reference_spaces_around_tokens() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

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
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_pretty_print_type_table_access_modifiers {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_type_table_access_modifiers() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

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
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_pretty_print_type_table_preserve_indexer_location {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_type_table_preserve_indexer_location() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

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
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_pretty_print_type_table_preserve_original_indexer_style {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_type_table_preserve_original_indexer_style() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

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
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_pretty_print_type_table_preserve_property_definition_style {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_type_table_preserve_property_definition_style() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#"
        type Foo = {
            ["$$typeof1"]: string,
            ['$$typeof2']: string,
        }
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_type_table_semicolon_separators {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_type_table_semicolon_separators() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#"
        type Foo = {
            bar: number;
            baz: number;
        }
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_type_table_spaces_between_tokens {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_type_table_spaces_between_tokens() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

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
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_pretty_print_type_table_string_properties_spaces_between_tokens {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_type_table_string_properties_spaces_between_tokens() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#"
        type Foo = {
            [  "$$typeof1"]: string,
            ['$$typeof2'  ]: string,
            ['$$typeof2'  ]: string,
        }
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_typeof_spaces_around_tokens {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_typeof_spaces_around_tokens() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    for code in [
      r#" type X = typeof(x) "#,
      r#" type X =    typeof(x) "#,
      r#" type X = typeof   (x) "#,
      r#" type X = typeof(   x) "#,
      r#" type X = typeof(x   ) "#,
    ] {
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_pretty_print_types_preserve_parentheses_style {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_types_preserve_parentheses_style() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    for code in [
      r#" type Foo = number "#,
      r#" type Foo = (number) "#,
      r#" type Foo = ((number)) "#,
      r#" type Foo = (  (number)  ) "#,
    ] {
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_pretty_print_unary {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_unary() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = r#"
local a = 1
local b = -1
local c = true
local d = not c
local e = 'hello'
local d = #e
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_union_reverse {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_union_reverse() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = "local a: nil | number";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_union_spaces_around_tokens {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_union_spaces_around_tokens() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    for code in ["local a: string   | number", "local a: string |   number"] {
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_pretty_print_union_type_nested {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_union_type_nested() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = "local a: ((number)->(string))|((string)->(string))";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_union_type_nested_2 {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_union_type_nested_2() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = "local a: (number&string)|(string&boolean)";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_union_type_nested_3 {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_union_type_nested_3() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = "local a: nil | (string & number)";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_pretty_print_varargs {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_pretty_print_varargs() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = "local function f(...) return ... end";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_raw_strings {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_raw_strings() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = " local a = [[ hello world ]] ";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_raw_strings_with_blocks {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_raw_strings_with_blocks() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = " local a = [==[ hello world ]==] ";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_remixed_simple_class {

  use ulua_common::fflag;
  #[test]
  fn pretty_printer_remixed_simple_class() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _fflag = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

    let code = r#"
class Point
    function length(self)
        return 100
    end
    public x
    function new(): Point
        return Point { x = 0, y = 0 }
    end
    public y
end
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_repeat_until_loop {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_repeat_until_loop() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = " repeat print() until f(x) ";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_repeat_until_loop_condition_on_new_line {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_repeat_until_loop_condition_on_new_line() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#"
    repeat
        print()
    until
        f(x) "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_returns_spaces_around_tokens {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_returns_spaces_around_tokens() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    for code in [" return    1 ", " return 1   , 2 ", " return 1,  2 "] {
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        false,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_roundtrip_generic_types {

  #[test]
  fn pretty_printer_roundtrip_generic_types() {
    use ulua_ast::{
      functions::pretty_print_with_types_pretty_printer_alt_b::pretty_print_with_types_ast_stat_block,
      records::{
        allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions,
        parser::Parser,
      },
    };

    let code = r#"
        export type A<T> = {v:T, next:A<T>}
    "#;
    // Box 钉堆：AstNameTable/Lexer/Parser 捕获宿主地址，宿主移动即悬垂。
    let mut allocator = Box::new(Allocator::new());
    let mut names = AstNameTable::new(&mut allocator);

    let parse_result = Parser::parse(
      code,
      &mut names,
      &mut allocator,
      ParseOptions::default(),
    );

    assert!(parse_result.errors.is_empty());
    assert!(!parse_result.root.is_null());
    let actual = unsafe { pretty_print_with_types_ast_stat_block(&mut *parse_result.root) };
    assert_eq!(code, actual);
  }
}

mod pretty_printer_roundtrip_types {

  #[test]
  fn pretty_printer_roundtrip_types() {
    use ulua_ast::{
      functions::pretty_print_with_types_pretty_printer_alt_b::pretty_print_with_types_ast_stat_block,
      records::{
        allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions,
        parser::Parser,
      },
    };

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

    let parse_result = Parser::parse(
      code,
      &mut names,
      &mut allocator,
      ParseOptions::default(),
    );

    assert!(parse_result.errors.is_empty());
    assert!(!parse_result.root.is_null());
    let actual = unsafe { pretty_print_with_types_ast_stat_block(&mut *parse_result.root) };
    assert_eq!(code, actual);
  }
}

mod pretty_printer_simple_class_example {

  use ulua_common::fflag;
  #[test]
  fn pretty_printer_simple_class_example() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _fflag = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

    let code = r#"
class Point
    public x: number
    public y: number
    function length(self)
        return 100
    end
    function new()
        return Point { x = 0, y = 0 }
    end
end
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_simple_class_with_public_functions {

  use ulua_common::fflag;
  #[test]
  fn pretty_printer_simple_class_with_public_functions() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _fflag = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

    let code = r#"
class Point
    public function length(self)
        return 100
    end
    public x
    public function new(): Point
        return Point { x = 0, y = 0 }
    end
    public y
end
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_simple_interp_string {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_simple_interp_string() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = " local a = `hello world` ";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_single_quoted_strings {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_single_quoted_strings() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = " local a = 'hello world' ";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_spaces_between_keywords_even_if_it_pushes_the_line_estimation_off {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_spaces_between_keywords_even_if_it_pushes_the_line_estimation_off() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = " if math.abs(raySlope) < .01 then return 0 end ";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_stmt_semicolon {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_stmt_semicolon() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    for code in [" local test = 1; ", " local test = 1  ; "] {
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_string_literals {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_string_literals() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#" local S='abcdef\n\f\a\020' "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_string_literals_containing_utf8 {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_string_literals_containing_utf8() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = " local S='lalala こんにちは' ";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_strips_type_annotations {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_strips_type_annotations() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = " local s: string= 'hello there' ";
    let expected = " local s        = 'hello there' ";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(expected, result.code);
  }
}

mod pretty_printer_strips_type_assertion_expressions {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_strips_type_assertion_expressions() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = " local s= some_function() :: any+ something_else() :: number ";
    let expected = " local s= some_function()       + something_else()           ";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(expected, result.code);
  }
}

mod pretty_printer_table_literal_closing_brace_at_correct_position {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_table_literal_closing_brace_at_correct_position() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#"
        local t={
            eggs='Tasty',
            avocado='more like awesomecavo amirite'
        }
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_table_literal_multiline_with_indexers {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_table_literal_multiline_with_indexers() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#"
        local t = {
            ["my first value"] = "x";
            ["my second value"] = "y";
        }
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_table_literal_preserves_record_vs_general {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_table_literal_preserves_record_vs_general() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#" local t={['foo']='bar',quux=42} "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_table_literal_with_keyword_key {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_table_literal_with_keyword_key() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#" local t={['nil']=nil,['true']=true} "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_table_literal_with_numeric_key {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_table_literal_with_numeric_key() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#" local t={[5]='five',[6]='six'} "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_table_literal_with_semicolon_separators {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_table_literal_with_semicolon_separators() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#"
        local t = { x = 1; y = 2 }
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_table_literal_with_spaces_around_equals {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_table_literal_with_spaces_around_equals() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#"
        local t = { x    =   1  }
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_table_literal_with_spaces_around_separator {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_table_literal_with_spaces_around_separator() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#"
        local t = { x = 1  , y = 2 }
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_table_literal_with_trailing_separators {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_table_literal_with_trailing_separators() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#"
        local t = { x = 1, y = 2, }
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_table_literals {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_table_literals() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = r#" local t={1, 2, 3, foo='bar', baz=99,[5.5]='five point five', 'end'} "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_test_1 {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_test_1() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let example = r#"
local function isPortal(element)
    if type(element)~='table'then
        return false
    end

    return element.component == Core.Portal
end
"#;

    let result = pretty_print_string_view_parse_options_bool_bool(
      example,
      ParseOptions::default(),
      false,
      false,
    );
    assert_eq!(example, result.code);
  }
}

mod pretty_printer_type_alias_spaces_around_tokens {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_type_alias_spaces_around_tokens() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

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
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_type_alias_with_defaults_spaces_around_tokens {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_type_alias_with_defaults_spaces_around_tokens() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    for code in [
      r#" type Foo<X = string, Z... = ...any> = string "#,
      r#" type Foo<X   = string, Z... = ...any> = string "#,
      r#" type Foo<X =   string, Z... = ...any> = string "#,
      r#" type Foo<X = string, Z...   = ...any> = string "#,
      r#" type Foo<X = string, Z... =   ...any> = string "#,
    ] {
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_type_assertion_spaces_around_tokens {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_type_assertion_spaces_around_tokens() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = "local a = 5   :: number";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);

    let code = "local a = 5 ::   number";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_type_lists_should_be_emitted_correctly {
  //! Ported from upstream Luau doctest.
  //! Source: `tests/PrettyPrinter.test.cpp`
  use super::*;

  #[test]
  fn pretty_printer_type_lists_should_be_emitted_correctly() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

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
}

mod pretty_printer_type_packs_spaces_around_tokens {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_type_packs_spaces_around_tokens() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

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
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}

mod pretty_printer_types_should_not_be_considered_cyclic_if_they_are_not_recursive {
  //! Ported from upstream Luau doctest.
  //! Source: `tests/PrettyPrinter.test.cpp`
  use super::*;

  #[test]
  fn pretty_printer_types_should_not_be_considered_cyclic_if_they_are_not_recursive() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

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
}

mod pretty_printer_unary_spaces_around_tokens {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_unary_spaces_around_tokens() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = r#"
local _ =   -1
local _ = -  1
local _ =   not true
local _ = not   true
local _ =   #e
local _ = #  e
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_while_do_semicolon {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_while_do_semicolon() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let _fixture = Fixture::default();
    let code = r#"
        while true do
        end;
    "#;
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_while_loop {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_while_loop() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    let code = " while f(x)do print() end ";
    let result =
      pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), false, false);
    assert_eq!(code, result.code);
  }
}

mod pretty_printer_while_loop_spaces_around_tokens {
  //! Source: `tests/PrettyPrinter.test.cpp`

  #[test]
  fn pretty_printer_while_loop_spaces_around_tokens() {
    use ulua_ast::{
      functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool,
      records::parse_options::ParseOptions,
    };

    for code in [
      " while     f(x) do print() end ",
      " while f(x)    do print() end ",
      " while f(x) do    print() end ",
      " while f(x) do print()    end ",
    ] {
      let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        false,
        false,
      );
      assert_eq!(code, result.code);
    }
  }
}
