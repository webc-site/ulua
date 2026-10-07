//! 结果打表：表头 → 逐用例×引擎耗时行 → 几何平均行 → 返回值一致性告警 →
//! 可选分配计数表。输出口径与历史输出逐字符一致（列宽由 [`Widths`] 决定）。

#[cfg(feature = "count-alloc")]
use std::hint::black_box;
use std::{collections::BTreeMap, io};

#[cfg(feature = "count-alloc")]
use crate::alloc_count;
use crate::{
  case::BenchMeta,
  engine::LiveEngine,
  measure::{MS_RESOLUTION, geomean, measure_rounds},
};

/// 表格列宽口径（exec 历史口径与分组口径不同，输出必须逐字符不变）。
#[derive(Clone, Copy)]
pub(crate) struct Widths {
  /// 用例名 / 「几何平均耗时」列宽（左对齐）。
  pub case: usize,
  /// 引擎列宽（右对齐，含表头与 ERR 格）。
  pub eng: usize,
  /// 耗时数字列宽（右对齐，一位小数）。
  pub ms: usize,
}

/// exec 组历史列宽。
pub(crate) const W_EXEC: Widths = Widths {
  case: 12,
  eng: 16,
  ms: 13,
};
/// compile/analysis 分组列宽。
pub(crate) const W_GROUP: Widths = Widths {
  case: 16,
  eng: 18,
  ms: 15,
};

/// 打表 + 测量的完整产物（写 JSON 所需的全部中间量）。
pub(crate) struct Tabulation {
  /// 用例 id → (引擎 key → 最小耗时 ms)。
  pub data_map: BTreeMap<String, BTreeMap<String, f64>>,
  /// 跨引擎返回值指纹是否全部一致（false 时已向 stderr 输出分歧详情）。
  pub values_ok: bool,
}

/// 测量并打表（exec 与分组共用）：测量口径统一收口在 [`measure_rounds`]。
pub(crate) fn measure_and_tabulate(
  cases: &[&BenchMeta],
  live: &[LiveEngine],
  runs: usize,
  w: Widths,
  alloc: bool,
) -> io::Result<Tabulation> {
  print!("{:<case$}", "用例", case = w.case);
  for engine in live {
    print!(" {:>eng$}", engine.spec.meta.label, eng = w.eng);
  }
  println!();

  let mut data_map: BTreeMap<String, BTreeMap<String, f64>> = BTreeMap::new();
  let mut samples: Vec<Vec<f64>> = vec![Vec::new(); live.len()];
  let mut values_ok = true;

  for case in cases {
    print!("{:<case$}", case.id, case = w.case);
    let mut case_data: BTreeMap<String, f64> = BTreeMap::new();
    // 本用例首个引擎的返回值指纹（跨引擎一致性比对锚点）。
    let mut anchor: Option<&str> = None;
    let engine_runs = measure_rounds(live, &case.src, runs);
    for (slot, engine) in live.iter().enumerate() {
      let run = &engine_runs[slot];
      match run.best {
        Some(ms) => {
          let rounded = (ms * MS_RESOLUTION).round() / MS_RESOLUTION;
          print!(" {:>ms$.1} ms", rounded, ms = w.ms);
          case_data.insert(engine.spec.meta.key.to_owned(), rounded);
          if ms > 0.0 {
            samples[slot].push(ms);
          }
        }
        None => {
          print!(" {:>eng$}", "ERR", eng = w.eng);
          eprintln!(
            "\n[{}] 引擎 {} 运行失败: {}",
            case.id,
            engine.spec.meta.label,
            run.err.as_deref().unwrap_or("无有效样本")
          );
        }
      }
      // 跨引擎返回值一致性：同一用例在所有产出指纹的引擎下必须同值。
      if let Some(value) = &run.value {
        match anchor {
          None => anchor = Some(value),
          Some(first) if first != value => {
            values_ok = false;
            eprintln!(
              "\n[{}] 返回值分歧: {} = {:?}（此前为 {:?}）",
              case.id, engine.spec.meta.key, value, first
            );
          }
          Some(_) => {}
        }
      }
    }
    println!();
    data_map.insert(case.id.clone(), case_data);
  }

  // 几何平均耗时
  println!();
  print!("{:<case$}", "几何平均耗时", case = w.case);
  for time in &samples {
    print!(" {:>ms$.1} ms", geomean(time.iter().copied()), ms = w.ms);
  }
  println!();
  println!();

  // `--alloc`：追加 ulua 侧分配计数表（mlua 引擎不经 Rust 全局分配器，不列）。
  if alloc {
    let ulua_engines: Vec<LiveEngine> = live
      .iter()
      .filter(|e| e.spec.meta.is_ulua)
      .copied()
      .collect();
    report_allocations(cases, &ulua_engines)?;
  }
  Ok(Tabulation {
    data_map,
    values_ok,
  })
}

/// 输出 ulua 侧分配计数表（`--alloc`；feature 关闭版：仅提示口径，不做任何测量）。
#[cfg(not(feature = "count-alloc"))]
fn report_allocations(cases: &[&BenchMeta], live: &[LiveEngine]) -> io::Result<()> {
  let _ = (cases, live);
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
/// 口径为 Rust 侧 `GlobalAlloc` 请求（mlua vendored C Lua 直连 C malloc，
/// 不经此路径，故只报 ulua 侧计数），不进入耗时表与既有 JSON。
#[cfg(feature = "count-alloc")]
fn report_allocations(cases: &[&BenchMeta], live: &[LiveEngine]) -> io::Result<()> {
  println!();
  println!("ulua 侧分配计数 (口径: Rust GlobalAlloc 请求; C 侧 malloc 不计入)");
  if live.is_empty() {
    println!("(当前分组没有可计数的 ulua 侧引擎)");
    return Ok(());
  }

  print!("{:<16}", "用例");
  for engine in live {
    print!(
      " {:>14} {:>14}",
      format!("{} 次数", engine.spec.meta.label),
      format!("{} 字节", engine.spec.meta.label)
    );
  }
  println!();

  for case in cases {
    print!("{:<16}", case.id);
    for engine in live {
      // 预热一次排除惰性初始化，再取单次运行的快照差。
      if let Err(err) = engine.imp.run(&case.src) {
        print!(" {:>30}", format!("{}: ERR", engine.spec.meta.label));
        eprintln!("\n[{}] 分配计数预热失败: {}", case.id, err);
        continue;
      }
      let (calls0, bytes0) = alloc_count::snapshot();
      let outcome = engine.imp.run(black_box(&case.src));
      let (calls1, bytes1) = alloc_count::snapshot();
      match outcome {
        Ok(_) => print!(" {:>14} {:>14}", calls1 - calls0, bytes1 - bytes0),
        Err(err) => {
          print!(" {:>30}", format!("{}: ERR", engine.spec.meta.label));
          eprintln!("\n[{}] 分配计数运行失败: {}", case.id, err);
        }
      }
    }
    println!();
  }
  Ok(())
}
