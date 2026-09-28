//! Faithful port of `void profilerDump(const char* path)` (CLI/src/Profiler.cpp:115).

use core::sync::atomic::Ordering;
use std::io::Write;

use ulua_vm::functions::lua_c_statename::lua_c_statename;

use crate::functions::{create_dump_writer::create_dump_writer, profiler_trigger::G_PROFILER};

pub(crate) fn profiler_dump(path: &str) {
  // Safety: profiler_stop 已 join 采样线程、VM 已停止执行，落在 dump 窗口契约：
  // data/gc 与写者同线程顺序化无竞争，samples 原子字段无并发写者。
  let profiler = unsafe { G_PROFILER.dump() };

  let Some(mut f) = create_dump_writer(path, "profile") else {
    return;
  };

  let mut total: u64 = 0;
  for (stack, &ticks) in profiler.data.iter() {
    // C++: fprintf(f, "%lld %s\n", ticks, stack)
    let _ = writeln!(f, "{} {}", ticks, stack);
    total += ticks;
  }
  // cpp `fclose` 隐式 flush；失败同样静默
  let _ = f.flush();
  drop(f);

  let stacks = profiler.data.len();
  let samples = profiler.samples.load(Ordering::Relaxed);
  println!(
    "Profiler dump written to {} (total runtime {:.3} seconds, {} samples, {} stacks)",
    path,
    total as f64 / 1e6,
    samples,
    stacks
  );

  let totalgc: u64 = profiler.gc.iter().sum();

  if totalgc != 0 {
    print!(
      "GC: {:.3} seconds ({:.2}%)",
      totalgc as f64 / 1e6,
      totalgc as f64 / total as f64 * 100.0
    );

    for (i, &p) in profiler.gc.iter().enumerate() {
      if p != 0 {
        let name = lua_c_statename(i as u8);
        print!(", {} {:.2}%", name, p as f64 / totalgc as f64 * 100.0);
      }
    }

    println!();
  }
}
