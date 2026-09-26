use alloc::string::String;
use core::ffi::c_int;
use std::{
  env::{var, var_os},
  fs::read,
  sync::OnceLock,
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
  enums::code_gen_compilation_result::CodeGenCompilationResult,
  functions::luau_codegen_supported::luau_codegen_supported,
  records::{compilation_options::CompilationOptions, compilation_result::CompilationResult},
};
use ulua_compiler::{
  functions::compile_or_throw_compiler::compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options,
  records::compile_options::CompileOptions,
};
use ulua_vm::{
  enums::lua_status::LuaStatus,
  functions::lua_l_newstate::lua_l_newstate,
  macros::lua_globalsindex::LUA_GLOBALSINDEX,
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

use crate::common::{
  functions::{
    default_codegen_options::default_codegen_options,
    default_compile_options::default_compile_options,
    dump_assembly_both_targets::dump_assembly_both_targets,
    find_conformance_source_dir::find_conformance_source_dir, init_system::init_system,
    lua_collectgarbage::lua_collectgarbage, lua_loadstring::lua_loadstring,
    lua_silence::lua_silence, safe_api,
  },
  records::state_ref::StateRef,
};

/// cpp `tests/main.cpp:44-57` 的文件静态量 `verbose` / `optimizationLevel` /
/// `codegen` / `jitInliner`，由 `tests/main.cpp:553-583` 的 argv 解析写入。
///
/// 本端口的 conformance 用例跑在 libtest/nextest 之下，没有 `tests/main.cpp` 的
/// argv 入口，因此改用同名环境变量承载：`LUAU_VERBOSE` / `LUAU_CODEGEN` /
/// `LUAU_OPTIMIZATION_LEVEL`。缺省值与 cpp 逐字对应（false / false / 1），首次访问
/// 时解析一次并用 `OnceLock` 缓存 —— 并行测试线程共享只读视图，没有 `static mut`
/// 写入竞争，也不会再出现“开关恒为缺省值、原生分支静默空转”的情况。
///
/// `jitInliner`（cpp `Conformance.test.cpp:330-331` 的 `luau_enable_jit_inliner`）
/// 本端口没有对应实现：`Luau::JitInliner`（`cpp/Inliner/`）尚未移植成任何 crate，
/// `ulua-repl-cli/src/functions/repl_main.rs:33` 记录了同一缺口，故不保留恒假的开关。
///
/// 未接通的两个 `main.cpp` 入口（保留为缺口，不是遗漏）：
/// - `--fflags=`（`main.cpp:484-516` 的 `setFastFlags` → 批量改写
///   `FValue<bool>::list` / `FValue<int>::list` 的全局值）。该批量写入的契约是
///   “仅在线程启动前、无并发 `get()` 时执行”（见
///   `ulua-common/src/records/f_value.rs` 对 `set_all_unless` 的说明）；libtest 没有
///   `main()` 那样的启动窗口，用例已经并行在跑，任何后期批量改写都是数据竞争。
///   单个用例改 flag 请继续用 `ScopedFastFlag` / `ScopedFastInt`（线程本地覆盖，
///   `crates/ulua-conformance/tests/common/records/scoped_f_value.rs`）。
/// - `Luau::assertHandler() = testAssertionHandler`（`main.cpp:531`）：上游把
///   `LUAU_ASSERT` 失败转成 doctest 的 `ADD_FAIL_AT`（记录后继续跑），libtest 没有
///   等价的“从回调里让当前用例失败但不打断执行”的机制（唯一手段是 panic，而 panic
///   会穿过 VM 的 C 边界），故沿用端口默认的断言行为。
static CONFIG: OnceLock<TestConfig> = OnceLock::new();

/// `main.cpp` 那批文件静态量的载体。
struct TestConfig {
  /// cpp: `bool verbose`
  verbose: bool,
  /// cpp: `bool codegen`
  codegen: bool,
  /// cpp: `int optimizationLevel`
  optimization_level: c_int,
}

/// cpp `main.cpp:48` 的缺省优化级别。
const DEFAULT_OPTIMIZATION_LEVEL: c_int = 1;

/// cpp `main.cpp:572` 的合法上限（`level > 2` 判负）。
const MAX_OPTIMIZATION_LEVEL: c_int = 2;

/// 布尔开关的真值判定：环境变量置了且不是显式假值即为真，
/// 对应 doctest `parseFlag` 的“出现即生效”。
fn env_on(key: &str) -> bool {
  match var_os(key) {
    Some(v) => !v.is_empty() && !matches!(v.to_str(), Some("0" | "no" | "off" | "false")),
    None => false,
  }
}

/// cpp `main.cpp:568-583`：`-On` 越界或非数字时打印同样的提示并保留缺省值。
fn env_optimization_level() -> c_int {
  let Some(raw) = var("LUAU_OPTIMIZATION_LEVEL").ok() else {
    return DEFAULT_OPTIMIZATION_LEVEL;
  };
  match raw.parse::<c_int>() {
    Ok(level @ 0..=MAX_OPTIMIZATION_LEVEL) => level,
    _ => {
      eprintln!("Optimization level must be between 0 and 2 inclusive");
      DEFAULT_OPTIMIZATION_LEVEL
    }
  }
}

fn config() -> &'static TestConfig {
  CONFIG.get_or_init(|| TestConfig {
    verbose: env_on("LUAU_VERBOSE"),
    codegen: env_on("LUAU_CODEGEN"),
    optimization_level: env_optimization_level(),
  })
}

/// cpp: `verbose`。为假时全局 `print` 被换成 `lua_silence`
/// （`Conformance.test.cpp:343-346`）。
pub fn verbose() -> bool {
  config().verbose
}

/// cpp: `codegen`。为真时才创建 native codegen 上下文并原生执行。
pub fn codegen() -> bool {
  config().codegen
}

/// cpp: `optimizationLevel`。
pub fn optimization_level() -> c_int {
  config().optimization_level
}

/// cpp 用例里的 `if (!codegen || !luau_codegen_supported()) return;`
/// （如 `Conformance.test.cpp:4464`）：返回 `true` 表示本用例无需原生执行。
///
/// 与上游的唯一差别：通过 `LUAU_CODEGEN` 显式请求原生编译、而当前配置不支持时直接
/// 失败。上游由 doctest 的 `--codegen` 保证请求即生效；这里若不拦，这些用例会停在
/// `!supported()` 分支上“看似在跑、实际零覆盖”，把原生路径的缺失藏起来。
pub fn codegen_skipped() -> bool {
  if !codegen() {
    return true;
  }
  assert!(
    luau_codegen_supported() != 0,
    "LUAU_CODEGEN was requested, but native code generation is not supported by this configuration"
  );
  false
}

/// cpp: `Conformance.test.cpp:355-358`
/// `#if defined(LUAU_ENABLE_ASAN) || defined(_NOOPT) || defined(_DEBUG) || defined(__ANDROID__)`
/// 时把 `limitedstack` 置为 `true`，让 `conformance/` 下
/// calls/errors/iter/cyield/pcall/types/coroutine 共 10 处
/// `if not limitedstack then ...` 的深度递归压力用例降级；条件不成立时*不写*这个
/// 全局（保持 key 不存在，避免污染 `_G` 枚举 —— `types.luau` 的 RTTI 比对会遍历它）。
///
/// 对应关系：`_NOOPT`/`_DEBUG` → `debug_assertions`（nextest 的 test profile 继承
/// dev，未优化）；`__ANDROID__` → `target_os = "android"`。`LUAU_ENABLE_ASAN` 在
/// 本仓无编译期对应物：ASan 只在 `fuzz/` 经 `AFL_USE_ASAN` 开启，且必然同时是
/// unoptimized/debug 构建，已被 `debug_assertions` 覆盖。
const LIMITED_STACK: bool = cfg!(debug_assertions) || cfg!(target_os = "android");

/// cpp: `validateBytecodeGraph`（`cpp/tests/Conformance.test.cpp:252-278`）。
/// 源码编译到 BytecodeBuilder 后逐函数做
/// `fromFunctionBytecode` → `toFunctionBytecode` 往返验证。
///
/// `pub(crate)`：除本 harness 的通用路径外，`conformance_huge_function` 之类的
/// 手工用例（上游不调用 `runConformance`）也要在 `luau_compile` 前显式跑一次同样的
/// 往返验证（cpp `Conformance.test.cpp:4735`）。
pub(crate) fn validate_bytecode_graph(source: &[u8], opts: &CompileOptions) {
  // Box 钉堆：AstNameTable/Lexer/Parser 捕获 allocator 地址，宿主移动即悬垂。
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);
  let parse_result = Parser::parse(source, &mut names, &mut allocator, ParseOptions::default());

  assert!(
    parse_result.errors.is_empty(),
    "parse errors in conformance source"
  );

  // opts（CompileOptions）具有 Copy 语义，按值取用即 cpp 语义的直译
  let compile_options = *opts;

  let mut bcb = BytecodeBuilder::new(None);
  compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options(
    &mut bcb,
    &parse_result,
    &mut names,
    &compile_options,
  );

  // 字符串表按原始字节传递（cpp `getStringTable()` 同样是 `vector<string_view>`）
  let strings: Vec<&[u8]> = bcb.get_string_table();
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

/// cpp `Conformance.test.cpp:388-448`：主编译结果必须为 Success；proto 失败仅容忍四种
/// 合法原因（其余即失败），其余情况以 MESSAGE 形式打印定位信息后继续。
///
/// safe：只消费 `compile_internal` 已返回的结果对象（纯 Rust 字段与 `Vec` 遍历），
/// 不触碰 `LuaState`，因此从 `run_conformance` 的 unsafe 段里上提出来。
fn assert_codegen_result(result: &CompilationResult) {
  assert_eq!(
    result.result,
    CodeGenCompilationResult::Success,
    "Unexpected main code generation failure result"
  );

  if !result.has_errors() {
    return;
  }

  for proto_failure in &result.proto_failures {
    // cpp: 这四种是合法的 proto 失败原因，仅 MESSAGE 提示；
    // 其余枚举值 FAIL（`cpp/tests/Conformance.test.cpp:398-448`）
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

/// 运行一个 conformance 夹具（cpp `runConformance`）。
///
/// C ABI 前置条件全部收口在 [`safe_api`] 门面与本函数体内的最小 `unsafe`：
/// `setup` / `yield_fn` 会被 VM 以 C 回调方式调用（`extern "C-unwind"`），
/// `initial_lua_state` 为 `None`（缺省新建）或 `newstate`/`lua_l_newstate` 交回、
/// 交由本函数用 [`StateRef`] 接管的裸句柄。
pub fn run_conformance(
  name: &str,
  setup: Option<unsafe extern "C-unwind" fn(*mut LuaState)>,
  yield_fn: Option<unsafe extern "C-unwind" fn(*mut LuaState) -> bool>,
  initial_lua_state: Option<*mut LuaState>,
  options: Option<&CompileOptions>,
  skip_codegen: bool,
  codegen_options: Option<&CompilationOptions>,
) -> StateRef {
  // cpp `main.cpp:530`：进程启动时的一次性系统初始化；本端口每个用例跑在独立线程
  // 上（MXCSR 为线程状态），故每次进入时执行。
  init_system();

  let mut path = find_conformance_source_dir();
  if !path.ends_with('/') {
    path.push('/');
  }
  path.push_str(name);

  let source = read(&path).unwrap_or_else(|_| {
        panic!(
            "File {path} is not found. Make sure you run tests from the root or specify custom directory using LUAU_CONFORMANCE_SOURCE_DIR env variable"
        )
    });

  // `None` = 调用方未预置状态（cpp 缺省 `nullptr`），本侧新建；`Some` = 接管调用方状态。
  let initial_lua_state = initial_lua_state.unwrap_or_else(lua_l_newstate);

  let global_state = StateRef::new(initial_lua_state).expect("lua state allocation failed");
  let l = global_state.as_ptr();

  // 纯 Rust 编排（安全逻辑出 unsafe）：注册表内容与几个按值取用的选项都在本帧构造，
  // 供下方 unsafe 段消费；它们不触碰 `l`，故与 C-API 调用的相对顺序无耦合。
  let mut funcs = vec![
    LuaLReg::new(b"collectgarbage", lua_collectgarbage),
    LuaLReg::new(b"loadstring", lua_loadstring),
  ];

  if !verbose() {
    funcs.push(LuaLReg::new(b"print", lua_silence));
  }

  let chunkname = format!("={name}");

  // cpp `Conformance.test.cpp:377`：`lua_CompileOptions opts = options ? *options
  // : defaultOptions();`（按值取用，不改动调用方的对象）。
  let mut options = options.copied().unwrap_or_else(default_compile_options);

  // cpp `Conformance.test.cpp:386`：`CompilationOptions nativeOpts =
  // codegenOptions ? *codegenOptions : defaultCodegenOptions();`（同样按值取用）。
  let native_opts = codegen_options
    .cloned()
    .unwrap_or_else(default_codegen_options);

  // 按 cpp 顺序先建 native 上下文再 openlibs。
  if codegen() && !skip_codegen && luau_codegen_supported() != 0 {
    safe_api::codegen_create(l);
  }

  safe_api::openlibs(l);

  safe_api::pushvalue(l, LUA_GLOBALSINDEX);
  safe_api::l_register(l, &funcs);
  safe_api::pop(l, 1);

  // 未优化 / ASan / Android 构建的 C 栈消耗更大，深度递归压力用例会栈溢出或被
  // OOM kill（cpp `Conformance.test.cpp:354-358`）。条件不成立时不写这个全局，
  // 保持 key 不存在以免污染 `types.luau` 的 `_G` 枚举。
  if LIMITED_STACK {
    safe_api::pushboolean(l, 1);
    safe_api::setglobal(l, b"limitedstack\0");
  }

  if let Some(setup_fn) = setup {
    // Safety: `setup_fn` 由调用方按 `extern "C-unwind" fn(*mut LuaState)` 契约提供，
    // `l` 是本帧刚建好、尚未 close 的存活状态。
    unsafe { setup_fn(l) };
  }

  safe_api::sandbox(l);
  safe_api::sandboxthread(l);

  safe_api::pushvalue(l, LUA_GLOBALSINDEX);
  safe_api::setfield(l, -1, b"_G\0");

  // cpp 在 `luau_compile` 前做 bytecode graph 往返验证（`cpp/tests/Conformance.test.cpp:379`）。
  // 源与编译入口（`compile`，经 `safe_api::load_source`）一致按 unchecked UTF-8
  // 透传（原 `luau_compile.rs:40` 语义），
  // literals/pm/sort 等含非 UTF-8 原始字节。`validate_bytecode_graph` 是 safe fn，
  // 只编译 `source`、不触碰 `l`。
  validate_bytecode_graph(&source, &options);

  // 加载结果 `load_result` 上提给下方分支——与 cpp 一致，非零加载**不中止**，
  // 交由后续分支走受控的错误路径（故不能改用会在失败时 panic 的 compile_and_load 门面）。
  let load_result = safe_api::load_source(l, &chunkname, &source, &mut options);

  // 主编译结果交安全断言辅助核对（`assert_codegen_result`）。
  if load_result == 0 && codegen() && !skip_codegen && luau_codegen_supported() != 0 {
    // cpp 用 `Luau::CodeGen::compile` 并检查主编译结果
    // （`cpp/tests/Conformance.test.cpp:388-392`）。
    let result = safe_api::compile_native(l, &native_opts);
    assert_codegen_result(&result);
  }

  if load_result == 0 && luau_codegen_supported() != 0 {
    // 前提（门面契约）：`l` 存活且栈顶为已加载 proto；转储只读产物、不改栈。
    dump_assembly_both_targets(l, native_opts);
  }

  // 终态 `status` 上提给下方校验段——YIELD/BREAK 时反复经 yield_fn 决定续体恢复方式。
  let mut status = if load_result == 0 {
    safe_api::resume(l, None, 0)
  } else {
    LuaStatus::ErrSyntax as c_int
  };

  while let Some(yield_cb) = yield_fn {
    if status != LuaStatus::Yield as c_int && status != LuaStatus::Break as c_int {
      break;
    }

    // Safety: `yield_cb` 由调用方按 `extern "C-unwind" fn(*mut LuaState) -> bool`
    // 契约提供，`l` 至本处仍存活（StateRef 接管）。
    let resume_error = unsafe { yield_cb(l) };
    status = if resume_error {
      safe_api::resumeerror(l)
    } else {
      safe_api::resume(l, None, 0)
    };
  }

  safe_api::c_validate(l);

  if status == 0 {
    // 栈顶应为 chunk 留下的结果串。
    let is_string = safe_api::isstring(l, -1) != 0;
    assert!(
      is_string,
      "{path}: conformance chunk must leave a string result on the stack top"
    );
    let result = safe_api::top_diag_str(l, -1);
    assert_eq!(result, "OK", "{path}: conformance chunk did not report OK");
    // 弹出刚读取的结果串，保持与 cpp 相同的栈平衡。
    safe_api::pop(l, 1);
  } else {
    // cpp `Conformance.test.cpp:501-509`：非 YIELD 时取栈顶错误串，再拼 debugtrace，
    // 一起作为失败信息抛出（doctest 的 INFO(path) 在此内联进各条断言消息）。
    let error = if status == LuaStatus::Yield as c_int {
      String::from("thread yielded unexpectedly")
    } else {
      safe_api::top_diag_str(l, -1)
    };
    let trace = safe_api::debugtrace_text(l);
    panic!("{error}\nstacktrace:\n{trace}");
  }

  global_state
}

/// cpp `Conformance.test.cpp:282-290` 的 `runConformance(name)`（其余参数取缺省值）。
///
/// Rust 没有缺省参数，上游的“同一函数 + 默认实参”在这里按调用形态拆成
/// [`run_fixture`] / [`run_fixture_setup`] / [`run_conformance`] 三个出口，
/// 三者共用同一个 `run_conformance` 实现，语义与上游逐字对应。
pub fn run_fixture(name: &str) -> StateRef {
  run_conformance(name, None, None, None, None, false, None)
}

/// cpp `Conformance.test.cpp:282-290` 的 `runConformance(name, setup)`
/// （`yield` / `initialLuaState` / `options` / `skipCodegen` / `codegenOptions` 取缺省值）。
pub fn run_fixture_setup(
  name: &str,
  setup: unsafe extern "C-unwind" fn(*mut LuaState),
) -> StateRef {
  run_conformance(name, Some(setup), None, None, None, false, None)
}
