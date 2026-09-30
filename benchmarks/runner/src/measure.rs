//! 测量内核：单次采样、轮次交替采样、几何平均与比率。

use std::{collections::BTreeMap, hint::black_box, time::Instant};

use crate::{
  case::BenchMeta,
  engine::{EngineImpl, LiveEngine},
};

/// 秒 → 毫秒。
pub(crate) const MS_PER_SEC: f64 = 1e3;
/// 主表耗时列保留的精度（0.1 ms）。
pub(crate) const MS_RESOLUTION: f64 = 10.0;
/// 几何平均比率保留的精度（两位小数）。
pub(crate) const RATIO_RESOLUTION: f64 = 100.0;

/// 单引擎在一轮「预热 + N 轮采样」后的完整结果。
pub(crate) struct EngineRun {
  /// 返回值指纹（预热轮捕获），供跨引擎一致性校验；引擎无返回值时为 `None`。
  pub value: Option<String>,
  /// `runs` 轮中的最小耗时（毫秒）；预热或全部采样失败时为 `None`。
  pub best: Option<f64>,
  /// 首个失败信息（预热或采样）。
  pub err: Option<String>,
}

/// 单次采样：跑一次被测引擎，返回墙上耗时毫秒与执行结果。
///
/// 计时只用 `Instant`（单调时钟、纳秒级分辨率）；`coarsetime` 的 ~1µs 粒度面向
/// 墙上时间戳，不适合区段测量（墙上时间戳由 `jiff` 负责）。`black_box` 同时喂入
/// 源码与结果，防止整段执行被判为死代码或被常量折叠掉。
fn measure_once(imp: EngineImpl, src: &str) -> (Result<Option<String>, String>, f64) {
  let start = Instant::now();
  let outcome = imp.run(black_box(src));
  let ms = start.elapsed().as_secs_f64() * MS_PER_SEC;
  black_box(&outcome);
  (outcome, ms)
}

/// 「轮次 × 引擎」交替采样，每引擎取 `runs` 轮中的**最小**耗时。
///
/// 口径理由：本机常被并行任务挤到 loadavg 50+。块状轮转（先跑完 A 引擎所有轮次
/// 再跑 B）会把两次块之间的负载差算成引擎差；同一轮内交替采样让各引擎面对几乎
/// 相同的机器状态。聚合用最小值而非中位数：负载尖峰只会抬高样本，最小值是「无
/// 干扰时引擎工作量」的一致估计，中位数在持续性干扰下仍被整体抬高。
///
/// 只接受 [`LiveEngine`]（实现已与注册条目按值配对，无运行期判空）。返回与
/// `live` 等长的结果向量：预热或测量失败的槽携带错误信息。
pub(crate) fn measure_rounds(live: &[LiveEngine], src: &str, runs: usize) -> Vec<EngineRun> {
  let mut runs_out: Vec<EngineRun> = live
    .iter()
    .map(|_| EngineRun {
      value: None,
      best: None,
      err: None,
    })
    .collect();

  // 预热 (Warmup)：分配器与 JIT 的热身不计入样本；同时捕获返回值指纹。
  for (slot, engine) in live.iter().enumerate() {
    let (outcome, _) = measure_once(engine.imp, src);
    match outcome {
      Ok(value) => runs_out[slot].value = value,
      Err(e) => runs_out[slot].err = Some(e),
    }
  }

  for _round in 0..runs {
    for (slot, engine) in live.iter().enumerate() {
      if runs_out[slot].err.is_some() {
        continue;
      }
      let (outcome, ms) = measure_once(engine.imp, src);
      match outcome {
        Ok(_) => runs_out[slot].best = Some(runs_out[slot].best.map_or(ms, |b| b.min(ms))),
        Err(e) => runs_out[slot].err = Some(format!("{}: {e}", engine.spec.meta.key)),
      }
    }
  }

  runs_out
}

/// 几何平均值（对数域求和，免连乘溢出）；忽略非正样本，全空返回 0.0。
pub(crate) fn geomean(times: impl Iterator<Item = f64>) -> f64 {
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

/// `engine_key` 相对基线 `baseline_key` 的几何平均比率（缺测/非正样本剔除），
/// 四舍五入到两位小数；无有效样本返回 `None`。
pub(crate) fn geomean_ratio(
  baseline_key: &str,
  engine_key: &str,
  cases: &[&BenchMeta],
  data: &BTreeMap<String, BTreeMap<String, f64>>,
) -> Option<f64> {
  let ratios = cases
    .iter()
    .filter_map(|c| data.get(&c.id))
    .filter_map(|row| Some((*row.get(baseline_key)?, *row.get(engine_key)?)))
    .filter(|(base, mine)| *base > 0.0 && *mine > 0.0)
    .map(|(base, mine)| mine / base);
  let mean = geomean(ratios);
  (mean > 0.0).then(|| (mean * RATIO_RESOLUTION).round() / RATIO_RESOLUTION)
}
