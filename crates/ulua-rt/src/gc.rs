//! Garbage-collector control. Mirrors `mlua::Lua`'s `gc_*` surface and the
//! `mlua::state::{GcMode, GcIncParams, GcGenParams}` types.
//!
//! Luau ships a single **incremental** GC (no generational mode). The control
//! ops map onto ulua's `lua_gc`:
//!
//! | mlua                | `lua_gc` op         |
//! |---------------------|---------------------|
//! | `gc_collect`        | `LUA_GCCOLLECT`     |
//! | `gc_stop`           | `LUA_GCSTOP`        |
//! | `gc_restart`        | `LUA_GCRESTART`     |
//! | `gc_is_running`     | `LUA_GCISRUNNING`   |
//! | `gc_count`          | `LUA_GCCOUNT`/`..B` |
//! | `gc_step`           | `LUA_GCSTEP`        |
//! | `gc_inc(goal,mul,sz)`| `LUA_GCSETGOAL/..` |
//!
//! **DEVIATION:** Luau has no generational GC, so `gc_gen` and
//! `GcMode::Generational` are not backed by the VM. `gc_set_mode` accepts the
//! incremental mode (applying its params) and *reports* the previous mode as
//! incremental; passing `Generational` is a no-op that returns the current
//! (incremental) mode, matching the only behavior Luau can honor.

use crate::{error::Result, state::Lua, sys::*};

/// Parameters for Luau's incremental GC, mirroring `mlua::state::GcIncParams`.
///
/// On Luau the tunables are the **goal** (heap-growth target percentage),
/// the **step multiplier**, and the **step size** (KB). (Lua 5.x's `pause` is
/// replaced by `goal` here — see [`GcIncParams::goal`].)
#[derive(Debug, Clone, Copy, Default)]
pub struct GcIncParams {
  pub(crate) goal: Option<c_int>,
  pub(crate) step_multiplier: Option<c_int>,
  pub(crate) step_size: Option<c_int>,
}

impl GcIncParams {
  /// The heap-growth goal (percentage). Luau's analog of Lua's `pause`.
  /// Mirrors `mlua::state::GcIncParams::goal`.
  pub fn goal(mut self, goal: u32) -> Self {
    self.goal = Some(goal as c_int);
    self
  }

  /// The GC step multiplier (percentage of allocation to collect per step).
  /// Mirrors `mlua::state::GcIncParams::step_multiplier`.
  pub fn step_multiplier(mut self, mul: u32) -> Self {
    self.step_multiplier = Some(mul as c_int);
    self
  }

  /// The GC step size in KB. Mirrors `mlua::state::GcIncParams::step_size`.
  pub fn step_size(mut self, size: u32) -> Self {
    self.step_size = Some(size as c_int);
    self
  }
}

/// Parameters for a generational GC, mirroring `mlua::state::GcGenParams`.
///
/// **DEVIATION:** Luau has no generational GC; this exists only for signature
/// parity with mlua's Lua 5.4/5.5 surface and is never honored by the VM.
#[derive(Debug, Clone, Copy, Default)]
pub struct GcGenParams {
  pub minor_multiplier: u32,
  pub major_multiplier: u32,
}

/// The GC operating mode, mirroring `mlua::state::GcMode`.
///
/// Luau only supports [`GcMode::Incremental`]; [`GcMode::Generational`] is
/// provided for signature parity and is treated as a no-op by [`Lua::gc_set_mode`].
#[derive(Debug, Clone, Copy)]
pub enum GcMode {
  /// Incremental GC (the only mode Luau supports).
  Incremental(GcIncParams),
  /// Generational GC — **not supported by Luau** (see the module note).
  Generational(GcGenParams),
}

impl Lua {
  /// The number of bytes currently used by the VM. Mirrors
  /// `mlua::Lua::used_memory` (ulua's `totalbytes`).
  pub fn used_memory(&self) -> usize {
    unsafe {
      let g = (*self.state()).global;
      (*g).totalbytes
    }
  }

  /// Whether the GC is currently running. Mirrors `mlua::Lua::gc_is_running`.
  pub fn gc_is_running(&self) -> bool {
    unsafe { lua_gc(self.state(), LuaGcOp::Isrunning as c_int, 0) != 0 }
  }

  /// Stop the GC. Mirrors `mlua::Lua::gc_stop`.
  pub fn gc_stop(&self) {
    unsafe { lua_gc(self.state(), LuaGcOp::Stop as c_int, 0) };
  }

  /// Restart the GC. Mirrors `mlua::Lua::gc_restart`.
  pub fn gc_restart(&self) {
    unsafe { lua_gc(self.state(), LuaGcOp::Restart as c_int, 0) };
  }

  /// The total memory in use, in KB (the `LUA_GCCOUNT` op). Mirrors
  /// `mlua::Lua::gc_count`.
  pub fn gc_count(&self) -> usize {
    unsafe { lua_gc(self.state(), LuaGcOp::Count as c_int, 0).max(0) as usize }
  }

  /// Run one incremental GC step over `kbytes` of work. Returns whether a full
  /// collection cycle finished. Mirrors `mlua::Lua::gc_step_kbytes`/`gc_step`.
  pub fn gc_step_kbytes(&self, kbytes: c_int) -> Result<bool> {
    let finished = unsafe { lua_gc(self.state(), LuaGcOp::Step as c_int, kbytes) != 0 };
    Ok(finished)
  }

  /// Run a default-size incremental GC step. Mirrors `mlua::Lua::gc_step`.
  pub fn gc_step(&self) -> Result<bool> {
    self.gc_step_kbytes(0)
  }

  /// Apply incremental-GC parameters (goal / step multiplier / step size).
  /// Mirrors `mlua::Lua::gc_inc`; returns the previous [`GcMode`] (always
  /// incremental on Luau).
  pub fn gc_inc(&self, pause: c_int, step_multiplier: c_int, step_size: c_int) -> GcMode {
    // mlua maps `pause` -> goal on the Luau backend; non-positive values are
    // left untouched.
    self.apply_inc_params(&GcIncParams {
      goal: (pause > 0).then_some(pause),
      step_multiplier: (step_multiplier > 0).then_some(step_multiplier),
      step_size: (step_size > 0).then_some(step_size),
    });
    GcMode::Incremental(GcIncParams::default())
  }

  /// Set the GC mode, returning the previous mode. Mirrors
  /// `mlua::Lua::gc_set_mode`.
  ///
  /// **DEVIATION:** Luau is always incremental. Passing
  /// [`GcMode::Incremental`] applies its params and returns the prior
  /// (incremental) mode; passing [`GcMode::Generational`] is a no-op that
  /// returns the current incremental mode.
  pub fn gc_set_mode(&self, mode: GcMode) -> GcMode {
    if let GcMode::Incremental(p) = &mode {
      self.apply_inc_params(p);
    }
    // Luau's only real mode is incremental.
    GcMode::Incremental(GcIncParams::default())
  }

  /// Apply the set incremental-GC parameters to the VM (`lua_gc` set-ops).
  /// Shared by [`Lua::gc_inc`] and [`Lua::gc_set_mode`].
  fn apply_inc_params(&self, p: &GcIncParams) {
    let state = self.state();
    unsafe {
      if let Some(goal) = p.goal {
        lua_gc(state, LuaGcOp::Setgoal as c_int, goal);
      }
      if let Some(mul) = p.step_multiplier {
        lua_gc(state, LuaGcOp::Setstepmul as c_int, mul);
      }
      if let Some(sz) = p.step_size {
        lua_gc(state, LuaGcOp::Setstepsize as c_int, sz);
      }
    }
  }
}
