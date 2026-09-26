//! Faithful port of `void profilerDump(const char* path)` (CLI/src/Profiler.cpp:115).

use alloc::borrow::Cow;
use core::{ffi::c_int, sync::atomic::Ordering};
use std::io::Write;

use ulua_common::functions::c_str::cstr_cow;
use ulua_vm::functions::lua_c_statename::lua_c_statename;

use crate::functions::{create_dump_writer::create_dump_writer, profiler_trigger::G_PROFILER};
pub(crate) fn profiler_dump(path: &str) {
  // profiler_stop 已 join 采样线程，此处独占；逐字段共享借用，不构造整个 `Profiler`
  // 的 `&mut`（见 profiler_trigger 的同类注释）。
  let profiler = G_PROFILER.get();

  let Some(mut f) = create_dump_writer(path, "profile") else {
    return;
  };

  let mut total: u64 = 0;
  // Safety: profiler_stop 已 join 采样线程，`data` 仅由 VM 线程（即本线程）在
  // profiler_trigger 中写入，读与写同线程顺序化，无竞争。
  let data = unsafe { &(*profiler).data };
  for (stack, &ticks) in data.iter() {
    // C++: fprintf(f, "%lld %s\n", ticks, stack)
    let _ = writeln!(f, "{} {}", ticks, stack);
    total += ticks;
  }
  // cpp `fclose` 隐式 flush；失败同样静默
  let _ = f.flush();
  drop(f);

  let stacks = data.len();
  // Safety: 采样线程已 join；`samples` 为原子字段，join 后无并发写者。
  let samples = unsafe { (*profiler).samples.load(Ordering::Relaxed) };
  println!(
    "Profiler dump written to {} (total runtime {:.3} seconds, {} samples, {} stacks)",
    path,
    total as f64 / 1e6,
    samples,
    stacks
  );

  // Safety: `gc` 仅由 VM 线程（即本线程）写入，且 VM 已停止，无竞争。
  let totalgc: u64 = unsafe { (*profiler).gc.iter().sum() };

  if totalgc != 0 {
    print!(
      "GC: {:.3} seconds ({:.2}%)",
      totalgc as f64 / 1e6,
      totalgc as f64 / total as f64 * 100.0
    );

    // Safety: 同上，`gc` 为仅本线程写入的普通字段，读无竞争。
    for (i, &p) in unsafe { (*profiler).gc.iter() }.enumerate() {
      if p != 0 {
        // 「判空 + CStr::from_ptr」样板收敛到 cstr_cow 单点门面：null 译空串，
        // 与原 if/else 的 Cow::Borrowed("") 分支等价。
        // Safety: lua_c_statename 返回空指针或指向静态 NUL 结尾字符串。
        let name: Cow<'_, str> = unsafe { cstr_cow(lua_c_statename(i as c_int)) };
        print!(", {} {:.2}%", name, p as f64 / totalgc as f64 * 100.0);
      }
    }

    println!();
  }
}
