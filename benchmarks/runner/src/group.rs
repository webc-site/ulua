//! 分组定义与统一主流程。
//!
//! exec / compile / analysis 三组共用一条流程：扫描用例 → 过滤 → 组装实测引擎
//! → 预热+交替最小值测量+打表 → 写各自 JSON。组的差异全部收在 [`GroupSpec`]
//! 的字段里（引擎集、用例目录、列宽、JSON 形状、是否归一 FFlag），流程本体
//! 只有这一份。

use std::io::{Error as IoError, ErrorKind};

use crate::{
  Config,
  case::{filter_cases, locate_dir, scan_group_cases},
  compile,
  engine::{self, EngineImpl},
  output::{export_group_json, export_results_json},
  report::{W_EXEC, W_GROUP, Widths, measure_and_tabulate},
};

/// 分组定义。`engines_builder` 返回的引擎表需保持注册顺序（表格列序与 JSON
/// 引擎清单都依赖它；首引擎是分组的比率基线）。
pub struct GroupSpec {
  pub id: &'static str,
  pub title: &'static str,
  pub dir_candidates: &'static [&'static str],
  pub ext: &'static str,
  pub engines_builder: fn() -> Vec<engine::EngineSpec>,
  /// `--json` 未显式给出时的结果输出路径。
  pub json_out: &'static str,
  pub widths: Widths,
  /// 主流程开始前是否把默认开启的 Luau FFlag 归一为 true（compile/analysis
  /// 的解析口径需要；exec 组保持 `Lua::new()` 的开箱默认）。
  pub normalize_flags: bool,
}

impl GroupSpec {
  /// 组装本组引擎表（含当前后端未编译的占位条目）。
  pub fn engines(&self) -> Vec<engine::EngineSpec> {
    (self.engines_builder)()
  }
}

/// exec 组：16 个纯计算 Lua 用例 × 全引擎矩阵（ulua 双模式 + 当前 C 后端）。
pub static EXEC_GROUP: GroupSpec = GroupSpec {
  id: "exec",
  title: "纯计算用例执行吞吐",
  dir_candidates: &["benchmarks/cases", "cases"],
  ext: "lua",
  engines_builder: engine::exec_engines,
  json_out: crate::DEFAULT_JSON_OUT,
  widths: W_EXEC,
  normalize_flags: false,
};

/// 编译吞吐组（parse / parse+compile 到字节码）。
pub static COMPILE_GROUP: GroupSpec = GroupSpec {
  id: "compile",
  title: "编译吞吐 (parse / parse+compile 到字节码)",
  dir_candidates: &["benchmarks/compile_cases", "compile_cases"],
  ext: "luau",
  engines_builder: compile_engines,
  json_out: crate::COMPILE_JSON_OUT,
  widths: W_GROUP,
  normalize_flags: true,
};

/// 类型检查组（ulua-analysis 前端，strict 模式）。
pub static ANALYSIS_GROUP: GroupSpec = GroupSpec {
  id: "analysis",
  title: "类型检查吞吐 (ulua-analysis 前端, strict 模式)",
  dir_candidates: &["benchmarks/analysis_cases", "analysis_cases"],
  ext: "luau",
  engines_builder: analysis_engines,
  json_out: crate::ANALYSIS_JSON_OUT,
  widths: W_GROUP,
  normalize_flags: true,
};

/// 全部分组（main 按 id 查找）。
pub static GROUPS: &[&GroupSpec] = &[&EXEC_GROUP, &COMPILE_GROUP, &ANALYSIS_GROUP];

fn compile_engines() -> Vec<engine::EngineSpec> {
  let mut engines = vec![
    engine::group_spec("ulua-parse", "ulua parse", true, Some(EngineImpl::UluaParse)),
    engine::group_spec(
      "ulua-compile",
      "ulua parse+compile",
      true,
      Some(EngineImpl::UluaCompile),
    ),
  ];
  #[cfg(feature = "engine-luau")]
  engines.push(engine::group_spec(
    "mlua-compile",
    "mlua/luau compile",
    false,
    Some(EngineImpl::LuauCppCompile),
  ));
  #[cfg(not(feature = "engine-luau"))]
  engines.push(engine::group_spec("mlua-compile", "mlua/luau compile", false, None));
  engines
}

fn analysis_engines() -> Vec<engine::EngineSpec> {
  vec![
    engine::group_spec(
      "ulua-analysis-globals",
      "globals 基线",
      true,
      Some(EngineImpl::AnalysisGlobals),
    ),
    engine::group_spec(
      "ulua-analysis-check",
      "globals+检查",
      true,
      Some(EngineImpl::AnalysisCheck),
    ),
  ]
}

/// 按用户 `--engines` 过滤实测引擎，返回「注册条目 + 实现」按值配对的
/// [`engine::LiveEngine`] 列表；空过滤器 = 全部实测引擎。顺序保持注册序。
/// 指定了当前二进制不可测的 key 时报错（带可用清单与后端提示），不静默跳过。
fn select_engines(spec: &GroupSpec, cfg: &Config) -> Result<Vec<engine::LiveEngine>, IoError> {
  let all = spec.engines();
  let live: Vec<engine::LiveEngine> = all
    .into_iter()
    .filter(|e| cfg.engine_keys.is_empty() || cfg.engine_keys.iter().any(|key| key == e.meta.key))
    .filter_map(|e| e.engine.map(|imp| engine::LiveEngine { spec: e, imp }))
    .collect();
  for want in &cfg.engine_keys {
    if !live.iter().any(|e| e.spec.meta.key == want) {
      let available: Vec<&str> = spec
        .engines()
        .iter()
        .filter(|e| e.engine.is_some())
        .map(|e| e.meta.key)
        .collect();
      return Err(IoError::new(
        ErrorKind::InvalidInput,
        format!(
          "引擎 {want:?} 不在当前二进制的可测清单（当前后端 {}，可用: {}）",
          engine::active_backend_name(),
          available.join(", ")
        ),
      ));
    }
  }
  if live.is_empty() {
    return Err(IoError::new(
      ErrorKind::InvalidInput,
      format!("过滤后没有可实测引擎（--engines {:?}）", cfg.engine_keys),
    ));
  }
  Ok(live)
}

/// 通用分组主流程。测量口径与输出格式与历史实现逐字符一致。
pub fn run_group(spec: &GroupSpec, cfg: &Config) -> Result<(), IoError> {
  if spec.normalize_flags {
    compile::apply_luau_flags_default();
  }
  let cases_dir = locate_dir(spec.dir_candidates);
  let all_cases = scan_group_cases(&cases_dir, spec.ext)?;
  let active_cases = filter_cases(&all_cases, &cfg.filters);
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
  let live = select_engines(spec, cfg)?;

  println!();
  println!(
    "ulua {} 基准评测 (分组 {}, 每项交替采样 {} 轮取最小耗时)",
    spec.title, spec.id, cfg.runs
  );
  println!();

  let tab = measure_and_tabulate(&active_cases, &live, cfg.runs, spec.widths, cfg.alloc)?;
  if !tab.values_ok {
    eprintln!(
      "警告: 分组 {} 存在跨引擎返回值分歧（详情见上方 [用例] 行），对比数据请谨慎采信",
      spec.id
    );
  }

  let json_path = if cfg.json_custom {
    cfg.json_out.clone()
  } else {
    spec.json_out.to_owned()
  };
  if spec.id == "exec" {
    export_results_json(
      &json_path,
      &spec.engines(),
      &active_cases,
      tab.data_map,
      cfg.runs,
      cfg.append,
    )
  } else {
    export_group_json(&json_path, spec, &active_cases, tab.data_map, cfg.runs)
  }
}
