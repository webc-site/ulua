//! ulua 纯 Rust 进程内性能基准对比测试执行器
//!
//! 在纯 Rust 内存中对比 ulua 与 C 侧引擎（按后端三选一：Luau C++ 解释/CodeGen、
//! LuaJIT 2.1 解释/JIT、PUC Lua 5.4），动态扫描 benchmarks/cases/ 目录下的所有
//! 基准测试脚本，杜绝外部二进制冷启动与进程创建 (fork/exec) 开销，排除终端
//! I/O 干扰，测量极高精度的纯执行耗时。
//!
//! 引擎后端（互斥三选一，cargo feature）：
//! - `engine-luau`   → ulua ×2 + mlua/luau ×2（Luau C++ interp / CodeGen）
//! - `engine-luajit` → ulua ×2 + LuaJIT ×2（interp / JIT）
//! - `engine-lua54`  → ulua ×2 + PUC Lua 5.4
//!
//! mlua 0.12 的 ffi 模块把各 Lua 版本的 C 符号扁平 re-export，同一二进制只能
//! 共链一种 C Lua，故后端用可选依赖 + 互斥 feature 实现；`bench.sh` 依次跑三个
//! 后端并以 `--append` 把部分结果合并进同一 results.json（网站即得全集对比）。
//!
//! 分组机制（`--group=<name>`，默认 `exec`，见 group.rs 注册表）：
//! - `exec`：纯计算用例的全引擎 interp/JIT 双模式运行耗时。
//! - `compile`：parse / parse+compile(bytecode) / mlua-compile 编译吞吐，
//!   用例位于 `benchmarks/compile_cases/*.luau`，结果写独立 JSON。
//! - `analysis`：ulua-analysis 前端类型检查吞吐，用例位于
//!   `benchmarks/analysis_cases/*.luau`，结果写独立 JSON。
//!
//! 追加旗标 `--alloc`：输出 ulua 侧分配次数/字节表（需
//! `--features count-alloc` 编译；默认 feature 关闭，行为与历史输出零差异）。
//! 追加旗标 `--engines k1,k2`：只测指定 key 的引擎（多后端补测时跳过已测列）。
//! 追加旗标 `--append`：写 JSON 前并入既有结果（按用例 × 引擎 key 并集）。

// 后端互斥守卫：多于一个后端 feature 会让两份 vendored C 库的同名 `lua_*`
// 符号共存于同一链接单元集合，解析结果不确定（静默错调度），必须编译期拒绝。
#[cfg(all(feature = "engine-luau", feature = "engine-luajit"))]
compile_error!(
  "engine-luau 与 engine-luajit 互斥: 同一二进制只能共链一种 C Lua (mlua ffi 扁平 re-export)"
);
#[cfg(all(feature = "engine-luau", feature = "engine-lua54"))]
compile_error!(
  "engine-luau 与 engine-lua54 互斥: 同一二进制只能共链一种 C Lua (mlua ffi 扁平 re-export)"
);
#[cfg(all(feature = "engine-luajit", feature = "engine-lua54"))]
compile_error!(
  "engine-luajit 与 engine-lua54 互斥: 同一二进制只能共链一种 C Lua (mlua ffi 扁平 re-export)"
);
#[cfg(not(any(
  feature = "engine-luau",
  feature = "engine-luajit",
  feature = "engine-lua54"
)))]
compile_error!("必须选择引擎后端: --features engine-luau | engine-luajit | engine-lua54");

mod alloc_count;
mod analysis;
mod case;
mod compile;
mod engine;
mod group;
mod measure;
mod output;
mod report;

use std::{
  env,
  io::{Error as IoError, ErrorKind},
};

// `count-alloc` 开启时 MiMalloc 仅在计数模块内部使用（feature-off 时才是全局分配器类型）。
#[cfg(not(feature = "count-alloc"))]
use mimalloc::MiMalloc;
use ulua_common::records::f_value;

#[cfg(not(feature = "count-alloc"))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
#[cfg(feature = "count-alloc")]
#[global_allocator]
static GLOBAL: alloc_count::Counting = alloc_count::Counting;

/// 未指定 `--runs` 时每个「用例 × 引擎」组合的测量次数。
const DEFAULT_RUNS: usize = 5;
/// 未指定 `--json` 时的结果输出路径。
const DEFAULT_JSON_OUT: &str = "benchmarks/results.json";
/// `--json` 未显式给出时，编译吞吐组的结果输出路径（不污染 `results.json`）。
const COMPILE_JSON_OUT: &str = "benchmarks/results-compile.json";
/// `--json` 未显式给出时，类型检查组的结果输出路径。
const ANALYSIS_JSON_OUT: &str = "benchmarks/results-analysis.json";
/// 未指定 `--group` 时的分组：全引擎执行吞吐。
const DEFAULT_GROUP: &str = "exec";

/// 命令行解析出的运行配置。
struct Config {
  runs: usize,
  json_out: String,
  /// 用户是否显式给出 `--json`（分组默认写各自的独立结果路径）。
  json_custom: bool,
  /// 用例 id 过滤器；为空表示全部。
  filters: Vec<String>,
  /// 测试分组 id（见 group::GROUPS）。
  group: String,
  /// 是否输出 ulua 侧分配计数表（需 `--features count-alloc` 编译）。
  alloc: bool,
  /// 只测这些引擎 key（为空 = 全部实测引擎）。
  engine_keys: Vec<String>,
  /// 写 JSON 前并入既有结果文件（多后端补测合并）。
  append: bool,
  /// ulua 侧 FastFlag 覆盖（`--fflag Name=true,Other=false`，同二进制 A/B
  /// 对照测量用；经 `set_flag_by_name` 应用，未知名报警忽略）。
  fflags: Vec<(String, bool)>,
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

/// 极简参数解析：`--runs[= ]N`、`--json[= ]PATH`、`--group[= ]NAME`、
/// `--engines[= ]k1,k2`、`--fflag[= ]Name=bool,Name2=bool`、`--append`；
/// 其余非选项参数为用例 id 过滤器。
fn parse_args() -> Config {
  let mut cfg = Config {
    runs: DEFAULT_RUNS,
    json_out: DEFAULT_JSON_OUT.to_owned(),
    json_custom: false,
    filters: Vec::new(),
    group: DEFAULT_GROUP.to_owned(),
    alloc: false,
    engine_keys: Vec::new(),
    append: false,
    fflags: Vec::new(),
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
      "--engines" => {
        let keys = value();
        if keys.is_empty() {
          eprintln!("警告: --engines 缺少取值，忽略（测量全部实测引擎）");
        } else {
          cfg.engine_keys = keys
            .split(',')
            .map(str::trim)
            .filter(|k| !k.is_empty())
            .map(str::to_owned)
            .collect();
        }
      }
      "--append" => cfg.append = true,
      "--fflag" => {
        let specs = value();
        for spec in specs.split(',').map(str::trim).filter(|s| !s.is_empty()) {
          // `Name=true|false`：缺 `=` 或值非布尔即报警忽略该段（其余段照常）
          let Some((name, raw)) = spec.split_once('=') else {
            eprintln!("警告: --fflag 段 {spec:?} 缺 `=bool`，忽略");
            continue;
          };
          match raw.trim() {
            "true" => cfg.fflags.push((name.trim().to_owned(), true)),
            "false" => cfg.fflags.push((name.trim().to_owned(), false)),
            other => eprintln!("警告: --fflag {name}={other} 非布尔值，忽略"),
          }
        }
      }
      "--alloc" => cfg.alloc = true,
      "--cases" | "--case" => {
        let cases = value();
        if !cases.is_empty() {
          cfg.filters.extend(
            cases
              .split(',')
              .map(str::trim)
              .filter(|s| !s.is_empty())
              .map(str::to_owned),
          );
        }
      }
      _ if is_option => eprintln!("警告: 忽略未知参数 {arg}"),
      _ => cfg.filters.push(arg),
    }
  }
  cfg.runs = cfg.runs.max(1);
  cfg
}

fn main() -> Result<(), IoError> {
  let cfg = parse_args();
  // FastFlag 覆盖先于任何测量（同二进制旗标开关对照测量：A/B 两轮除旗标外
  // 逐位同参，见 --fflag）。先用 rt enable_jit 的同参烧掉 set_luau_bool_flags
  // 的启动 Once（其后 no-op），否则覆盖会被首次 enable_jit 全量重置吞掉。
  f_value::set_luau_bool_flags(true);
  for (name, value) in &cfg.fflags {
    if !f_value::FValue::<bool>::set_flag_by_name(name, *value) {
      eprintln!("警告: 未知 FastFlag {name}，忽略");
    }
  }
  let Some(spec) = group::GROUPS.iter().find(|g| g.id == cfg.group) else {
    let known = group::GROUPS
      .iter()
      .map(|g| g.id)
      .collect::<Vec<_>>()
      .join(" / ");
    return Err(IoError::new(
      ErrorKind::InvalidInput,
      format!("未知分组 {:?}（可用: {known}）", cfg.group),
    ));
  };
  group::run_group(spec, &cfg)
}
