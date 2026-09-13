use alloc::{collections::BTreeMap, string::String};
use core::{
  ptr::null_mut,
  sync::atomic::{AtomicBool, AtomicU64},
};
use std::thread::JoinHandle;

use ulua_vm::records::lua_callbacks::LuaCallbacks;

pub struct Profiler {
  pub(crate) callbacks: *mut LuaCallbacks,
  pub(crate) frequency: i32,
  pub(crate) thread: Option<JoinHandle<()>>,
  pub(crate) exit: AtomicBool,
  pub(crate) ticks: AtomicU64,
  pub(crate) samples: AtomicU64,
  pub(crate) current_ticks: u64,
  pub(crate) stack_scratch: String,
  pub(crate) data: Option<BTreeMap<String, u64>>,
  pub(crate) gc: [u64; 16],
}

impl Default for Profiler {
  fn default() -> Self {
    Self {
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
    }
  }
}
