// 原生 codegen / IR 用例（上游 TEST_CASE("Native*")、"Codegen*")）
// 移植自 `cpp/tests/Conformance.test.cpp`。

use core::{
  ffi::c_int,
  ptr::{null, null_mut},
};

use ulua_code_gen::{
  enums::code_gen_flags::CodeGenFlags, records::compilation_options::CompilationOptions,
};

use crate::common::functions::cstr::cstr;

/// 编译 `source` 为原生码并返回其字节大小（`CompilationStats::native_code_size_bytes`）。
///
/// 收口 `conformance_codegen_nop_padding_deterministic_off` 与
/// `conformance_codegen_randomize_code_size_non_decreasing` 两处同构内联闭包里的
/// `new_state → luau_codegen_create → compile_and_load → compile_internal` 四步样板；
/// 每次调用独立开/关状态机，与原闭包逐字等价。
///
/// # Safety
/// 用例独占状态机、无并发共享；`l` 借自本函数内创建的 `StateRef`，编译对象是本函数
/// 刚载入栈顶的字节码。
unsafe fn native_code_size(source: &str, options: &CompilationOptions) -> usize {
  use ulua_code_gen::{
    functions::{compile_internal::compile_internal, luau_codegen_create::luau_codegen_create},
    records::compilation_stats::CompilationStats,
  };

  use crate::common::functions::{compile_and_load::compile_and_load, new_state::new_state};

  let global_state = new_state();
  let l = global_state.as_ptr();
  let mut stats = CompilationStats::default();

  unsafe {
    luau_codegen_create(l);
    compile_and_load(l, source, "=test", None);
    let _ = compile_internal(&None, l, -1, options, &mut stats);
  }
  stats.native_code_size_bytes
}

#[test]
fn conformance_codegen_nop_padding_deterministic_off() {
  use ulua_code_gen::records::compilation_options::CompilationOptions;

  use crate::common::functions::run_conformance::codegen_skipped;

  if codegen_skipped() {
    return;
  }

  let source = r#"
        local function add(a, b) return a + b end
        return add(1, 2)
    "#;

  let compile = || unsafe { native_code_size(source, &CompilationOptions::default()) };

  assert_eq!(compile(), compile());
}

#[test]
fn conformance_codegen_randomize_code_size_non_decreasing() {
  use ulua_code_gen::records::compilation_options::CompilationOptions;

  use crate::common::functions::run_conformance::codegen_skipped;

  if codegen_skipped() {
    return;
  }

  let source = r#"
        local function classify(x)
            if x > 0 then
                return "positive"
            elseif x < 0 then
                return "negative"
            else
                return "zero"
            end
        end
        return classify(1)
    "#;

  let compile = |nop_padding: bool| unsafe {
    native_code_size(
      source,
      &CompilationOptions {
        nop_padding,
        ..Default::default()
      },
    )
  };

  assert!(compile(true) >= compile(false));
}

#[test]
fn conformance_codegen_randomize_functional_correctness() {
  use ulua_code_gen::{
    functions::{compile_internal::compile_internal, luau_codegen_create::luau_codegen_create},
    records::compilation_options::CompilationOptions,
  };
  use ulua_vm::{functions::lua_pcall::lua_pcall, macros::lua_tonumber::lua_tonumber};

  use crate::common::functions::{
    compile_and_load::compile_and_load, cstr_text::lua_tostring_text, new_state::new_state,
    openlibs_and_sandbox::openlibs_and_sandbox, run_conformance::codegen_skipped,
  };

  if codegen_skipped() {
    return;
  }

  let source = r#"
        local function add(a, b) return a + b end
        return add(10, 32)
    "#;

  let global_state = new_state();
  let l = global_state.as_ptr();

  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 为 `global_state` 借出的活跃状态机。
  unsafe {
    luau_codegen_create(l);
    openlibs_and_sandbox(l);

    compile_and_load(l, source, "=test", None);
  }

  let nop_options = CompilationOptions {
    nop_padding: true,
    ..Default::default()
  };

  // Safety: 同上；编译对象为栈顶字节码，stats 传 null 与 cpp 缺省一致。
  unsafe {
    // FFI: c-API 要求 NULL
    let _ = compile_internal(&None, l, -1, &nop_options, null_mut());
  }

  // Safety: 同上；pcall 失败分支只读本帧栈顶错误串。
  unsafe {
    let call_result = lua_pcall(l, 0, 1, 0);
    if call_result != 0 {
      let message = lua_tostring_text(l, -1);
      panic!("lua_pcall failed: {message}");
    }

    assert_eq!(42.0, lua_tonumber!(l, -1));
  }
}

#[test]
fn conformance_codegen_supported() {
  use ulua_code_gen::functions::luau_codegen_supported::luau_codegen_supported;

  use crate::common::functions::run_conformance::codegen;

  if codegen() && luau_codegen_supported() == 0 {
    eprintln!(
      "Native code generation is not supported by the current configuration and will be disabled"
    );
  }
}

#[test]
fn conformance_ir_instruction_limit() {
  use alloc::string::String;

  use ulua_code_gen::{
    enums::code_gen_compilation_result::CodeGenCompilationResult,
    functions::{compile_internal::compile_internal, luau_codegen_create::luau_codegen_create},
    records::compilation_stats::CompilationStats,
  };
  use ulua_common::fint;

  use crate::common::{
    functions::{
      compile_and_load::compile_and_load, default_codegen_options::default_codegen_options,
      new_state::new_state, openlibs_and_sandbox::openlibs_and_sandbox,
      run_conformance::codegen_skipped,
    },
    type_aliases::scoped_fast_int::ScopedFastInt,
  };

  if codegen_skipped() {
    return;
  }

  let _codegen_heuristics_instruction_limit =
    ScopedFastInt::new(&fint::CodegenHeuristicsInstructionLimit, 50_000);

  let mut source = String::new();

  for function_index in 0..100 {
    source.push_str(&format!(
      "local function fn{function_index}(...)\nif ... then\nlocal p1, p2 = ...\nlocal _ = {{\n"
    ));

    source.extend((0..100).map(|i| format!("p1*0.{i},p2+0.{i},")));

    source.push_str("}\n");
    source.push_str("return _\n");
    source.push_str("end\n");
    source.push_str("end\n");
  }

  let global_state = new_state();
  let l = global_state.as_ptr();

  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 为 `global_state` 借出的活跃状态机。
  unsafe {
    luau_codegen_create(l);

    openlibs_and_sandbox(l);

    compile_and_load(l, &source, "=HugeFunction", None);
  }

  // Safety: 同上；编译对象为栈顶大函数模块。
  let native_options = default_codegen_options();
  let mut native_stats = CompilationStats::default();
  let native_result = unsafe { compile_internal(&None, l, -1, &native_options, &mut native_stats) };

  // 断言只读取返回值与本地 stats 结构，无指针操作。
  assert_eq!(CodeGenCompilationResult::Success, native_result.result);
  assert!(native_result.has_errors());
  assert!(!native_result.proto_failures.is_empty());

  let first_failure = &native_result.proto_failures[0];
  assert_eq!(
    CodeGenCompilationResult::CodeGenOverflowInstructionLimit,
    first_failure.result
  );
  assert_ne!(-1, first_failure.line);
  assert_ne!("", first_failure.debugname);

  assert!(native_stats.functions_compiled > 0);
  assert!(native_stats.functions_compiled < 101);
}

#[test]
fn conformance_jit_inliner() {
  use core::{ffi::c_int, ptr::null_mut, sync::atomic::Ordering};

  use ulua_common::{fflag, functions::c_str::with_c_str};
  use ulua_vm::{
    enums::lua_status::LuaStatus,
    functions::{
      lua_callbacks::lua_callbacks, lua_newthread::lua_newthread, lua_resume::lua_resume,
    },
    macros::{lua_getglobal::lua_getglobal, lua_pop::lua_pop, lua_tostring::lua_tostring},
  };

  use crate::common::{
    functions::{
      conformance_jit_inliner_interrupt::{JIT_INLINER_INDEX, conformance_jit_inliner_interrupt},
      cstr_text::cstr_text,
      run_conformance::run_conformance,
    },
    records::state_ref::StateRef,
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  // cpp 同名用例还设 `LuauBytecodeFold` + `LuauVirtualBuilder` 两旗：
  // `LuauBytecodeFold` 属 Inliner 组件（本端口未移植该优化阶段，无消费点）；
  // `LuauVirtualBcBuilder` 在 fflag.rs 已声明但 VM 侧尚无读取点，设了也不生效，
  // 故均不在此设置。行为断言仅依赖 `LuauEmitCallFeedback` 的 call feedback 通路。
  let _luau_emit_call_feedback = ScopedFastFlag::new(&fflag::LuauEmitCallFeedback, true);

  let global_state: StateRef =
    run_conformance("jit_inliner.luau", None, None, None, None, true, None);
  let l = global_state.as_ptr();

  unsafe {
    (*lua_callbacks(l)).interrupt = Some(conformance_jit_inliner_interrupt);
  }

  // 对应 C++ 的 "fuzzfail_infinite" + std::to_string(test)；cpp
  // `Conformance.test.cpp:1476` 为 test <= 3。第三段 fuzzfail_infinite3
  // （cpp `tests/conformance/jit_inliner.luau:115-125`）曾在 init 裁剪时连带
  // 删除，实测当前 VM 可跑（中断计时器如预期以 timeout 终止），已恢复对齐。
  let global_name = |test: u32| format!("fuzzfail_infinite{test}");
  for test in 1..=3 {
    let t = unsafe { lua_newthread(l) };

    let name = global_name(test);
    // Safety: `t` 为存活线程；`with_c_str` 补 NUL 的临时指针在闭包调用期内被
    // `lua_getglobal` 消费（key 走 intern 表，callee 当场复制）。
    unsafe { with_c_str(name.as_bytes(), |key| lua_getglobal(t, key)) };

    JIT_INLINER_INDEX.store(0, Ordering::SeqCst);
    // FFI: c-API 要求 NULL
    let status = unsafe { lua_resume(t, null_mut(), 0) };
    assert_eq!(status, LuaStatus::ErrRun as c_int);

    let top = unsafe { lua_tostring!(t, -1) };
    assert!(!top.is_null());
    // Safety: 上一断言已排除 null，`lua_tostring!` 保证栈槽内 NUL 结尾串存活。
    let text = unsafe { cstr_text(top) };
    assert!(
      text.contains("timeout"),
      "expected timeout error, got {text}"
    );

    unsafe { lua_pop(l, 1) };
  }
}

fn run_conformance_native(debug_luau_aborting_checks: bool, optimization_level: c_int) {
  use ulua_common::fflag;
  use ulua_compiler::records::compile_options::CompileOptions;

  use crate::common::{
    functions::{
      default_compile_options::default_compile_options,
      run_conformance::{codegen_skipped, run_conformance},
      setup_native_helpers::setup_native_helpers,
    },
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  if codegen_skipped() {
    return;
  }

  let _debug_luau_aborting_checks =
    ScopedFastFlag::new(&fflag::DebugLuauAbortingChecks, debug_luau_aborting_checks);

  let copts = CompileOptions {
    optimization_level,
    ..default_compile_options()
  };

  run_conformance(
    "native.luau",
    Some(setup_native_helpers),
    None,
    None,
    Some(&copts),
    false,
    None,
  );
}

#[test]
fn conformance_native_regular_o0() {
  run_conformance_native(false, 0);
}

#[test]
fn conformance_native_regular_o1() {
  run_conformance_native(false, 1);
}

#[test]
fn conformance_native_regular_o2() {
  run_conformance_native(false, 2);
}

#[test]
fn conformance_native_checked_o0() {
  run_conformance_native(true, 0);
}

#[test]
fn conformance_native_checked_o1() {
  run_conformance_native(true, 1);
}

#[test]
fn conformance_native_checked_o2() {
  run_conformance_native(true, 2);
}

#[test]
fn conformance_native_attribute() {
  use ulua_code_gen::{
    enums::code_gen_compilation_result::CodeGenCompilationResult,
    functions::{compile_internal::compile_internal, luau_codegen_create::luau_codegen_create},
    records::compilation_stats::CompilationStats,
  };

  use crate::common::functions::{
    compile_and_load::compile_and_load, default_codegen_options::default_codegen_options,
    new_state::new_state, openlibs_and_sandbox::openlibs_and_sandbox,
    run_conformance::codegen_skipped,
  };

  if codegen_skipped() {
    return;
  }

  let source = r#"
        @native
        local function sum(x, y)
            local function sumHelper(z)
                return (x+y+z)
            end
            return sumHelper
        end

        local function sub(x, y)
            @native
            local function subHelper(z)
                return (x+y-z)
            end
            return subHelper
        end"#;

  let global_state = new_state();
  let l = global_state.as_ptr();

  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 为 `global_state` 借出的活跃状态机。
  unsafe {
    luau_codegen_create(l);

    openlibs_and_sandbox(l);

    compile_and_load(l, source, "=Code", None);
  }

  // Safety: 同上；编译对象为栈顶 @native 标注模块。
  let native_options = default_codegen_options();
  let mut native_stats = CompilationStats::default();
  let native_result = unsafe { compile_internal(&None, l, -1, &native_options, &mut native_stats) };

  // 断言只读取返回值与本地 stats 结构，无指针操作。
  assert_eq!(CodeGenCompilationResult::Success, native_result.result);
  assert!(!native_result.has_errors());
  assert!(native_result.proto_failures.is_empty());

  assert_eq!(2, native_stats.functions_compiled);
}

fn run_conformance_native_integer_spills(optimization_level: c_int) {
  use ulua_common::fflag;
  use ulua_compiler::records::compile_options::CompileOptions;

  use crate::common::{
    functions::{
      default_compile_options::default_compile_options, run_conformance::run_conformance,
    },
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _integer_type = ScopedFastFlag::new(&fflag::LuauIntegerType2, true);
  let _codegen_dse = ScopedFastFlag::new(&fflag::LuauCodegenDseRestoreHintUpdate, true);

  let copts = CompileOptions {
    optimization_level,
    ..default_compile_options()
  };

  run_conformance(
    "native_integer_spills.luau",
    None,
    None,
    None,
    Some(&copts),
    false,
    None,
  );
}

#[test]
fn conformance_native_integer_spills_o0() {
  run_conformance_native_integer_spills(0);
}

#[test]
fn conformance_native_integer_spills_o1() {
  run_conformance_native_integer_spills(1);
}

#[test]
fn conformance_native_integer_spills_o2() {
  run_conformance_native_integer_spills(2);
}

#[test]
fn conformance_native_type_annotations() {
  use crate::common::functions::{
    conformance_native_type_annotations_setup::conformance_native_type_annotations_setup,
    run_conformance::{codegen_skipped, run_fixture_setup},
  };

  if codegen_skipped() {
    return;
  }

  run_fixture_setup(
    "native_types.luau",
    conformance_native_type_annotations_setup,
  );
}

#[test]
fn conformance_native_userdata() {
  use ulua_compiler::records::compile_options::CompileOptions;

  use crate::common::functions::{
    codegen_ir_hook_callbacks::{
      userdata_access_bytecode_type_callback, userdata_access_callback,
      userdata_metamethod_bytecode_type_callback, userdata_metamethod_callback,
      userdata_namecall_bytecode_type_callback, userdata_namecall_callback,
      vector_access_bytecode_type_callback, vector_access_callback,
      vector_namecall_bytecode_type_callback, vector_namecall_callback,
    },
    conformance_native_userdata_setup::conformance_native_userdata_setup,
    default_codegen_options::default_codegen_options,
    default_compile_options::default_compile_options,
    run_conformance::run_conformance,
  };

  let userdata_compile_types = [
    cstr(b"vec2\0"),
    cstr(b"color\0"),
    cstr(b"mat3\0"),
    cstr(b"vertex\0"),
    null(),
  ];
  let userdata_run_types = [
    cstr(b"extra\0"),
    cstr(b"color\0"),
    cstr(b"vec2\0"),
    cstr(b"mat3\0"),
    cstr(b"vertex\0"),
    null(),
  ];

  for use_ir_hooks in [false, true] {
    for optimization_level in 0..=2 {
      let copts = CompileOptions {
        optimization_level,
        userdata_types: userdata_compile_types.as_ptr(),
        ..default_compile_options()
      };
      let mut native_options = default_codegen_options();

      if use_ir_hooks {
        native_options.hooks.vector_access_bytecode_type =
          Some(vector_access_bytecode_type_callback);
        native_options.hooks.vector_namecall_bytecode_type =
          Some(vector_namecall_bytecode_type_callback);
        native_options.hooks.vector_access = Some(vector_access_callback);
        native_options.hooks.vector_namecall = Some(vector_namecall_callback);

        native_options.hooks.userdata_access_bytecode_type =
          Some(userdata_access_bytecode_type_callback);
        native_options.hooks.userdata_metamethod_bytecode_type =
          Some(userdata_metamethod_bytecode_type_callback);
        native_options.hooks.userdata_namecall_bytecode_type =
          Some(userdata_namecall_bytecode_type_callback);
        native_options.hooks.userdata_access = Some(userdata_access_callback);
        native_options.hooks.userdata_metamethod = Some(userdata_metamethod_callback);
        native_options.hooks.userdata_namecall = Some(userdata_namecall_callback);

        native_options.userdata_types = userdata_run_types.as_ptr();
      }

      run_conformance(
        "native_userdata.luau",
        Some(conformance_native_userdata_setup),
        None,
        None,
        Some(&copts),
        false,
        Some(&native_options),
      );
    }
  }
}

#[test]
fn conformance_large_module_a64() {
  use alloc::string::String;

  use ulua_code_gen::{
    enums::{
      code_gen_compilation_result::CodeGenCompilationResult,
      function_stats_flags::FunctionStatsFlags, target::Target,
    },
    functions::{
      compile_internal::compile_internal, get_assembly::get_assembly,
      luau_codegen_create::luau_codegen_create, luau_codegen_supported::luau_codegen_supported,
    },
    records::{
      assembly_options::AssemblyOptions, compilation_stats::CompilationStats,
      lowering_stats::LoweringStats,
    },
  };
  use ulua_common::fflag;
  use ulua_compiler::records::compile_options::CompileOptions;
  use ulua_vm::{
    functions::{lua_l_openlibs::lua_l_openlibs, lua_resume::lua_resume},
    lua_tonumber,
  };

  use crate::common::{
    functions::{
      compile_and_load::compile_and_load, default_codegen_options::default_codegen_options,
      default_compile_options::default_compile_options, new_state::new_state,
      run_conformance::codegen,
    },
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  // 对齐 C++ Conformance.test.cpp:5016-5018 的两条 ScopedFastFlag：
  // - `LuauCodegenProtectData` 必须开启以保证数据与代码按页独立布局；
  // - `LuauCodegenA64FarRefs`（A64 远引用跳转垫片）在本端口无对应 FFlag，A64
  //   汇编器未移植该旗标门控的分支，故不设置。
  let _protect_data = ScopedFastFlag::new(&fflag::LuauCodegenProtectData, true);

  const FILLER_COUNT: usize = 2;
  const LINES_PER_FILLER: usize = 50;
  const EXPECTED_RESULT: f64 = 140_000.0;

  let mut source = String::new();
  for i in 0..FILLER_COUNT {
    source.push_str(&format!("function filler{i}(x: number)\n"));
    for k in 1..=LINES_PER_FILLER {
      source.push_str(&format!("    x = x + {k}\n"));
    }
    source.push_str("    return x\nend\n");
  }

  // 常量刻意选为无法 fmov 下沉，必须分配到数据段。
  source.push_str("function trigger(x: number)\n");
  source.push_str("    x = x + 0.1\n");
  source.push_str("    x = x + 0.3\n");
  source.push_str("    return x\n");
  source.push_str("end\n");
  source.push_str("return math.floor(trigger(1) * 100000)\n");

  let global_state = new_state();
  let l = global_state.as_ptr();

  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 为 `global_state` 借出的活跃状态机，
  // 支持位探测/代码段创建只作用于该状态。
  unsafe {
    if luau_codegen_supported() != 0 {
      luau_codegen_create(l);
    }

    lua_l_openlibs(l);
  }

  // cpp `Conformance.test.cpp:4956-4957`：`opts = defaultOptions();` 后固定 O2。
  let mut opts = CompileOptions {
    optimization_level: 2,
    ..default_compile_options()
  };

  // Safety: 同上；载入的是本用例拼接的大模块源码。
  unsafe { compile_and_load(l, &source, "=LargeModuleA64", Some(&mut opts)) };

  // cpp `Conformance.test.cpp:4964-4981`：本用例只按 A64 反汇编一次，且只开
  // `includeAssembly`（IR 相关开关保持缺省 false），双目标反汇编由
  // `run_conformance` 尾部的 `dump_assembly_both_targets` 覆盖。
  let mut stats = LoweringStats {
    function_stats_flags: FunctionStatsFlags::FunctionStatsEnable as u32,
    ..Default::default()
  };
  let mut codegen_opts = default_codegen_options();
  codegen_opts.flags = CodeGenFlags::CodeGenColdFunctions as u32;
  let assembly_options = AssemblyOptions {
    target: Target::A64,
    compilation_options: codegen_opts,
    output_binary: false,
    include_assembly: true,
    include_ir: false,
    include_outlined_code: false,
    include_ir_types: false,
    include_ir_prefix: Default::default(),
    include_use_info: Default::default(),
    include_cfg_info: Default::default(),
    include_reg_flow_info: Default::default(),
    annotator: None,
    // FFI: c-API 要求 NULL
    annotator_context: null_mut(),
  };

  // Safety: 同上；反汇编只读栈顶闭包，`stats` 为其写出的本地结构。
  unsafe {
    if luau_codegen_supported() != 0 {
      let a64 = get_assembly(l, -1, assembly_options, &mut stats);
      assert!(!a64.is_empty());
      assert_eq!(0, stats.reg_alloc_errors);
      assert_eq!(0, stats.lowering_errors);
    }
  }

  // Safety: 同上；分支内原生编译只读栈顶闭包，native_options/native_stats 为本帧
  // 局部、在 compile_internal 调用期内存活，返回的 CompilationResult 为 owned 数据。
  if codegen() && luau_codegen_supported() != 0 {
    let native_result = unsafe {
      let mut native_options = default_codegen_options();
      native_options.flags = CodeGenFlags::CodeGenColdFunctions as u32;
      let mut native_stats = CompilationStats::default();
      compile_internal(&None, l, -1, &native_options, &mut native_stats)
    };

    assert_eq!(CodeGenCompilationResult::Success, native_result.result);
    if native_result.has_errors() {
      for proto_failure in &native_result.proto_failures {
        assert!(
          matches!(
            proto_failure.result,
            CodeGenCompilationResult::CodeGenOverflowInstructionLimit
              | CodeGenCompilationResult::CodeGenOverflowBlockLimit
              | CodeGenCompilationResult::CodeGenOverflowBlockInstructionLimit
              | CodeGenCompilationResult::CodeGenLoweringFailure
          ),
          "Unexpected proto failure result: {:?}",
          proto_failure.result
        );
      }
    }
  }

  // Safety: 同上；resume 跑的是本用例载入并压栈的 main 线程。
  unsafe {
    // FFI: c-API 要求 NULL
    let status = lua_resume(l, null_mut(), 0);
    assert_eq!(0, status);
    assert_eq!(EXPECTED_RESULT, lua_tonumber!(l, -1));
  }
}
