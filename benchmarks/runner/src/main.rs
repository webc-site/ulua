//! ulua 纯 Rust 进程内性能基准对比测试执行器
//!
//! 在纯 Rust 内存中对比 ulua 与 mlua (Luau C++ 解释执行与 CodeGen JIT)，
//! 动态扫描 benchmarks/cases/ 目录下的所有基准测试脚本，
//! 杜绝外部二进制冷启动与进程创建 (fork/exec) 开销，
//! 排除终端 I/O 干扰，测量极高精度的纯执行耗时。
//!
//! 分组机制（`--group=<name>`，默认 `exec`，与既有输出/`results.json` 完全一致）：
//! - `exec`：8 个纯计算用例的 interp/JIT 双模式运行耗时（原有行为）。
//! - `compile`：parse / parse+compile(bytecode) / mlua-compile 编译吞吐，
//!   用例位于 `benchmarks/compile_cases/*.luau`，结果写独立 JSON。
//! - `analysis`：ulua-analysis 前端类型检查吞吐，用例位于
//!   `benchmarks/analysis_cases/*.luau`，结果写独立 JSON。
//!
//! 追加旗标 `--alloc`：输出 ulua 侧分配次数/字节表（需
//! `--features count-alloc` 编译；默认 feature 关闭，行为与历史输出零差异）。

use std::{
  collections::BTreeMap,
  env, fs,
  hint::black_box,
  io::{Error as IoError, ErrorKind},
  panic::{AssertUnwindSafe, catch_unwind},
  path::{Path, PathBuf},
  thread::available_parallelism,
  time::Instant,
};

// `count-alloc` 开启时 MiMalloc 仅在计数模块内部使用（feature-off 时才是全局分配器类型）。
#[cfg(not(feature = "count-alloc"))]
use mimalloc::MiMalloc;
use serde::Serialize;
use ulua::Lua;
use ulua_analysis::{
  enums::solver_mode::SolverMode,
  functions::freeze::freeze,
  records::{
    config_resolver::ConfigResolver, file_resolver::FileResolver, frontend::Frontend,
    frontend_options::FrontendOptions, source_code::SourceCode, type_check_limits::TypeCheckLimits,
  },
  type_aliases::{frontend_callbacks::TaskQueue, module_name_type::ModuleName},
};
use ulua_ast::{
  enums::mode::Mode,
  records::{
    allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions, parser::Parser,
  },
};
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_common::{
  functions::is_default_enabled_flag::is_default_enabled_flag, records::f_value::FValue,
};
use ulua_compiler::{
  functions::compile_or_throw_compiler::compile_or_throw_bytecode_builder_string_compile_options_parse_options,
  records::compile_options::CompileOptions,
};
use ulua_config::records::config::Config as LuauConfig;

#[cfg(not(feature = "count-alloc"))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

/// cfg 门控的分配计数包装器（`--features count-alloc` 启用）。
///
/// 口径：只统计 Rust 侧 `GlobalAlloc` 请求（`alloc`/`alloc_zeroed`/`realloc` 各计
/// 一次，字节数按新布局大小）；mlua vendored Luau C++ 直接走 C `malloc`，不经
/// Rust 全局分配器，故分配计数只报 ulua 侧。默认（feature 关闭）不编译本模块，
/// 全局分配器保持裸 `MiMalloc`，既有对比行为与耗时完全不变。
#[cfg(feature = "count-alloc")]
mod alloc_count {
  use std::{
    alloc::{GlobalAlloc, Layout},
    sync::atomic::{AtomicU64, Ordering},
  };

  use mimalloc::MiMalloc;

  pub static CALLS: AtomicU64 = AtomicU64::new(0);
  pub static BYTES: AtomicU64 = AtomicU64::new(0);

  pub struct Counting;

  unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
      CALLS.fetch_add(1, Ordering::Relaxed);
      BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
      unsafe { MiMalloc.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
      CALLS.fetch_add(1, Ordering::Relaxed);
      BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
      unsafe { MiMalloc.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
      CALLS.fetch_add(1, Ordering::Relaxed);
      BYTES.fetch_add(new_size as u64, Ordering::Relaxed);
      unsafe { MiMalloc.realloc(ptr, layout, new_size) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
      unsafe { MiMalloc.dealloc(ptr, layout) }
    }
  }

  /// 当前累计 (次数, 字节)。
  pub fn snapshot() -> (u64, u64) {
    (CALLS.load(Ordering::Relaxed), BYTES.load(Ordering::Relaxed))
  }
}

#[cfg(feature = "count-alloc")]
#[global_allocator]
static GLOBAL: alloc_count::Counting = alloc_count::Counting;

/// 未指定 `--runs` 时每个「用例 × 引擎」组合的测量次数。
const DEFAULT_RUNS: usize = 5;
/// 未指定 `--json` 时的结果输出路径。
const DEFAULT_JSON_OUT: &str = "benchmarks/results.json";
/// 主表耗时列保留的精度（0.1 ms）。
const MS_RESOLUTION: f64 = 10.0;
/// 几何平均比率保留的精度（两位小数）。
const RATIO_RESOLUTION: f64 = 100.0;
/// 秒 → 毫秒。
const MS_PER_SEC: f64 = 1e3;
/// JSON 里的平台标签：说明测量口径（进程内、无 fork/exec）。
const PLATFORM: &str = "Pure Rust In-Memory (No Process Overhead)";
/// 基准单位（毫秒），随用例一起导出给前端。
const UNIT_MS: &str = "ms";
/// 相对 ulua 解释执行的基线引擎 key（其比率恒为 1.0）。
const BASELINE_KEY: &str = "ulua";
/// 未指定 `--group` 时的分组：既有的 8 用例执行吞吐，行为与历史输出完全一致。
const DEFAULT_GROUP: &str = "exec";
/// `--json` 未显式给出时，编译吞吐组的结果输出路径（不污染 `results.json`）。
const COMPILE_JSON_OUT: &str = "benchmarks/results-compile.json";
/// `--json` 未显式给出时，类型检查组的结果输出路径。
const ANALYSIS_JSON_OUT: &str = "benchmarks/results-analysis.json";

/// 命令行解析出的运行配置。
struct Config {
  runs: usize,
  json_out: String,
  /// 用户是否显式给出 `--json`（分组默认写各自的独立结果路径）。
  json_custom: bool,
  /// 用例 id 过滤器；为空表示全部。
  filters: Vec<String>,
  /// 测试分组：`exec`（默认，既有行为）| `compile` | `analysis`。
  group: String,
  /// 是否输出 ulua 侧分配计数表（需 `--features count-alloc` 编译）。
  alloc: bool,
}

/// 基准测试用例元数据
struct BenchMeta {
  id: String,
  src: String,
}

/// 执行器函数签名
type RunnerFn = fn(&str) -> Result<(), String>;

/// 评测引擎定义。`runner` 为 `None` 表示无实测实现、只有静态参考值。
struct EngineDef {
  id: &'static str,
  key: &'static str,
  label: &'static str,
  lang: &'static str,
  mode: &'static str, // "interp" | "jit"
  color: &'static str,
  is_ulua: bool,
  is_reference: bool,
  runner: Option<RunnerFn>,
}

/// 可实测引擎：定义 + runner 一次配对，避免主循环里逐次 `Option::expect`。
type LiveEngine = (&'static EngineDef, RunnerFn);

#[derive(Serialize, Clone)]
struct BenchmarkItem {
  id: String,
  name: String,
  unit: &'static str,
}

#[derive(Serialize, Clone)]
struct EngineItem {
  id: &'static str,
  key: &'static str,
  label: &'static str,
  lang: &'static str,
  mode: &'static str,
  color: &'static str,
  is_ulua: bool,
  is_reference: bool,
}

#[derive(Serialize, Clone, Debug)]
struct EnvironmentInfo {
  os: String,
  arch: String,
  cpu: String,
  cores: usize,
  ram_gb: usize,
  summary: String,
}

#[derive(Serialize)]
struct BenchmarkOutput {
  timestamp: String,
  platform: &'static str,
  environment: EnvironmentInfo,
  runs: usize,
  benchmarks: Vec<BenchmarkItem>,
  engines: Vec<EngineItem>,
  data: BTreeMap<String, BTreeMap<String, f64>>,
  /// 静态参考值（非实测），与 data 分列，读者须能区分实测对比与文档性参考
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  reference: BTreeMap<String, BTreeMap<String, f64>>,
  geomean_vs_ulua: BTreeMap<String, f64>,
}

/// ulua 与 mlua 两侧的 runner 都新建一个宿主状态再执行脚本：`Lua::new()` 的
/// 内置库注册开销对所有被测引擎完全对称，比值不受影响，而「开一个 state 跑
/// 一段脚本」正是嵌入方的真实成本，故刻意留在计时窗口内。脚本源码在扫描阶段
/// 一次性读入内存，文件 I/O 与进程创建都在窗口之外。
fn run_ulua_interp(src: &str) -> Result<(), String> {
  let lua = Lua::new();
  lua.load(src).exec().map_err(|e| format!("{e:?}"))
}

fn run_ulua_jit(src: &str) -> Result<(), String> {
  let lua = Lua::new();
  lua.enable_jit(true).map_err(|e| format!("{e:?}"))?;
  lua.load(src).exec().map_err(|e| format!("{e:?}"))
}

fn run_mlua_interp(src: &str) -> Result<(), String> {
  let lua = mlua::Lua::new();
  lua.enable_jit(false);
  lua.load(src).exec().map_err(|e| format!("{e:?}"))
}

fn run_mlua_jit(src: &str) -> Result<(), String> {
  let lua = mlua::Lua::new();
  lua.enable_jit(true);
  lua.load(src).exec().map_err(|e| format!("{e:?}"))
}

// ---------------------------------------------------------------------------
// 编译吞吐组（--group=compile）runner：全部走各 crate 的公开 API。
// ---------------------------------------------------------------------------

/// 与 ulua-compile / ulua-analyze CLI 启动一致：把默认开启的 Luau FFlag
/// 归一为 true（默认关闭的不动），避免 fflag 状态影响编译/分析路径。
fn apply_luau_flags_default() {
  FValue::<bool>::set_all_unless(true, |name| !is_default_enabled_flag(name));
}

/// 纯解析（ulua-ast `Parser::parse`，含 Luau 类型语法），不产出字节码。
///
/// `Allocator` 经 `Box` 钉堆（`AstNameTable`/`Parser` 捕获宿主地址，移动即悬垂），
/// 与 `ulua_compiler::parse_pinned` 内部同一契约，此处直接复用公开 API 组合。
fn run_ulua_parse(src: &str) -> Result<(), String> {
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);
  let result = Parser::parse(src, &mut names, &mut allocator, ParseOptions::default());
  if !result.errors.is_empty() {
    return Err(format!("解析失败: {} 处语法错误", result.errors.len()));
  }
  Ok(())
}

/// parse + compile 到字节码（ulua-compiler 公开入口，与 luau-compile 管线同源）。
///
/// 入口在语法错误时 `panic_any(ParseErrors)`（cpp `throw` 的保真直译），
/// 与 CLI 一样以 `catch_unwind` 收口为失败值。
fn run_ulua_parse_compile(src: &str) -> Result<(), String> {
  let outcome = catch_unwind(AssertUnwindSafe(|| {
    let mut bcb = BytecodeBuilder::new(None);
    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
      &mut bcb,
      src,
      &CompileOptions::default(),
      &ParseOptions::default(),
    );
  }));
  match outcome {
    Ok(()) => Ok(()),
    Err(_) => Err("编译失败（panic: ParseErrors/CompileError）".to_owned()),
  }
}

/// mlua/luau（Luau C++）的 parse+compile 对照：`into_function` 完成源码到
/// 字节码的编译但不执行脚本体。
fn run_mlua_compile(src: &str) -> Result<(), String> {
  let lua = mlua::Lua::new();
  let function = lua
    .load(src)
    .into_function()
    .map_err(|e| format!("{e:?}"))?;
  black_box(&function);
  Ok(())
}

/// 分组内单个引擎：JSON key、表头标签、runner 与「是否 ulua 侧」（分配计数只覆盖 ulua 侧）。
struct GroupEngine {
  key: &'static str,
  label: &'static str,
  runner: RunnerFn,
  ulua: bool,
}

/// 测试分组定义：用例目录、扩展名、引擎集合与独立结果 JSON 路径。
/// `exec` 组沿用历史主流程，不走本结构；`compile`/`analysis` 组经 `run_group` 驱动。
struct GroupSpec {
  id: &'static str,
  title: &'static str,
  dir_candidates: &'static [&'static str],
  ext: &'static str,
  engines: &'static [GroupEngine],
  json_out: &'static str,
}

const COMPILE_GROUP: GroupSpec = GroupSpec {
  id: "compile",
  title: "编译吞吐 (parse / parse+compile 到字节码)",
  dir_candidates: &["benchmarks/compile_cases", "compile_cases"],
  ext: "luau",
  engines: &[
    GroupEngine {
      key: "ulua-parse",
      label: "ulua parse",
      runner: run_ulua_parse,
      ulua: true,
    },
    GroupEngine {
      key: "ulua-compile",
      label: "ulua parse+compile",
      runner: run_ulua_parse_compile,
      ulua: true,
    },
    GroupEngine {
      key: "mlua-compile",
      label: "mlua/luau compile",
      runner: run_mlua_compile,
      ulua: false,
    },
  ],
  json_out: COMPILE_JSON_OUT,
};

// ---------------------------------------------------------------------------
// 类型检查组（--group=analysis）：走 ulua-analysis 公开前端 API（Frontend/
// FileResolver/ConfigResolver），与 ulua-analyze CLI 同构的最小搭建。
// ---------------------------------------------------------------------------

/// 把当前被测源码喂给 Frontend 的内存版 FileResolver（实现公开 trait，无 internals）。
struct BenchFileResolver {
  source: String,
}

impl FileResolver for BenchFileResolver {
  fn read_source(&mut self, _name: &ModuleName) -> Option<SourceCode> {
    Some(SourceCode {
      source: self.source.clone(),
      r#type: SourceCode::MODULE,
    })
  }
}

/// `#[repr(C)]` 的 `base` 首字段布局：vtable 回调收到的 `this` 即整体实例指针，
/// 与 ulua-analyze-cli 的 `CliConfigResolver` 同一契约（本 crate 无法复用其私有类型）。
#[repr(C)]
struct BenchConfigResolver {
  base: ConfigResolver,
  config: *const LuauConfig,
}

/// `getConfig` 静态实现：恒定返回构造期 `Box::leak` 的固定模式配置。
///
/// # Safety
/// `this` 必须是持有本实例 `base` 槽位（首字段）的 `BenchConfigResolver` 指针；
/// `config` 指向泄漏的 `Config`，比任何调用点长寿。
unsafe fn bench_get_config(
  this: *const ConfigResolver,
  _name: *const ModuleName,
  _limits: *const TypeCheckLimits,
) -> *const LuauConfig {
  // Safety: 契约见上——`base` 为 `#[repr(C)]` 首字段，指针回推整体安全；
  // `config` 来自 `Box::leak`，地址与内容恒定。
  unsafe { (*this.cast::<BenchConfigResolver>()).config }
}

impl BenchConfigResolver {
  fn new(mode: Mode) -> Self {
    // bench 进程定位：模式配置只读、每 runner 至多一次泄漏（数十字节量级，
    // 进程随测毕退出），换取 `getConfig` 返回引用的地址稳定。
    let config: &'static LuauConfig = Box::leak(Box::new(LuauConfig {
      mode,
      ..Default::default()
    }));
    Self {
      base: ConfigResolver {
        get_config: Some(bench_get_config),
      },
      config,
    }
  }
}

/// Frontend 搭建（可选执行模块检查）：`check=false` 时只注册/冻结内置全局，
/// 作为 `check=true` 的固定开销基线。resolver 均为本地且后于 frontend 声明，
/// frontend（Box）先析构，满足 Frontend 持有裸指针的长寿契约。
fn analysis_common(src: &str, check: bool) -> Result<(), String> {
  let mut file_resolver = BenchFileResolver {
    source: src.to_owned(),
  };
  let mut config_resolver = BenchConfigResolver::new(Mode::Strict);
  let mut frontend = Frontend::new_boxed(
    SolverMode::New,
    &mut file_resolver,
    Some(&mut config_resolver.base),
    FrontendOptions::default(),
  );

  frontend.register_builtin_globals(false);
  freeze(frontend.globals.global_types_mut());

  if check {
    let module = ModuleName::from("BenchModule");
    frontend.queue_module_check_vector_module_name(&[module]);
    let execute_tasks: TaskQueue = Box::new(|tasks, run| {
      for task in tasks {
        run(task);
      }
    });
    let checked = frontend.check_queued_modules(None, execute_tasks, |_done, _total| true);
    if checked.is_empty() {
      return Err("类型检查未产出任何模块（源码可能未通过解析）".to_owned());
    }
  }
  Ok(())
}

/// 仅内置全局注册 + 冻结：分析组的固定开销基线。
fn run_ulua_analysis_globals(src: &str) -> Result<(), String> {
  analysis_common(src, false)
}

/// 全量：内置全局 + strict 模式类型检查被测模块。
fn run_ulua_analysis_check(src: &str) -> Result<(), String> {
  analysis_common(src, true)
}

const ANALYSIS_GROUP: GroupSpec = GroupSpec {
  id: "analysis",
  title: "类型检查吞吐 (ulua-analysis 前端, strict 模式)",
  dir_candidates: &["benchmarks/analysis_cases", "analysis_cases"],
  ext: "luau",
  engines: &[
    GroupEngine {
      key: "ulua-analysis-globals",
      label: "globals 基线",
      runner: run_ulua_analysis_globals,
      ulua: true,
    },
    GroupEngine {
      key: "ulua-analysis-check",
      label: "globals+检查",
      runner: run_ulua_analysis_check,
      ulua: true,
    },
  ],
  json_out: ANALYSIS_JSON_OUT,
};

const ENGINES: &[EngineDef] = &[
  EngineDef {
    id: "ulua",
    key: "ulua",
    label: "ulua",
    lang: "Luau (纯 Rust)",
    mode: "interp",
    color: "#0969da",
    is_ulua: true,
    is_reference: false,
    runner: Some(run_ulua_interp),
  },
  EngineDef {
    id: "ulua_jit",
    key: "ulua-jit",
    label: "ulua (JIT)",
    lang: "Luau (纯 Rust)",
    mode: "jit",
    color: "#0284c7",
    is_ulua: true,
    is_reference: false,
    runner: Some(run_ulua_jit),
  },
  EngineDef {
    id: "mlua_luau",
    key: "mlua/luau",
    label: "mlua/luau",
    lang: "Luau (C++)",
    mode: "interp",
    color: "#1f883d",
    is_ulua: false,
    is_reference: false,
    runner: Some(run_mlua_interp),
  },
  EngineDef {
    id: "mlua_luau_jit",
    key: "mlua/luau-jit",
    label: "mlua/luau (JIT)",
    lang: "Luau (C++)",
    mode: "jit",
    color: "#d97706",
    is_ulua: false,
    is_reference: false,
    runner: Some(run_mlua_jit),
  },
  EngineDef {
    id: "mlua_luajit",
    key: "mlua/luajit",
    label: "LuaJIT (JIT)",
    lang: "LuaJIT 2.1",
    mode: "jit",
    color: "#8250df",
    is_ulua: false,
    is_reference: true,
    runner: None,
  },
  EngineDef {
    id: "mlua_luajit_interp",
    key: "mlua/luajit-interp",
    label: "LuaJIT (解释)",
    lang: "LuaJIT 2.1",
    mode: "interp",
    color: "#6366f1",
    is_ulua: false,
    is_reference: true,
    runner: None,
  },
  EngineDef {
    id: "mlua_lua54",
    key: "mlua/lua5.4",
    label: "Lua 5.4",
    lang: "Lua 5.4",
    mode: "interp",
    color: "#64748b",
    is_ulua: false,
    is_reference: true,
    runner: None,
  },
];

/// 静态校准参考值（非实测，Apple Silicon arm64 基线）：`引擎 key → [(用例 id, ms)]`。
/// 只作为无实测引擎的文档性对照，与实测 `data` 分列。
const REFERENCE: &[(&str, &[(&str, f64)])] = &[
  (
    "mlua/luajit",
    &[
      ("fib", 45.1),
      ("nbody", 52.4),
      ("mandel", 38.2),
      ("matmul", 41.2),
      ("tablesort", 44.5),
      ("strings", 44.7),
      ("binarytrees", 48.5),
      ("spectralnorm", 41.6),
    ],
  ),
  (
    "mlua/luajit-interp",
    &[
      ("fib", 109.0),
      ("nbody", 490.8),
      ("mandel", 345.2),
      ("matmul", 250.0),
      ("tablesort", 52.0),
      ("strings", 105.3),
      ("binarytrees", 120.5),
      ("spectralnorm", 1850.0),
    ],
  ),
  (
    "mlua/lua5.4",
    &[
      ("fib", 143.6),
      ("nbody", 734.6),
      ("mandel", 341.1),
      ("matmul", 434.2),
      ("tablesort", 58.5),
      ("strings", 92.7),
      ("binarytrees", 141.6),
      ("spectralnorm", 1689.7),
    ],
  ),
];

/// 可实测引擎列表（`ENGINES` 中带 runner 的子集），一次构造、全程复用。
fn live_engines() -> Vec<LiveEngine> {
  ENGINES
    .iter()
    .filter_map(|e| e.runner.map(|r| (e, r)))
    .collect()
}

/// 扫描 `cases_dir` 下指定扩展名（如 `lua`/`luau`）的用例，按 id 排序返回（源码一次性读入内存）。
fn scan_cases(cases_dir: &Path, ext: &str) -> Result<Vec<BenchMeta>, IoError> {
  let mut cases = Vec::new();
  for entry in fs::read_dir(cases_dir)? {
    let path = entry?.path();
    if path.extension().is_none_or(|e| e != ext) {
      continue;
    }
    let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
      continue;
    };
    // 读不进的用例（权限/竞态删除）跳过，但留一行说明，不静默吞掉。
    match fs::read_to_string(&path) {
      Ok(src) => cases.push(BenchMeta {
        id: stem.to_owned(),
        src,
      }),
      Err(err) => eprintln!("跳过无法读取的用例 {}: {err}", path.display()),
    }
  }
  cases.sort_unstable_by(|a, b| a.id.cmp(&b.id));
  Ok(cases)
}

/// 中位数（毫秒）；空样本返回 0.0。
fn median(mut list: Vec<f64>) -> f64 {
  let n = list.len();
  if n == 0 {
    return 0.0;
  }
  list.sort_unstable_by(f64::total_cmp);
  let mid = n / 2;
  if n.is_multiple_of(2) {
    (list[mid - 1] + list[mid]) / 2.0
  } else {
    list[mid]
  }
}

/// 几何平均值（对数域求和，免连乘溢出）；忽略非正样本，全空返回 0.0。
fn geomean(times: impl Iterator<Item = f64>) -> f64 {
  let (mut sum_ln, mut kept) = (0.0f64, 0usize);
  for t in times {
    if t > 0.0 {
      sum_ln += t.ln();
      kept += 1;
    }
  }
  if kept == 0 {
    return 0.0;
  }
  (sum_ln / kept as f64).exp()
}

/// 统一测试测量器：一次预热 + `runs` 次测量取中位数（毫秒）。
///
/// 计时只用 `Instant`（单调时钟、纳秒级分辨率）；`coarsetime` 的 ~1µs 粒度面向
/// 墙上时间戳，不适合区段测量（墙上时间戳由 `jiff` 负责）。`black_box` 同时喂入
/// 源码与结果，防止整段执行被判为死代码或被常量折叠掉。
fn measure(runner: RunnerFn, src: &str, runs: usize) -> Result<f64, String> {
  // 预热 (Warmup)：分配器与 JIT 的热身不计入样本。
  runner(src)?;

  let mut times = Vec::with_capacity(runs);
  for _ in 0..runs {
    let start = Instant::now();
    let outcome = runner(black_box(src));
    let ms = start.elapsed().as_secs_f64() * MS_PER_SEC;
    black_box(&outcome);
    outcome?;
    times.push(ms);
  }
  Ok(median(times))
}

/// 输出 ulua 侧分配计数表（`--alloc`；feature 关闭版：仅提示口径，不做任何测量）。
#[cfg(not(feature = "count-alloc"))]
fn report_allocations(
  cases: &[&BenchMeta],
  engines: &[(&'static str, RunnerFn)],
) -> Result<(), IoError> {
  let _ = (cases, engines);
  println!();
  println!(
    "分配计数未启用: 本二进制未以 `--features count-alloc` 编译，默认输出与既有对比完全一致。"
  );
  println!(
    "启用示例: cargo run --release --features count-alloc -- --alloc [--group=compile|analysis]"
  );
  Ok(())
}

/// 输出 ulua 侧分配计数表（`--alloc`；feature 开启版）。
///
/// 每用例每引擎：先预热一次，再对一次运行取 `(alloc_count::CALLS, BYTES)` 快照差。
/// 口径为 Rust 侧 `GlobalAlloc` 请求（mlua vendored Luau C++ 直连 C malloc，
/// 不经此路径，故只报 ulua 侧计数），不进入耗时表与既有 JSON。
#[cfg(feature = "count-alloc")]
fn report_allocations(
  cases: &[&BenchMeta],
  engines: &[(&'static str, RunnerFn)],
) -> Result<(), IoError> {
  println!();
  println!("ulua 侧分配计数 (口径: Rust GlobalAlloc 请求; C 侧 malloc 不计入)");
  if engines.is_empty() {
    println!("(当前分组没有可计数的 ulua 侧引擎)");
    return Ok(());
  }

  print!("{:<16}", "用例");
  for (label, _) in engines {
    print!(
      " {:>14} {:>14}",
      format!("{label} 次数"),
      format!("{label} 字节")
    );
  }
  println!();

  for case in cases {
    print!("{:<16}", case.id);
    for (label, runner) in engines {
      // 预热一次排除惰性初始化，再取单次运行的快照差。
      if let Err(err) = runner(&case.src) {
        print!(" {:>30}", format!("{label}: ERR"));
        eprintln!("\n[{}] 分配计数预热失败: {}", case.id, err);
        continue;
      }
      let (calls0, bytes0) = alloc_count::snapshot();
      let outcome = runner(black_box(&case.src));
      let (calls1, bytes1) = alloc_count::snapshot();
      match outcome {
        Ok(()) => print!(" {:>14} {:>14}", calls1 - calls0, bytes1 - bytes0),
        Err(err) => {
          print!(" {:>30}", format!("{label}: ERR"));
          eprintln!("\n[{}] 分配计数运行失败: {}", case.id, err);
        }
      }
    }
    println!();
  }
  Ok(())
}

/// 解析 `--runs` 之类的计数取值：非法值明确报警并回退默认，
/// 而不是像原实现那样静默忽略 `--runs=abc`。
fn parse_count(raw: &str, name: &str, default: usize) -> usize {
  match raw.parse::<usize>() {
    Ok(n) if n > 0 => n,
    _ => {
      eprintln!("警告: {name}={raw} 不是正整数，使用默认值 {default}");
      default
    }
  }
}

/// 极简参数解析：`--runs[= ]N`、`--json[= ]PATH`，其余非选项参数为用例 id 过滤器。
fn parse_args() -> Config {
  let mut cfg = Config {
    runs: DEFAULT_RUNS,
    json_out: DEFAULT_JSON_OUT.to_owned(),
    json_custom: false,
    filters: Vec::new(),
    group: DEFAULT_GROUP.to_owned(),
    alloc: false,
  };
  let mut args = env::args().skip(1);

  while let Some(arg) = args.next() {
    let is_option = arg.starts_with('-');
    let (flag, mut inline) = match arg.split_once('=') {
      Some((flag, val)) => (flag.to_owned(), Some(val.to_owned())),
      None => (arg.clone(), None),
    };
    // `--flag value` 分离形态：值从参数流再取一个；已内联则不消费。
    // 下一 token 以 `-` 开头视为缺值（`--json --runs 3` 不得把 "--runs"
    // 吞成输出路径、把 "3" 沦为用例过滤器）
    let mut value = || {
      inline
        .take()
        .or_else(|| args.next())
        .filter(|v| !v.starts_with('-'))
        .unwrap_or_default()
    };

    match flag.as_str() {
      "--runs" => cfg.runs = parse_count(&value(), "--runs", DEFAULT_RUNS),
      "--json" => {
        let path = value();
        if !path.is_empty() {
          cfg.json_out = path;
          cfg.json_custom = true;
        }
      }
      "--group" => {
        let group = value();
        if group.is_empty() {
          eprintln!("警告: --group 缺少取值，使用默认分组 {DEFAULT_GROUP}");
        } else {
          cfg.group = group;
        }
      }
      "--alloc" => cfg.alloc = true,
      _ if is_option => eprintln!("警告: 忽略未知参数 {arg}"),
      _ => cfg.filters.push(arg),
    }
  }
  cfg.runs = cfg.runs.max(1);
  cfg
}

/// 定位用例目录：依次尝试候选相对路径（先仓库根视角、后 runner 目录内视角），
/// 全部落空时回退第一个候选，让 `scan_cases` 报出真实错误。
fn locate_dir(candidates: &[&str]) -> PathBuf {
  let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
  candidates
    .iter()
    .map(|candidate| current_dir.join(candidate))
    .find(|dir| dir.is_dir())
    .unwrap_or_else(|| PathBuf::from(candidates.first().copied().unwrap_or("benchmarks/cases")))
}

/// 定位 exec 组 cases 目录（历史入口，保持原候选顺序）。
fn locate_cases_dir() -> PathBuf {
  locate_dir(&["benchmarks/cases", "cases"])
}

fn main() -> Result<(), IoError> {
  let cfg = parse_args();
  match cfg.group.as_str() {
    DEFAULT_GROUP => run_exec_group(&cfg),
    "compile" => run_group(&COMPILE_GROUP, &cfg),
    "analysis" => run_group(&ANALYSIS_GROUP, &cfg),
    other => Err(IoError::new(
      ErrorKind::InvalidInput,
      format!("未知分组 {other:?}（可用: exec / compile / analysis）"),
    )),
  }
}

/// 历史主流程：exec 组（8 个纯计算 Lua 用例 × interp/JIT 双模式）。
fn run_exec_group(cfg: &Config) -> Result<(), IoError> {
  let cases_dir = locate_cases_dir();

  let all_cases = scan_cases(&cases_dir, "lua").inspect_err(|err| match err.kind() {
    ErrorKind::NotFound => eprintln!("错误: 找不到用例目录 {cases_dir:?}"),
    _ => eprintln!("错误: 无法读取用例目录 {cases_dir:?}: {err}"),
  })?;
  if all_cases.is_empty() {
    return Err(IoError::other(format!(
      "未在 {cases_dir:?} 下找到任何 .lua 测试用例"
    )));
  }

  let active_cases: Vec<&BenchMeta> = if cfg.filters.is_empty() {
    all_cases.iter().collect()
  } else {
    all_cases
      .iter()
      .filter(|c| cfg.filters.iter().any(|f| f == &c.id))
      .collect()
  };
  let live = live_engines();

  println!();
  println!("ulua 纯 Rust 进程内性能基准评测 (每项迭代 {} 次)", cfg.runs);
  println!();

  // 表头
  print!("{:<12}", "用例");
  for (eng, _) in &live {
    print!(" {:>16}", eng.label);
  }
  println!();

  let mut data_map: BTreeMap<String, BTreeMap<String, f64>> = BTreeMap::new();
  // 与 `live` 同序的实测样本，按下标归属，免按键查表。
  let mut samples: Vec<Vec<f64>> = vec![Vec::new(); live.len()];

  for case in &active_cases {
    print!("{:<12}", case.id);
    let mut case_data = BTreeMap::new();

    for (slot, (eng, runner)) in live.iter().enumerate() {
      match measure(*runner, &case.src, cfg.runs) {
        Ok(ms) => {
          let rounded = (ms * MS_RESOLUTION).round() / MS_RESOLUTION;
          print!(" {:>13.1} ms", rounded);
          case_data.insert(eng.key.to_owned(), rounded);
          if ms > 0.0 {
            samples[slot].push(ms);
          }
        }
        Err(err) => {
          print!(" {:>16}", "ERR");
          eprintln!("\n[{}] 引擎 {} 运行失败: {}", case.id, eng.label, err);
        }
      }
    }
    println!();
    data_map.insert(case.id.clone(), case_data);
  }

  // 几何平均耗时
  println!();
  print!("{:<12}", "几何平均耗时");
  for time in &samples {
    print!(" {:>13.1} ms", geomean(time.iter().copied()));
  }
  println!();
  println!();

  // `--alloc`：追加 ulua 侧分配计数表（不进既有表/JSON，默认无输出差异）。
  if cfg.alloc {
    let ulua_runners: Vec<(&'static str, RunnerFn)> = live
      .iter()
      .filter(|(eng, _)| eng.is_ulua)
      .map(|(eng, runner)| (eng.label, *runner))
      .collect();
    report_allocations(&active_cases, &ulua_runners)?;
  }

  // 导出 JSON 数据供前端和 SVG 渲染
  export_json(&cfg.json_out, &active_cases, &live, data_map, cfg.runs)
}

/// 组装并写出结果 JSON。序列化与写盘的失败一律向上报错，不再静默丢弃。
fn export_json(
  path: &str,
  cases: &[&BenchMeta],
  live: &[LiveEngine],
  data_map: BTreeMap<String, BTreeMap<String, f64>>,
  runs: usize,
) -> Result<(), IoError> {
  let benchmarks: Vec<BenchmarkItem> = cases
    .iter()
    .map(|c| BenchmarkItem {
      id: c.id.clone(),
      name: c.id.clone(),
      unit: UNIT_MS,
    })
    .collect();

  let engines: Vec<EngineItem> = ENGINES.iter().map(engine_item).collect();

  // 参考表按「引擎 key → 行」查表，消掉原先三段雷同的 if 循环。
  let mut reference_map: BTreeMap<String, BTreeMap<String, f64>> = BTreeMap::new();
  for case in cases {
    let mut ref_row = BTreeMap::new();
    for (key, rows) in REFERENCE {
      if let Some(&(_, ms)) = rows.iter().find(|(id, _)| *id == case.id) {
        ref_row.insert((*key).to_owned(), ms);
      }
    }
    if !ref_row.is_empty() {
      reference_map.insert(case.id.clone(), ref_row);
    }
  }

  // 实测引擎相对 ulua 解释执行的几何平均比率（基线自身恒为 1.0）。
  let mut geomean_map: BTreeMap<String, f64> = BTreeMap::new();
  geomean_map.insert(BASELINE_KEY.to_owned(), 1.0);
  for (eng, _) in live {
    if eng.key == BASELINE_KEY {
      continue;
    }
    let ratios = cases
      .iter()
      .filter_map(|c| data_map.get(&c.id))
      .filter_map(|row| Some((*row.get(BASELINE_KEY)?, *row.get(eng.key)?)))
      .filter(|(base, mine)| *base > 0.0 && *mine > 0.0)
      .map(|(base, mine)| mine / base);
    let mean = geomean(ratios);
    if mean > 0.0 {
      geomean_map.insert(
        eng.key.to_owned(),
        (mean * RATIO_RESOLUTION).round() / RATIO_RESOLUTION,
      );
    }
  }

  let output = BenchmarkOutput {
    // 墙上时间戳用 jiff（§5 指定）；区段计时另有 `Instant`，两者用途不同。
    timestamp: jiff::Timestamp::now().to_string(),
    platform: PLATFORM,
    environment: detect_environment(),
    runs,
    benchmarks,
    engines,
    data: data_map,
    reference: reference_map,
    geomean_vs_ulua: geomean_map,
  };

  let json = sonic_rs::to_string_pretty(&output).map_err(IoError::other)?;
  fs::write(path, json)?;
  println!("✓ 性能评测结果已写入 {path}");
  Ok(())
}

/// 分组结果中的单个引擎条目（分组无 mlua 参考表，字段收敛到 key/label）。
#[derive(Serialize)]
struct GroupEngineItem {
  key: &'static str,
  label: &'static str,
}

/// 分组结果 JSON（与主 `results.json` 分文件，杜绝污染既有前端数据）。
#[derive(Serialize)]
struct GroupOutput {
  timestamp: String,
  group: &'static str,
  title: &'static str,
  environment: EnvironmentInfo,
  runs: usize,
  benchmarks: Vec<BenchmarkItem>,
  engines: Vec<GroupEngineItem>,
  data: BTreeMap<String, BTreeMap<String, f64>>,
  /// 各引擎相对分组首引擎（ulua 侧基线）的几何平均比率。
  geomean_vs_baseline: BTreeMap<String, f64>,
}

/// 通用分组主流程：扫描 → 过滤 → 预热+中位数测量 → 打表 → 写独立 JSON。
/// 测量口径与 exec 组完全一致（同 `measure`/`median`/`geomean`）。
fn run_group(spec: &GroupSpec, cfg: &Config) -> Result<(), IoError> {
  apply_luau_flags_default();
  let cases_dir = locate_dir(spec.dir_candidates);
  let all_cases = scan_cases(&cases_dir, spec.ext).inspect_err(|err| match err.kind() {
    ErrorKind::NotFound => eprintln!("错误: 找不到用例目录 {cases_dir:?}"),
    _ => eprintln!("错误: 无法读取用例目录 {cases_dir:?}: {err}"),
  })?;
  if all_cases.is_empty() {
    return Err(IoError::other(format!(
      "未在 {cases_dir:?} 下找到任何 .{} 测试用例",
      spec.ext
    )));
  }

  let active_cases: Vec<&BenchMeta> = if cfg.filters.is_empty() {
    all_cases.iter().collect()
  } else {
    all_cases
      .iter()
      .filter(|c| cfg.filters.iter().any(|f| f == &c.id))
      .collect()
  };
  if active_cases.is_empty() {
    eprintln!(
      "警告: 分组 {} 下没有匹配 {:?} 的用例（可用: {}）",
      spec.id,
      cfg.filters,
      all_cases
        .iter()
        .map(|c| c.id.as_str())
        .collect::<Vec<_>>()
        .join(", ")
    );
  }

  println!();
  println!(
    "ulua {} 基准评测 (分组 {}, 每项迭代 {} 次)",
    spec.title, spec.id, cfg.runs
  );
  println!();

  print!("{:<16}", "用例");
  for eng in spec.engines {
    print!(" {:>18}", eng.label);
  }
  println!();

  let mut data_map: BTreeMap<String, BTreeMap<String, f64>> = BTreeMap::new();
  let mut samples: Vec<Vec<f64>> = vec![Vec::new(); spec.engines.len()];

  for case in &active_cases {
    print!("{:<16}", case.id);
    let mut case_data = BTreeMap::new();
    for (slot, eng) in spec.engines.iter().enumerate() {
      match measure(eng.runner, &case.src, cfg.runs) {
        Ok(ms) => {
          let rounded = (ms * MS_RESOLUTION).round() / MS_RESOLUTION;
          print!(" {:>15.1} ms", rounded);
          case_data.insert(eng.key.to_owned(), rounded);
          if ms > 0.0 {
            samples[slot].push(ms);
          }
        }
        Err(err) => {
          print!(" {:>18}", "ERR");
          eprintln!("\n[{}] 引擎 {} 运行失败: {}", case.id, eng.label, err);
        }
      }
    }
    println!();
    data_map.insert(case.id.clone(), case_data);
  }

  println!();
  print!("{:<16}", "几何平均耗时");
  for time in &samples {
    print!(" {:>15.1} ms", geomean(time.iter().copied()));
  }
  println!();
  println!();

  // `--alloc`：追加 ulua 侧分配计数表（mlua 引擎不经 Rust 全局分配器，不列）。
  if cfg.alloc {
    let ulua_runners: Vec<(&'static str, RunnerFn)> = spec
      .engines
      .iter()
      .filter(|eng| eng.ulua)
      .map(|eng| (eng.label, eng.runner))
      .collect();
    report_allocations(&active_cases, &ulua_runners)?;
  }

  let json_path = if cfg.json_custom {
    cfg.json_out.clone()
  } else {
    spec.json_out.to_owned()
  };
  export_group_json(&json_path, spec, &active_cases, data_map, cfg.runs)
}

/// 组装并写出分组结果 JSON。
fn export_group_json(
  path: &str,
  spec: &GroupSpec,
  cases: &[&BenchMeta],
  data_map: BTreeMap<String, BTreeMap<String, f64>>,
  runs: usize,
) -> Result<(), IoError> {
  let benchmarks: Vec<BenchmarkItem> = cases
    .iter()
    .map(|c| BenchmarkItem {
      id: c.id.clone(),
      name: c.id.clone(),
      unit: UNIT_MS,
    })
    .collect();
  let engines = spec
    .engines
    .iter()
    .map(|e| GroupEngineItem {
      key: e.key,
      label: e.label,
    })
    .collect();

  let baseline = spec.engines.first().expect("分组至少有一个引擎");
  let mut geomean_map: BTreeMap<String, f64> = BTreeMap::new();
  geomean_map.insert(baseline.key.to_owned(), 1.0);
  for eng in spec.engines.iter().skip(1) {
    let ratios = cases
      .iter()
      .filter_map(|c| data_map.get(&c.id))
      .filter_map(|row| Some((*row.get(baseline.key)?, *row.get(eng.key)?)))
      .filter(|(base, mine)| *base > 0.0 && *mine > 0.0)
      .map(|(base, mine)| mine / base);
    let mean = geomean(ratios);
    if mean > 0.0 {
      geomean_map.insert(
        eng.key.to_owned(),
        (mean * RATIO_RESOLUTION).round() / RATIO_RESOLUTION,
      );
    }
  }

  let output = GroupOutput {
    timestamp: jiff::Timestamp::now().to_string(),
    group: spec.id,
    title: spec.title,
    environment: detect_environment(),
    runs,
    benchmarks,
    engines,
    data: data_map,
    geomean_vs_baseline: geomean_map,
  };

  let json = sonic_rs::to_string_pretty(&output).map_err(IoError::other)?;
  fs::write(path, json)?;
  println!("✓ {} 组评测结果已写入 {path}", spec.id);
  Ok(())
}

/// 动态检测当前主机的系统硬件与 OS 环境（基于 sysinfo 标准库）
fn detect_environment() -> EnvironmentInfo {
  // `new_all()` 内部已完成全量刷新，无需再 refresh_all（纯冗余二扫）
  let sys = sysinfo::System::new_all();

  let os_name = sysinfo::System::name().unwrap_or_else(|| env::consts::OS.to_string());
  let os_ver = sysinfo::System::os_version().unwrap_or_default();
  let os = if os_ver.is_empty() {
    os_name
  } else {
    format!("{os_name} {os_ver}")
  };

  let arch = {
    let a = sysinfo::System::cpu_arch();
    if a.is_empty() {
      env::consts::ARCH.to_string()
    } else {
      a
    }
  };
  let cpus = sys.cpus();
  let cores = if cpus.is_empty() {
    available_parallelism().map(|n| n.get()).unwrap_or(1)
  } else {
    cpus.len()
  };

  let cpu_brand = cpus
    .first()
    .map(|c| c.brand().trim())
    .filter(|s| !s.is_empty())
    .unwrap_or("");

  let cpu = if cpu_brand.is_empty() {
    format!("{cores} vCPU")
  } else {
    cpu_brand.to_string()
  };

  let ram_gb = (sys.total_memory() + (1 << 29)) / (1 << 30);

  let summary = if ram_gb > 0 {
    format!("{os} ({arch}) · {cpu} ({cores}核) · {ram_gb}GB 内存")
  } else {
    format!("{os} ({arch}) · {cpu} ({cores}核)")
  };

  EnvironmentInfo {
    os,
    arch,
    cpu,
    cores,
    ram_gb: ram_gb as usize,
    summary,
  }
}

/// `EngineDef` → 可序列化的引擎条目（字段同构，此处只做形状收敛）。
fn engine_item(e: &EngineDef) -> EngineItem {
  EngineItem {
    id: e.id,
    key: e.key,
    label: e.label,
    lang: e.lang,
    mode: e.mode,
    color: e.color,
    is_ulua: e.is_ulua,
    is_reference: e.is_reference,
  }
}
