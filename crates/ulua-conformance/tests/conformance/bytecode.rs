// 字节码产物与编译规模上限用例
// 移植自 `cpp/tests/Conformance.test.cpp`。
//
// 已知缺件（对照 cpp Conformance.test.cpp:5391）：`BytecodeDumpLinked` 用例
// 未移植——它 dump 空 `CompTimeBcFunction`/`BcFunction` 的 toString 产物，
// Rust 侧字节码 toString 通路尚未提供等价入口。

use core::ptr::null_mut;
use std::iter::once;

use crate::common::functions::c_alloc::c_free;

#[test]
fn conformance_bytecode_distribution_per_function_test() {
  use ulua_code_gen::records::function_bytecode_summary::FunctionBytecodeSummary;
  use ulua_common::enums::luau_opcode::LuauOpcode;

  use crate::common::functions::analyze_file::analyze_file;

  let source = r#"
local function first(n, p)
  local t = {}
  for i=1,p do t[i] = i*10 end

  local function inner(_,n)
    if n > 0 then
      n = n-1
      return n, unpack(t)
    end
  end
  return inner, nil, n
end

local function second(x)
 return x[1]
end
"#;

  let total_count =
    |summary: &FunctionBytecodeSummary| -> u32 { summary.get_counts(0).iter().copied().sum() };

  let summaries = analyze_file(source, 0, 1);

  assert_eq!("inner", summaries[0].get_name());
  assert_eq!(6, summaries[0].get_line());
  assert_eq!(1, summaries[0].get_count(0, LuauOpcode::LOP_LOADN as u8));
  assert_eq!(1, summaries[0].get_count(0, LuauOpcode::LOP_MOVE as u8));
  assert_eq!(1, summaries[0].get_count(0, LuauOpcode::LOP_GETUPVAL as u8));
  assert_eq!(
    1,
    summaries[0].get_count(0, LuauOpcode::LOP_GETIMPORT as u8)
  );
  assert_eq!(1, summaries[0].get_count(0, LuauOpcode::LOP_CALL as u8));
  assert_eq!(2, summaries[0].get_count(0, LuauOpcode::LOP_RETURN as u8));
  assert_eq!(
    1,
    summaries[0].get_count(0, LuauOpcode::LOP_JUMPIFNOTLT as u8)
  );
  assert_eq!(1, summaries[0].get_count(0, LuauOpcode::LOP_SUBK as u8));
  assert_eq!(
    1,
    summaries[0].get_count(0, LuauOpcode::LOP_FASTCALL1 as u8)
  );
  assert_eq!(10, total_count(&summaries[0]));

  assert_eq!("first", summaries[1].get_name());
  assert_eq!(2, summaries[1].get_line());
  assert_eq!(1, summaries[1].get_count(0, LuauOpcode::LOP_LOADNIL as u8));
  assert_eq!(2, summaries[1].get_count(0, LuauOpcode::LOP_LOADN as u8));
  assert_eq!(3, summaries[1].get_count(0, LuauOpcode::LOP_MOVE as u8));
  assert_eq!(1, summaries[1].get_count(0, LuauOpcode::LOP_SETTABLE as u8));
  assert_eq!(
    1,
    summaries[1].get_count(0, LuauOpcode::LOP_NEWCLOSURE as u8)
  );
  assert_eq!(1, summaries[1].get_count(0, LuauOpcode::LOP_RETURN as u8));
  assert_eq!(1, summaries[1].get_count(0, LuauOpcode::LOP_MULK as u8));
  assert_eq!(1, summaries[1].get_count(0, LuauOpcode::LOP_NEWTABLE as u8));
  assert_eq!(1, summaries[1].get_count(0, LuauOpcode::LOP_FORNPREP as u8));
  assert_eq!(1, summaries[1].get_count(0, LuauOpcode::LOP_FORNLOOP as u8));
  assert_eq!(1, summaries[1].get_count(0, LuauOpcode::LOP_CAPTURE as u8));
  assert_eq!(14, total_count(&summaries[1]));

  assert_eq!("second", summaries[2].get_name());
  assert_eq!(15, summaries[2].get_line());
  assert_eq!(
    1,
    summaries[2].get_count(0, LuauOpcode::LOP_GETTABLEN as u8)
  );
  assert_eq!(1, summaries[2].get_count(0, LuauOpcode::LOP_RETURN as u8));
  assert_eq!(2, total_count(&summaries[2]));

  assert_eq!("", summaries[3].get_name());
  assert_eq!(1, summaries[3].get_line());
  assert_eq!(1, summaries[3].get_count(0, LuauOpcode::LOP_RETURN as u8));
  assert_eq!(
    2,
    summaries[3].get_count(0, LuauOpcode::LOP_DUPCLOSURE as u8)
  );
  assert_eq!(
    1,
    summaries[3].get_count(0, LuauOpcode::LOP_PREPVARARGS as u8)
  );
  assert_eq!(4, total_count(&summaries[3]));
}

#[test]
fn conformance_huge_constant_table() {
  use alloc::string::String;

  use ulua_vm::macros::lua_tonumber::lua_tonumber;

  use crate::common::functions::{cold_codegen_run::cold_codegen_run, new_state::new_state};

  let mut source = String::from("function foo(...)\n");
  source.push_str("    local args = ...\n");
  source.push_str("    local t = args and {\n");

  // 生成 400 行 × 100 个 call(N.125) 片段，N 全局连续（对齐 cpp 的嵌套循环）
  source.extend((0..400).flat_map(|i| {
    (0..100)
      .map(move |k| format!("call({}.125), ", i * 100 + k))
      .chain(once("\n        ".to_owned()))
  }));

  source.push_str("    }\n");
  source.push_str("    return { a = 1, b = 2, c = 3 }\n");
  source.push_str("end\n");
  source.push_str("return foo().a + foo().b\n");

  let global_state = new_state();
  let l = global_state.as_ptr();

  // Safety: `l` 为 new_state 新建的存活状态、尚未 openlibs；resume 后栈顶为数值结果。
  unsafe {
    cold_codegen_run(l, &source, "=HugeConstantTable");

    assert_eq!(3.0, lua_tonumber!(l, -1));
  }
}

#[test]
fn conformance_huge_function() {
  use ulua_vm::macros::lua_tonumber::lua_tonumber;

  use crate::common::functions::{
    cold_codegen_run::cold_codegen_run, default_compile_options::default_compile_options,
    make_huge_function_source::make_huge_function_source, new_state::new_state,
    run_conformance::validate_bytecode_graph,
  };

  let source = make_huge_function_source();
  let global_state = new_state();
  let l = global_state.as_ptr();

  // cpp/tests/Conformance.test.cpp:4735：`validateBytecodeGraph(source, defaultOptions());`
  // 本用例不经 `runConformance`，通用往返覆盖不到；40000 个常量会逼出 JUMPKX /
  // 常量索引溢出 / fastcall 回退等最复杂的编码路径，必须显式做一次
  // fromFunctionBytecode → toFunctionBytecode 往返验证。该往返只编译 `source`、
  // 不触碰 `l`，故放在驱动状态之前，与状态操作无先后依赖。
  validate_bytecode_graph(source.as_bytes(), &default_compile_options());

  // Safety: `l` 为 new_state 新建的存活状态、尚未 openlibs；resume 后栈顶为数值结果。
  unsafe {
    cold_codegen_run(l, &source, "=HugeFunction");

    assert_eq!(42.0, lua_tonumber!(l, -1));
  }
}

#[test]
fn conformance_huge_function_load_failure() {
  use core::{ffi::c_char, slice::from_raw_parts, sync::atomic::Ordering};

  use ulua_common::functions::c_str::cstr_bytes;
  use ulua_compiler::functions::luau_compile::luau_compile;
  use ulua_vm::{
    functions::{lua_c_fullgc::lua_c_fullgc, lua_newstate::lua_newstate, luau_load::luau_load},
    macros::lua_tostring::lua_tostring,
  };

  use crate::common::{
    functions::{
      huge_function_load_failure_test_allocate::{
        HUGE_FUNCTION_LOAD_FAILURE_LARGE_ALLOCATION_COUNT,
        HUGE_FUNCTION_LOAD_FAILURE_LARGE_ALLOCATION_TO_FAIL,
        huge_function_load_failure_test_allocate,
      },
      make_huge_function_source::make_huge_function_source,
      openlibs_and_sandbox::openlibs_and_sandbox,
    },
    records::state_ref::StateRef,
  };

  let source = make_huge_function_source();
  let expected_total_large_allocations = 2usize;

  // 阶段①（cpp `Conformance.test.cpp:4861-4863`）：整体编译一次大函数，字节码在
  // 后续所有失败注入轮次里只读复用。
  let mut bytecode_size = 0usize;
  // Safety: `luau_compile` 收到 `source` 的不可变切片与空选项指针，返回非空则
  // 为 c 分配器分配的只读产物，用例末尾统一 `c_free`。
  let bytecode = unsafe {
    luau_compile(
      source.as_ptr() as *const c_char,
      source.len(),
      // FFI: c-API 要求 NULL
      null_mut(),
      &mut bytecode_size,
    )
  };
  assert!(!bytecode.is_null());
  // Safety: `bytecode` 为刚返回的非空缓冲、`bytecode_size` 字节内有效；切片在
  // `c_free` 之前只读，不越出本用例作用域。
  let bytes = unsafe { from_raw_parts(bytecode.cast::<u8>(), bytecode_size) };

  // 阶段②（cpp `:4865-4890` 的 for 循环）：逐轮指定「第 N 次大分配失败」，每轮
  // 全新状态加载同一字节码，必须报 `not enough memory`。原子 store 是安全操作，
  // 置于 unsafe 之外。
  let mut large_allocation_to_fail = 0usize;
  while large_allocation_to_fail != expected_total_large_allocations {
    HUGE_FUNCTION_LOAD_FAILURE_LARGE_ALLOCATION_TO_FAIL
      .store(large_allocation_to_fail, Ordering::SeqCst);
    HUGE_FUNCTION_LOAD_FAILURE_LARGE_ALLOCATION_COUNT.store(0, Ordering::SeqCst);

    // Safety: `lua_newstate` 的分配函数为受失败注入控制的宿主闭包（内部 c 分配），
    // ud 传 null 与其签名一致；返回空指针时 `StateRef::new` 以 None 上抛。
    let global_state = unsafe {
      StateRef::new(lua_newstate(
        Some(huge_function_load_failure_test_allocate),
        // FFI: c-API 要求 NULL
        null_mut(),
      ))
    }
    .expect("lua state allocation failed");
    let l = global_state.as_ptr();

    // Safety: `l` 为上一步新建的存活状态、尚未 openlibs；`global_state` 在本轮
    // 作用域末尾（各块之后）才析构。
    unsafe {
      openlibs_and_sandbox(l);
    }

    // Safety: `bytes` 为存活只读字节码；`luau_load` 预期失败（status 1）并把错误
    // 字符串压栈，`lua_tostring` 读 -1 即该字符串；`lua_c_fullgc` 作用于同一存活
    // 状态，回收本轮未能加载的残留。
    unsafe {
      let status = luau_load(l, "=HugeFunction", bytes, 0);
      assert_eq!(status, 1);

      assert_eq!(cstr_bytes(lua_tostring!(l, -1)), b"not enough memory");

      lua_c_fullgc(l);
    }

    large_allocation_to_fail += 1;
  }

  // Safety: `bytecode` 为 `luau_compile` 用 c 分配器分配的缓冲，所有读取已结束，
  // 此处按 cpp `:4890` 的 `free(bytecode)` 释放。
  unsafe { c_free(bytecode.cast()) };
}

#[test]
fn conformance_large_nested_closure() {
  use alloc::string::String;

  use ulua_vm::macros::lua_tonumber::lua_tonumber;

  use crate::common::functions::{cold_codegen_run::cold_codegen_run, new_state::new_state};

  const K_COUNT: usize = 2048;

  let mut source = String::new();
  source.push_str("local function test()\n");
  source.push_str("local x = 0\n");

  for n in 1..=K_COUNT {
    source.push_str(&format!("    function f{n}() x = x + 1; return {n} end\n"));
  }

  source.push_str(&format!("    return f{K_COUNT}\n"));
  source.push_str("end\n");
  source.push_str("return test()()\n");

  let global_state = new_state();
  let l = global_state.as_ptr();

  // Safety: `l` 为 new_state 新建的存活状态、尚未 openlibs；resume 后栈顶为数值结果。
  unsafe {
    cold_codegen_run(l, &source, "=LargeNestedClosure");

    assert_eq!(K_COUNT as f64, lua_tonumber!(l, -1));
  }
}

#[test]
fn conformance_same_hash() {
  use ulua_bytecode::records::{bytecode_builder::BytecodeBuilder, string_ref::StringRef};
  use ulua_vm::functions::lua_s_hash::lua_s_hash;

  // VM 侧 `lua_s_hash`：字节切片上的安全纯函数（5 份断言副本收敛到本帮助器）。
  fn vm_hash(s: &[u8]) -> u32 {
    lua_s_hash(s)
  }

  // 字节码侧 `BytecodeBuilder::getStringHash`：安全纯函数。
  fn bytecode_hash(s: &[u8]) -> u32 {
    BytecodeBuilder::get_string_hash(StringRef::from_slice(s))
  }

  // 两侧实现必须逐字一致（对齐 cpp `SameHash` 用例的 5 组 CHECK_EQ）。
  assert_eq!(vm_hash(b""), bytecode_hash(b""));
  assert_eq!(vm_hash(b"lua"), bytecode_hash(b"lua"));
  assert_eq!(vm_hash(b"luau"), bytecode_hash(b"luau"));
  assert_eq!(vm_hash(b"luaubytecode"), bytecode_hash(b"luaubytecode"));
  assert_eq!(
    vm_hash(b"luaubytecodehash"),
    bytecode_hash(b"luaubytecodehash")
  );

  // 同一全零缓冲取两个错位窗口（切片窗口，只验哈希与对齐/地址无关）：
  // 120 字节内容相同 → 哈希必须相等。
  let buf = [0u8; 128];
  assert_eq!(lua_s_hash(&buf[1..121]), lua_s_hash(&buf[2..122]));
}
