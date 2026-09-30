//! 行为级回归测试：`Compiler::compile_expr_function` 的临时寄存器作用域。
//!
//! 对照上游 `cpp/Compiler/src/Compiler.cpp:1631-1711`（`compileExprFunction`）：
//! `RegScope rs(this);`（cpp:1633）保证常量捕获（cpp:1661-1668）借用的临时寄存器在
//! 函数返回时归还 regTop，否则同一函数内后续表达式的寄存器分配会整体后移。
//!
//! 场景取自 `cpp/tests/Compiler.test.cpp:7393` `InlineCapture` 的常量捕获用例。

extern crate alloc;

use alloc::string::String;

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_compiler::{
  functions::compile_or_throw_compiler::compile_or_throw_bytecode_builder_string_compile_options_parse_options,
  records::compile_options::CompileOptions,
};

/// cpp `tests/Compiler.test.cpp:83` `compileFunction(source, id, optimizationLevel, typeInfoLevel)`
/// 的最小版本：只保留本文件需要的三个参数。
fn dump_function(source: &str, id: u32, optimization_level: i32) -> String {
  let mut bcb = BytecodeBuilder::new(None);
  bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE);

  let options = CompileOptions {
    optimization_level,
    ..Default::default()
  };
  let source = String::from(source);
  let parse_options = ParseOptions::default();

  compile_or_throw_bytecode_builder_string_compile_options_parse_options(
    &mut bcb,
    &source,
    &options,
    &parse_options,
  );

  bcb.dump_function(id)
}

/// `return foo(42), foo(43)`：两个内联调用各自把常量落到临时寄存器（cpp:1661-1668），
/// 返回值寄存器 R1/R2 不受影响；常量依次使用 R2、R3。
#[test]
fn constant_upvalue_capture_releases_temporary_register() {
  let actual = dump_function(
    r#"
local function foo(a)
    return function() return a end
end

return foo(42), foo(43)
"#,
    2,
    2,
  );

  let expected = r#"
DUPCLOSURE R0 K0 ['foo']
LOADN R2 42
NEWCLOSURE R1 P1
CAPTURE VAL R2
LOADN R3 43
NEWCLOSURE R2 P1
CAPTURE VAL R3
RETURN R1 2
"#;
  assert_eq!(actual.trim(), expected.trim());
}

/// 两条独立的 `local` 语句各自内联一次：常量捕获的临时寄存器不得把下一条语句的
/// 分配起点抬高（`local z` 必须仍然拿到 R2）。
#[test]
fn constant_upvalue_capture_does_not_shift_later_statements() {
  let actual = dump_function(
    r#"
local function foo(a)
    return function() return a end
end

local y = foo(42)
local z = foo(43)
return y == z
"#,
    2,
    2,
  );

  let expected = r#"
DUPCLOSURE R0 K0 ['foo']
LOADN R2 42
NEWCLOSURE R1 P1
CAPTURE VAL R2
LOADN R3 43
NEWCLOSURE R2 P1
CAPTURE VAL R3
JUMPIFEQ R1 R2 L0
LOADB R3 0 +1
L0: LOADB R3 1
L1: RETURN R3 1
"#;
  assert_eq!(actual.trim(), expected.trim());
}

/// 内联帧内部：闭包的常量捕获临时寄存器必须立刻归还，否则同一内联帧里后续语句的
/// 分配起点被抬高。少了 cpp:1633 的 `RegScope`，这里会得到 `LOADB R4 / MOVE R1 R4`。
#[test]
fn constant_upvalue_capture_inside_inlined_body_is_scratch_only() {
  let actual = dump_function(
    r#"
local function foo(a)
    local f = function() return a end
    local g = (f ~= nil)
    return g
end

return foo(42)
"#,
    2,
    2,
  );

  let expected = r#"
DUPCLOSURE R0 K0 ['foo']
LOADN R3 42
NEWCLOSURE R2 P1
CAPTURE VAL R3
JUMPXEQKNIL R2 L0 NOT
LOADB R3 0 +1
L0: LOADB R3 1
L1: MOVE R1 R3
RETURN R1 1
"#;
  assert_eq!(actual.trim(), expected.trim());
}
