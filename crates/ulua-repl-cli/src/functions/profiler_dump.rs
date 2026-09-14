//! Faithful port of `void profilerDump(const char* path)` (CLI/src/Profiler.cpp:115).

use alloc::{borrow::Cow, collections::BTreeMap};
use core::{
  ffi::{CStr, c_int},
  sync::atomic::Ordering,
};
use std::{fs::File, io::Write};

use ulua_vm::functions::lua_c_statename::lua_c_statename;

use crate::functions::profiler_trigger::G_PROFILER;
pub fn profiler_dump(path: &str) {
  unsafe {
    let profiler = core::ptr::addr_of_mut!(G_PROFILER).as_mut().unwrap();

    let mut f = match File::create(path) {
      Ok(f) => f,
      Err(_) => {
        eprintln!("Error opening profile {}", path);
        return;
      }
    };

    let mut total: u64 = 0;
    let data = profiler.data.get_or_insert_with(BTreeMap::new);
    for (stack, &ticks) in data.iter() {
      // C++: fprintf(f, "%lld %s\n", ticks, stack)
      let _ = writeln!(f, "{} {}", ticks, stack);
      total += ticks;
    }
    drop(f);

    let stacks = data.len();
    let samples = profiler.samples.load(Ordering::Relaxed);
    println!(
      "Profiler dump written to {} (total runtime {:.3} seconds, {} samples, {} stacks)",
      path,
      total as f64 / 1e6,
      samples,
      stacks
    );

    let mut totalgc: u64 = 0;
    for &p in profiler.gc.iter() {
      totalgc += p;
    }

    if totalgc != 0 {
      print!(
        "GC: {:.3} seconds ({:.2}%)",
        totalgc as f64 / 1e6,
        totalgc as f64 / total as f64 * 100.0
      );

      for (i, &p) in profiler.gc.iter().enumerate() {
        if p != 0 {
          let name_ptr = lua_c_statename(i as c_int);
          let name = if name_ptr.is_null() {
            Cow::Borrowed("")
          } else {
            CStr::from_ptr(name_ptr).to_string_lossy()
          };
          print!(", {} {:.2}%", name, p as f64 / totalgc as f64 * 100.0);
        }
      }

      println!();
    }
  }
}
