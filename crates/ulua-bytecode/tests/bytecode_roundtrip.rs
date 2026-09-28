//! 函数字节码 graph 往返测试，对齐 cpp `tests/BytecodeCompiler.test.cpp` 的
//! `bytecode_roundtrip` TEST_CASE：源 → 编译（opt 0..=2）→ `fromFunctionBytecode`
//! 解析成 graph → `toFunctionBytecode` 重序列化 → 指令段必须逐字节一致。
//!
//! cpp 侧共享同一个 reserializer（NEWCLOSURE 需要先前序列化的函数占位）并逐函数
//! `clearStrings`；此处保持同一形态，同时顺带覆盖 `clear_strings` 行为。

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::{
  functions::{
    from_function_bytecode::from_function_bytecode,
    to_function_bytecode_bytecode_graph::to_function_bytecode_bytecode_builder_comp_time_bc_function,
  },
  records::bytecode_builder::BytecodeBuilder,
};
use ulua_compiler::{
  functions::compile_or_throw_compiler::compile_or_throw_bytecode_builder_string_compile_options_parse_options,
  records::compile_options::CompileOptions,
};

/// 函数级 blob 的头部长度（maxstacksize/numparams/numupvalues/isvararg/flags）。
const FN_HEADER_LEN: usize = 5;

/// LEB128 varint 读取（推进 `offset`），与 cpp `readVarInt` 一致。
fn read_var_int(data: &[u8], offset: &mut usize) -> u32 {
  let mut result = 0u32;
  let mut shift = 0u32;
  loop {
    let byte = data[*offset];
    *offset += 1;
    result |= ((byte & 0x7F) as u32) << shift;
    if byte & 0x80 == 0 {
      break;
    }
    shift += 7;
  }
  result
}

/// cpp `extractCode`：跳过 5 字节函数头与类型段，取指令段原始字节。
fn extract_code(blob: &[u8]) -> &[u8] {
  let mut offset = FN_HEADER_LEN;
  let type_info_size = read_var_int(blob, &mut offset) as usize;
  offset += type_info_size;
  let code_size = read_var_int(blob, &mut offset) as usize;
  &blob[offset..offset + code_size * size_of::<u32>()]
}

/// 编译 snippet 后逐函数做 graph 往返，比对指令段字节。
fn check_roundtrip(snippet: &str) {
  for opt_level in 0..=2i32 {
    let mut bcb = BytecodeBuilder::new(None);
    bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE);
    let options = CompileOptions::new().with_optimization_level(opt_level);
    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
      &mut bcb,
      snippet,
      &options,
      &ParseOptions::default(),
    );

    let strings = bcb.get_string_table();
    // cpp：NEWCLOSURE 需要先前序列化的函数，故所有函数共享一个 reserializer
    let mut reserializer = BytecodeBuilder::new(None);

    for fi in 0..bcb.get_function_count() {
      let fn_data = bcb.get_function_data(fi);
      let mut function = from_function_bytecode(&fn_data, &strings)
        .unwrap_or_else(|| panic!("opt={opt_level} fn={fi}: 函数字节码应可解析"));
      let original = extract_code(&fn_data).to_vec();
      let dumped = to_function_bytecode_bytecode_builder_comp_time_bc_function(
        &mut reserializer,
        &mut function,
      );
      assert_eq!(
        original,
        extract_code(&dumped),
        "opt={opt_level} fn={fi}: 往返后指令段必须逐字节一致"
      );
      // StringRef 借自本轮 function/fn_data，函数出作用域前清空重序列化器的字符串表
      reserializer.clear_strings();
    }
  }
}

#[test]
fn bytecode_roundtrip() {
  let snippets = [
    r#"
        function fn(a, b)
            local extra = 0
            if a > b then extra = 1 end 
            return extra + a + b
        end
    "#,
    r#"
        function fn()
            local var = 0
            repeat var += 1 until var < 10
        end
    "#,
    r#"
        function fn()
            local var = 3
            for i = 1, 10 do
                if var > 0 then print(i) end
                var -= 1;
            end
        end
    "#,
    r#"
        function fn()
            local res = 0
            local var = 0
            repeat
                local i = 0
                repeat
                    res += i * var
                    i += 1
                until i < 5
                var += 1
            until var < 10
        end
    "#,
    r#"
        local function x()
            local a, b = f()
            return b, a
        end
    "#,
    r#"
        local function fn(n)
            if n > 0 then
                return 0, 1
            else
                local a, b = fn(n - 1)
                return a + b, fn(n)
            end
        end
    "#,
    r#"
        local function fn(a, ...)
            local b, c = ...
            local l = {...}
            return a + b + c + l[1], ...
        end
    "#,
    r#"
        local function fn(x)
            local f = function (a, b) return a .. " and " .. b .. " and agian " .. b end
            return f(x, "eleven")
        end
    "#,
    r#"
        local tt = {}
        local function fn(x)
            local t = { a = x, b = x .. 42 }
            return table.insert({t}, tt)
        end
    "#,
  ];

  for snippet in snippets {
    check_roundtrip(snippet);
  }
}
