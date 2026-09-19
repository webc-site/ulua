use alloc::string::String;
use core::{
  ffi::{c_char, c_int, c_void},
  ptr::{null, null_mut},
  str,
};
use std::{
  env::var,
  ffi::{CStr, CString},
  fs::read,
};

use ulua_ast::records::{
  allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions, parser::Parser,
};
use ulua_bytecode::{
  functions::{
    from_function_bytecode::from_function_bytecode,
    to_function_bytecode_bytecode_graph::to_function_bytecode_bytecode_builder_comp_time_bc_function,
  },
  records::bytecode_builder::BytecodeBuilder,
};
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
    assembly_options::AssemblyOptions, compilation_options::CompilationOptions,
    lowering_stats::LoweringStats,
  },
};
use ulua_compiler::{
  functions::{
    compile_or_throw_compiler::compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options,
    luau_compile::luau_compile,
  },
  records::{compile_options::CompileOptions, lua_compile_options::LuaCompileOptions},
};
use ulua_vm::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_c_validate::lua_c_validate, lua_debugtrace::lua_debugtrace, lua_isstring::lua_isstring,
    lua_l_newstate::lua_l_newstate, lua_l_openlibs::lua_l_openlibs, lua_l_register::lua_l_register,
    lua_l_sandbox::lua_l_sandbox, lua_l_sandboxthread::lua_l_sandboxthread,
    lua_pushvalue::lua_pushvalue, lua_resume::lua_resume, lua_resumeerror::lua_resumeerror,
    lua_setfield::lua_setfield, luau_load::luau_load,
  },
  macros::{lua_globalsindex::LUA_GLOBALSINDEX, lua_pop::lua_pop, lua_tostring::lua_tostring},
  records::{lua_l_reg::LuaLReg, lua_state::lua_State},
};

use crate::common::{
  functions::{
    default_codegen_options::default_codegen_options,
    find_conformance_source_dir::find_conformance_source_dir,
    lua_collectgarbage::lua_collectgarbage, lua_loadstring::lua_loadstring,
    lua_silence::lua_silence,
  },
  type_aliases::state_ref::StateRef,
};
unsafe extern "C" {
  fn free(ptr: *mut c_void);
}

/// cpp `tests/main.cpp` 的文件静态量 `verbose`/`codegen`/`optimizeLevel` 由该
/// 二进制的 argv 解析写入。本端口的 conformance 用例跑在 libtest/nextest 之下，
/// 没有 argv 入口（`tests/main.cpp` 的 Rust 对应物未挂进模块树），故这三个开关
/// 保持 cpp 的缺省值并以不可变 `static` 承载 —— 并行测试线程只读，无 `static mut`
/// 竞争。要启用 codegen 分支需先补一个写入口。
pub static VERBOSE: bool = false;
pub static CODEGEN: bool = false;
pub static OPTIMIZATION_LEVEL: c_int = 1;

fn default_lua_compile_options() -> LuaCompileOptions {
  LuaCompileOptions {
    optimization_level: OPTIMIZATION_LEVEL,
    debug_level: 1,
    type_info_level: 1,
    coverage_level: 0,
    vector_lib: null(),
    vector_ctor: null(),
    vector_type: null(),
    mutable_globals: null(),
    userdata_types: null(),
    libraries_with_known_members: null(),
    library_member_type_cb: None,
    library_member_constant_cb: None,
    disabled_builtins: null(),
  }
}

/// cpp: `validateBytecodeGraph`（`cpp/tests/Conformance.test.cpp:210`）。
/// 源码编译到 BytecodeBuilder 后逐函数做
/// `fromFunctionBytecode` → `toFunctionBytecode` 往返验证。
fn validate_bytecode_graph(source: &str, opts: &LuaCompileOptions) {
  // Box 钉堆：AstNameTable/Lexer/Parser 捕获 allocator 地址，宿主移动即悬垂。
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);
  let parse_result = Parser::parse(
    source,
    source.len(),
    &mut names,
    &mut allocator,
    ParseOptions::default(),
  );

  assert!(
    parse_result.errors.is_empty(),
    "parse errors in conformance source"
  );

  // cpp 用 `memcpy` 把 `lua_CompileOptions` 覆盖进 `CompileOptions`（布局相同）
  let compile_options = CompileOptions {
    optimization_level: opts.optimization_level,
    debug_level: opts.debug_level,
    type_info_level: opts.type_info_level,
    coverage_level: opts.coverage_level,
    vector_lib: opts.vector_lib,
    vector_ctor: opts.vector_ctor,
    vector_type: opts.vector_type,
    mutable_globals: opts.mutable_globals,
    userdata_types: opts.userdata_types,
    libraries_with_known_members: opts.libraries_with_known_members,
    library_member_type_cb: opts.library_member_type_cb,
    library_member_constant_cb: opts.library_member_constant_cb,
    disabled_builtins: opts.disabled_builtins,
  };

  let mut bcb = BytecodeBuilder::new(None);
  compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options(
    &mut bcb,
    &parse_result,
    &mut names,
    &compile_options,
  );

  let strings: Vec<&[u8]> = bcb
    .get_string_table()
    .iter()
    .map(|s| s.as_bytes())
    .collect();
  let mut reserialized = BytecodeBuilder::new(None);

  for fid in 0..bcb.get_function_count() {
    let bytecode = bcb.get_function_data(fid);
    if let Some(mut graph) = from_function_bytecode(&bytecode, &strings) {
      let function_bytecode =
        to_function_bytecode_bytecode_builder_comp_time_bc_function(&mut reserialized, &mut graph);
      assert!(
        !function_bytecode.is_empty(),
        "Failed to serialize function {fid}"
      );

      reserialized.clear_strings();
    }
  }
}

/// # Safety
/// 调用方须保证裸指针参数有效（C 侧调用契约）。
pub unsafe fn run_conformance(
  name: *const c_char,
  setup: Option<unsafe extern "C-unwind" fn(*mut lua_State)>,
  yield_fn: Option<unsafe extern "C-unwind" fn(*mut lua_State) -> bool>,
  initial_lua_state: *mut lua_State,
  options: *mut LuaCompileOptions,
  skip_codegen: bool,
  codegen_options: *mut CompilationOptions,
) -> StateRef {
  let name_str = unsafe { CStr::from_ptr(name) }
    .to_string_lossy()
    .into_owned();

  let mut path =
    var("LUAU_CONFORMANCE_SOURCE_DIR").unwrap_or_else(|_| find_conformance_source_dir());
  if path.is_empty() {
    path = "Client/Luau/tests/conformance".to_owned();
  }
  if !path.ends_with('/') {
    path.push('/');
  }
  path.push_str(&name_str);

  let source = read(&path).unwrap_or_else(|_| {
        panic!(
            "File {path} is not found. Make sure you run tests from the root or specify custom directory using LUAU_CONFORMANCE_SOURCE_DIR env variable"
        )
    });

  let initial_lua_state = if initial_lua_state.is_null() {
    lua_l_newstate()
  } else {
    initial_lua_state
  };

  let global_state = StateRef::new(initial_lua_state).expect("lua state allocation failed");
  let l = global_state.as_ptr();

  unsafe {
    if CODEGEN && !skip_codegen && luau_codegen_supported() != 0 {
      luau_codegen_create(l);
    }

    lua_l_openlibs(l);

    let mut funcs = vec![
      LuaLReg {
        name: c"collectgarbage".as_ptr(),
        func: Some(lua_collectgarbage),
      },
      LuaLReg {
        name: c"loadstring".as_ptr(),
        func: Some(lua_loadstring),
      },
    ];

    if !VERBOSE {
      funcs.push(LuaLReg {
        name: c"print".as_ptr(),
        func: Some(lua_silence),
      });
    }

    funcs.push(LuaLReg {
      name: null(),
      func: None,
    });

    lua_pushvalue(l, LUA_GLOBALSINDEX);
    lua_l_register(l, null(), funcs.as_ptr());
    lua_pop(l, 1);

    if let Some(setup_fn) = setup {
      setup_fn(l);
    }

    lua_l_sandbox(l);
    lua_l_sandboxthread(l);

    lua_pushvalue(l, LUA_GLOBALSINDEX);
    lua_setfield(l, -1, c"_G".as_ptr());

    let chunkname = CString::new(format!("={name_str}")).expect("chunk name contains nul");

    let mut local_options;
    let options = if options.is_null() {
      local_options = default_lua_compile_options();
      &mut local_options as *mut LuaCompileOptions
    } else {
      options
    };

    // cpp 在 `luau_compile` 前做 bytecode graph 往返验证
    // （`cpp/tests/Conformance.test.cpp:351`）。
    // 源与 `luau_compile` 一致按 unchecked UTF-8 透传（`luau_compile.rs:40`），
    // literals/pm/sort 等含非 UTF-8 原始字节。
    let source_str: &str = str::from_utf8_unchecked(&source);
    validate_bytecode_graph(source_str, &*options);

    let mut bytecode_size = 0usize;
    let bytecode = luau_compile(
      source.as_ptr() as *const c_char,
      source.len(),
      options,
      &mut bytecode_size,
    );
    let load_result = luau_load(l, chunkname.as_ptr(), bytecode, bytecode_size, 0);
    free(bytecode as *mut c_void);

    let native_opts = if codegen_options.is_null() {
      default_codegen_options()
    } else {
      (*codegen_options).clone()
    };

    if load_result == 0 && CODEGEN && !skip_codegen && luau_codegen_supported() != 0 {
      // cpp 用 `Luau::CodeGen::compile` 并检查主编译结果
      // （`cpp/tests/Conformance.test.cpp:354-360`）
      let result = compile_internal(&None, l, -1, &native_opts, null_mut());

      assert_eq!(
        result.result,
        CodeGenCompilationResult::Success,
        "Unexpected main code generation failure result"
      );

      if result.has_errors() {
        for proto_failure in &result.proto_failures {
          // cpp: 这四种是合法的 proto 失败原因，仅 MESSAGE 提示；
          // 其余枚举值 FAIL（`cpp/tests/Conformance.test.cpp:362-415`）
          assert!(
            matches!(
              proto_failure.result,
              CodeGenCompilationResult::CodeGenOverflowInstructionLimit
                | CodeGenCompilationResult::CodeGenOverflowBlockLimit
                | CodeGenCompilationResult::CodeGenOverflowBlockInstructionLimit
                | CodeGenCompilationResult::CodeGenLoweringFailure
            ),
            "Unexpected main code generation failure result: {:?}",
            proto_failure.result
          );

          eprintln!(
            "Function '{}':{} encountered a proto compilation failure: {:?}",
            if proto_failure.debugname.is_empty() {
              "(anonymous)"
            } else {
              &proto_failure.debugname
            },
            proto_failure.line,
            proto_failure.result
          );
        }
      }
    }

    if load_result == 0 && luau_codegen_supported() != 0 {
      let mut assembly_options = AssemblyOptions {
        target: Target::A64,
        compilation_options: native_opts.clone(),
        output_binary: false,
        include_assembly: true,
        include_ir: true,
        include_outlined_code: true,
        include_ir_types: true,
        include_ir_prefix: Default::default(),
        include_use_info: Default::default(),
        include_cfg_info: Default::default(),
        include_reg_flow_info: Default::default(),
        annotator: None,
        annotator_context: null_mut(),
      };
      let mut stats = LoweringStats {
        function_stats_flags: FunctionStatsFlags::FunctionStatsEnable as u32,
        ..Default::default()
      };
      let a64 = get_assembly(l, -1, assembly_options.clone(), &mut stats);
      assert!(!a64.is_empty());
      assert_eq!(stats.reg_alloc_errors, 0);
      assert_eq!(stats.lowering_errors, 0);

      assembly_options.target = Target::X64SystemV;
      let x64 = get_assembly(l, -1, assembly_options, &mut stats);
      assert!(!x64.is_empty());
      assert_eq!(stats.reg_alloc_errors, 0);
      assert_eq!(stats.lowering_errors, 0);
    }

    let mut status = if load_result == 0 {
      lua_resume(l, null_mut(), 0)
    } else {
      LuaStatus::ErrSyntax as c_int
    };

    while let Some(yield_fn) = yield_fn {
      if status != LuaStatus::Yield as c_int && status != LuaStatus::Break as c_int {
        break;
      }

      let resume_error = yield_fn(l);
      status = if resume_error {
        lua_resumeerror(l, null_mut())
      } else {
        lua_resume(l, null_mut(), 0)
      };
    }

    lua_c_validate(l);

    if status == 0 {
      assert!(lua_isstring(l, -1) != 0);
      let result = CStr::from_ptr(lua_tostring!(l, -1)).to_string_lossy();
      assert_eq!(result.as_ref(), "OK");
      lua_pop(l, 1);
    } else {
      let error = if status == LuaStatus::Yield as c_int {
        String::from("thread yielded unexpectedly")
      } else {
        CStr::from_ptr(lua_tostring!(l, -1))
          .to_string_lossy()
          .into_owned()
      };
      let trace = CStr::from_ptr(lua_debugtrace(l)).to_string_lossy();
      panic!("{error}\nstacktrace:\n{trace}");
    }
  }

  global_state
}
