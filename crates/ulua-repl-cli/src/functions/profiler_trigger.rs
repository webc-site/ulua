use alloc::{collections::BTreeMap, string::String};
use core::{
  ffi::CStr,
  mem::zeroed,
  ptr::{addr_of_mut, null_mut},
  sync::atomic::{AtomicBool, AtomicU64, Ordering},
};

use ulua_vm::{
  functions::{lua_callbacks::lua_callbacks, lua_getinfo::lua_getinfo},
  records::lua_debug::LuaDebug,
  type_aliases::lua_state::lua_State,
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
  data: BTreeMap::new(),
  gc: [0; 16],
};

// Faithful port of Profiler.cpp's `static void profilerTrigger(lua_State* l, int gc)`.
//
// 采样线程（profiler_loop）与本函数分处两线程：前者只碰 ticks/samples/callbacks
// 指向的 interrupt 槽，后者只碰 current_ticks/stack_scratch/data/gc。因此这里用
// 裸指针逐字段访问，绝不构造覆盖整个 `Profiler`（含原子字段）的 `&mut`——那会与
// 另一线程的原子写别名，属确定 UB。
pub unsafe fn profiler_trigger(l: *mut lua_State, gc: i32) {
  unsafe {
    let profiler = addr_of_mut!(G_PROFILER);

    let current_ticks = (*profiler).ticks.load(Ordering::Relaxed);
    let elapsed_ticks = current_ticks - (*profiler).current_ticks;

    if elapsed_ticks != 0 {
      let stack = &mut (*profiler).stack_scratch;
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
        let data = &mut (*profiler).data;
        // cpp `data[stack] += elapsed`；仅新栈首次出现时才复制 key（每秒最多
        // frequency 次采样，热路径上克隆整条栈字符串开销可观）
        match data.get_mut(stack.as_str()) {
          Some(ticks) => *ticks += elapsed_ticks,
          None => {
            data.insert(stack.clone(), elapsed_ticks);
          }
        }
      }

      if gc > 0 {
        // cpp `gProfiler.gc[gc]` 无检查；Rust 侧索引越界即 panic，用 get_mut 挡住
        // 超出 GC 状态数的异常入参
        if let Some(slot) = (*profiler).gc.get_mut(gc as usize) {
          *slot += elapsed_ticks;
        }
      }
    }

    (*profiler).current_ticks = current_ticks;

    let callbacks = (*profiler).callbacks;
    if !callbacks.is_null() {
      (*callbacks).interrupt = None;
    } else {
      // The sampling-thread shim does not publish a callbacks pointer, so
      // clear the live state's interrupt directly — equivalent to C++'s
      // `gProfiler.callbacks->interrupt = nullptr`.
      (*lua_callbacks(l)).interrupt = None;
    }
  }
}
