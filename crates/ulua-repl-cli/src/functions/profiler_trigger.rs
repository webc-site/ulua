use alloc::{collections::BTreeMap, string::String};
use core::{
  ffi::CStr,
  mem::zeroed,
  ptr::null_mut,
  sync::atomic::{AtomicBool, AtomicU64, Ordering},
};

use ulua_vm::{
  functions::{lua_callbacks::lua_callbacks, lua_getinfo::lua_getinfo},
  records::{lua_debug::LuaDebug, lua_state::lua_State},
};

// Trigger-side view of Profiler.cpp's file-static `gProfiler`. The sampling
// thread publishes `ticks` (atomic) and the VM-thread trigger consumes them,
// keeping `current_ticks`, the `stack_scratch` reuse Buffer, the accumulated
// per-stack `data` and the per-GC-state timing array — exactly the fields
// `profilerTrigger` reads and writes. (Profiler.cpp uses a
// DenseHashMap<string,uint64> for `data`; a BTreeMap captures the same
// stack→ticks accumulation.)
use crate::records::profiler::Profiler;

pub(crate) static mut G_PROFILER: Profiler = Profiler {
  callbacks: null_mut(),
  frequency: 1000,
  thread: None,
  exit: AtomicBool::new(false),
  ticks: AtomicU64::new(0),
  samples: AtomicU64::new(0),
  current_ticks: 0,
  stack_scratch: String::new(),
  data: None,
  gc: [0; 16],
};

// Faithful port of Profiler.cpp's `static void profilerTrigger(lua_State* l, int gc)`.
pub unsafe fn profiler_trigger(l: *mut lua_State, gc: i32) {
  unsafe {
    let profiler = core::ptr::addr_of_mut!(G_PROFILER).as_mut().unwrap();

    let current_ticks = profiler.ticks.load(Ordering::Relaxed);
    let elapsed_ticks = current_ticks - profiler.current_ticks;

    if elapsed_ticks != 0 {
      let stack = &mut profiler.stack_scratch;
      stack.clear();

      if gc > 0 {
        stack.push_str("GC,GC,");
      }

      let mut ar: LuaDebug = zeroed();
      let mut level = 0;
      while lua_getinfo(l, level, c"sn".as_ptr(), &mut ar as *mut LuaDebug) != 0 {
        if !stack.is_empty() {
          stack.push(';');
        }

        if !ar.short_src.is_null() {
          stack.push_str(&CStr::from_ptr(ar.short_src).to_string_lossy());
        }
        stack.push(',');
        if !ar.name.is_null() {
          stack.push_str(&CStr::from_ptr(ar.name).to_string_lossy());
        }
        stack.push(',');
        if ar.linedefined > 0 {
          use core::fmt::Write;
          let _ = write!(stack, "{}", ar.linedefined);
        }

        level += 1;
      }

      if !stack.is_empty() {
        let key = stack.clone();
        let data = profiler.data.get_or_insert_with(BTreeMap::new);
        *data.entry(key).or_insert(0) += elapsed_ticks;
      }

      if gc > 0 {
        profiler.gc[gc as usize] += elapsed_ticks;
      }
    }

    profiler.current_ticks = current_ticks;

    if !profiler.callbacks.is_null() {
      (*profiler.callbacks).interrupt = None;
    } else {
      // The sampling-thread shim does not publish a callbacks pointer, so
      // clear the live state's interrupt directly — equivalent to C++'s
      // `gProfiler.callbacks->interrupt = nullptr`.
      (*lua_callbacks(l)).interrupt = None;
    }
  }
}
