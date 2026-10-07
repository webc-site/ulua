//! Faithful port of `void profilerDump(const char* path)` (CLI/src/Profiler.cpp:115).

use core::sync::atomic::Ordering;
use std::io::Write;

use ulua_vm::functions::lua_c_statename::lua_c_statename;

use crate::functions::{
  create_dump_writer::create_dump_writer,
  profiler_trigger::{G_PROFILER_MAIN, G_PROFILER_SHARED},
};

pub(crate) fn profiler_dump(path: &str) {
  // profiler_stop 已 join 采样线程、VM 已停止执行：data/gc 为同线程 thread_local
  // 字段的常规读取，samples 为原子读取——全函数零 unsafe。写者创建失败早退，
  // 与 cpp `if (!f) return;` 一致。
  G_PROFILER_MAIN.with(|cell| {
    let profiler = cell.borrow();

    let Some(mut f) = create_dump_writer(path, "profile") else {
      return;
    };

    let mut total: u64 = 0;
    for (stack, &ticks) in &profiler.data {
      // C++: fprintf(f, "%lld %s\n", ticks, stack)
      let _ = writeln!(f, "{} {}", ticks, stack);
      total += ticks;
    }
    // cpp `fclose` 隐式 flush；失败同样静默
    let _ = f.flush();
    drop(f);

    let stacks = profiler.data.len();
    let samples = G_PROFILER_SHARED.samples.load(Ordering::Relaxed);
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
  });
}
