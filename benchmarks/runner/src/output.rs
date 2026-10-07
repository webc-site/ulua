//! 结果 JSON 的组装与落盘：主 `results.json`（exec 组，网站消费）与分组独立
//! JSON（compile/analysis）。`--append` 时主结果先读旧文件按「引擎 key」并行
//! 合并，供 bench.sh 多后端分次测量后汇成全集对比。

use std::{
  collections::BTreeMap,
  env::consts::{ARCH, OS},
  fs::{create_dir_all, read_to_string, write},
  io::Error as IoError,
  path::Path,
  thread::available_parallelism,
};

use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use sysinfo::System;

use crate::{case::BenchMeta, engine::EngineSpec, group::GroupSpec, measure::geomean_ratio};

/// JSON 里的平台标签：说明测量口径（进程内、无 fork/exec）。
const PLATFORM: &str = "Pure Rust In-Memory (No Process Overhead)";
/// 基准单位（毫秒），随用例一起导出给前端。
const UNIT_MS: &str = "ms";
/// 相对 ulua 解释执行的基线引擎 key（其比率恒为 1.0）。
const BASELINE_KEY: &str = "ulua";

#[derive(Serialize, Clone)]
struct BenchmarkItem {
  id: String,
  name: String,
  unit: &'static str,
}

/// 引擎清单条目（字段全是 `EngineSpec` 的 `&'static str` 投影）。
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

/// exec 组主结果（网站 `benchConvert.js` 消费，字段集是前端契约，不得裁剪）。
#[derive(Serialize)]
struct ResultsOutput {
  timestamp: String,
  platform: &'static str,
  environment: EnvironmentInfo,
  runs: usize,
  benchmarks: Vec<BenchmarkItem>,
  engines: Vec<EngineItem>,
  data: BTreeMap<String, BTreeMap<String, f64>>,
  /// 各引擎相对 ulua 解释执行的几何平均比率（基线自身恒为 1.0）。
  geomean_vs_ulua: BTreeMap<String, f64>,
}

/// 分组结果中的单个引擎条目（分组无参考基线表，字段收敛到 key/label）。
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

/// `--append` 合并所需的旧结果形状（只关心 data；缺字段按空处理）。
#[derive(Deserialize)]
struct ExistingResults {
  #[serde(default)]
  data: BTreeMap<String, BTreeMap<String, f64>>,
}

/// 用例 → JSON `benchmarks` 条目（id 与 name 同名，单位 ms）。
fn bench_items(cases: &[&BenchMeta]) -> Vec<BenchmarkItem> {
  cases
    .iter()
    .map(|c| BenchmarkItem {
      id: c.id.clone(),
      name: c.id.clone(),
      unit: UNIT_MS,
    })
    .collect()
}

fn engine_items(engines: &[EngineSpec]) -> Vec<EngineItem> {
  engines
    .iter()
    .map(|e| EngineItem {
      id: e.meta.id,
      key: e.meta.key,
      label: e.meta.label,
      lang: e.meta.lang,
      mode: e.meta.mode,
      color: e.meta.color,
      is_ulua: e.meta.is_ulua,
      is_reference: e.meta.is_reference,
    })
    .collect()
}

/// `--append`：读旧结果并把本轮 `data_map` 按（用例, 引擎 key）合并进旧表。
/// 本轮测到的引擎覆盖旧值，本轮未测的引擎保留旧值——多后端分次跑互不覆盖。
fn merged_data(
  path: &str,
  data_map: BTreeMap<String, BTreeMap<String, f64>>,
) -> BTreeMap<String, BTreeMap<String, f64>> {
  let Ok(raw) = read_to_string(path) else {
    return data_map;
  };
  let Ok(existing) = sonic_rs::from_str::<ExistingResults>(&raw) else {
    eprintln!("警告: --append 读取旧结果 {path} 失败（非法 JSON？），本轮结果将整体覆盖");
    return data_map;
  };
  let mut merged = existing.data;
  for (case, row) in data_map {
    let entry = merged.entry(case).or_default();
    for (key, ms) in row {
      entry.insert(key, ms);
    }
  }
  merged
}

/// 组装并写出 exec 组主结果 JSON。序列化与写盘的失败一律向上报错，不再静默丢弃。
///
/// `append=true` 时先并旧文件的 data（见 [`merged_data`]），几何平均随之在合并
/// 后的全集上重算；时间戳/环境/runs 以本轮为准（bench.sh 各后端共用同一 runs）。
pub fn export_results_json(
  path: &str,
  engines: &[EngineSpec],
  cases: &[&BenchMeta],
  data_map: BTreeMap<String, BTreeMap<String, f64>>,
  runs: usize,
  append: bool,
) -> Result<(), IoError> {
  let data = if append {
    merged_data(path, data_map)
  } else {
    data_map
  };

  // 实测引擎相对 ulua 解释执行的几何平均比率（基线自身恒为 1.0）；
  // `--engines` 过滤掉基线时（多后端补测场景）整表留空，比率由前端自算。
  let has_baseline = data.values().any(|row| row.contains_key(BASELINE_KEY));
  let mut geomean_map: BTreeMap<String, f64> = BTreeMap::new();
  if has_baseline {
    geomean_map.insert(BASELINE_KEY.to_owned(), 1.0);
    for engine in engines.iter().skip(1) {
      if let Some(ratio) = geomean_ratio(BASELINE_KEY, engine.meta.key, cases, &data) {
        geomean_map.insert(engine.meta.key.to_owned(), ratio);
      }
    }
  }

  let output = ResultsOutput {
    // 墙上时间戳用 jiff（§5 指定）；区段计时另有 `Instant`，两者用途不同。
    timestamp: Timestamp::now().to_string(),
    platform: PLATFORM,
    environment: detect_environment(),
    runs,
    benchmarks: bench_items(cases),
    engines: engine_items(engines),
    data,
    geomean_vs_ulua: geomean_map,
  };

  let json = sonic_rs::to_string_pretty(&output).map_err(IoError::other)?;
  fs_write_parented(path, json)?;
  println!("✓ 性能评测结果已写入 {path}");
  Ok(())
}

/// 组装并写出分组结果 JSON。
pub fn export_group_json(
  path: &str,
  group: &GroupSpec,
  cases: &[&BenchMeta],
  data_map: BTreeMap<String, BTreeMap<String, f64>>,
  runs: usize,
) -> Result<(), IoError> {
  let engines = group.engines();
  // 各引擎相对分组首引擎（ulua 侧基线）的几何平均比率（基线自身恒为 1.0）。
  let baseline = engines.first().expect("分组至少有一个引擎");
  let mut geomean_map: BTreeMap<String, f64> = BTreeMap::new();
  geomean_map.insert(baseline.meta.key.to_owned(), 1.0);
  for engine in engines.iter().skip(1) {
    if let Some(ratio) = geomean_ratio(baseline.meta.key, engine.meta.key, cases, &data_map) {
      geomean_map.insert(engine.meta.key.to_owned(), ratio);
    }
  }

  let output = GroupOutput {
    timestamp: Timestamp::now().to_string(),
    group: group.id,
    title: group.title,
    environment: detect_environment(),
    runs,
    benchmarks: bench_items(cases),
    engines: engines
      .iter()
      .map(|e| GroupEngineItem {
        key: e.meta.key,
        label: e.meta.label,
      })
      .collect(),
    data: data_map,
    geomean_vs_baseline: geomean_map,
  };

  let json = sonic_rs::to_string_pretty(&output).map_err(IoError::other)?;
  fs_write_parented(path, json)?;
  println!("✓ {} 组评测结果已写入 {path}", group.id);
  Ok(())
}

/// 写盘（父目录不存在则先建），序列化失败与 I/O 失败统一上抛。
fn fs_write_parented(path: &str, contents: String) -> Result<(), IoError> {
  if let Some(parent) = Path::new(path).parent() {
    create_dir_all(parent)?;
  }
  write(path, contents)
}

/// 动态检测当前主机的系统硬件与 OS 环境（基于 sysinfo）。
fn detect_environment() -> EnvironmentInfo {
  // `new_all()` 内部已完成全量刷新，无需再 refresh_all（纯冗余二扫）
  let sys = System::new_all();

  let os_name = System::name().unwrap_or_else(|| OS.to_string());
  let os_ver = System::os_version().unwrap_or_default();
  let os = if os_ver.is_empty() {
    os_name
  } else {
    format!("{os_name} {os_ver}")
  };

  let arch = {
    let a = System::cpu_arch();
    if a.is_empty() { ARCH.to_string() } else { a }
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
