//! Faithful port of Luau's FastFlag value `FValue<T>`. Reference:
//! `luau/Common/include/Luau/Common.h` (the `FValue<T>` template, the
//! `LUAU_FASTFLAG*` macros, `FValueVersionSetter`) and the list walkers in
//! `luau/CLI/src/Flags.cpp`. Oracle: `/tmp/fastflag_proto.rs` (reads, enumerate,
//! runtime-set, version-set-by-name, unknown-flag guard — all pass).
//!
//! Flags read as their `value` (the C++ `operator T()`, see [`FValue::get`]); a
//! per-type intrusive list lets the host enumerate flags and set them / their
//! version by name.
//!
//! Deviations from C++ (documented, behavior-faithful):
//! - The per-`T` `static FValue<T>* list` is inexpressible in Rust (no generic
//!   statics), so the head is supplied by the [`FValueList`] trait, implemented
//!   for exactly the instantiated types (`bool`, `i32`).
//! - The C++ ctor self-registers (`list = this`). Rust statics are
//!   const-initialized with no ctor side effects, so registration is the explicit
//!   [`FValue::register`], called once on a flag's `'static` instance. Reading a
//!   flag does NOT require registration — only enumeration / set-by-name does.
//! - Public mutable fields become `UnsafeCell` + `unsafe impl Sync`, matching the
//!   C++ contract (flags configured before worker threads start, read-only after;
//!   C++ has no synchronization here either).
//!
//! Downstream `LUAU_FASTFLAGVARIABLE(Foo)` becomes a
//! `static FOO: FValue<bool> = FValue::new(c"Foo", false, false);` registered at
//! startup; `FFlag::Foo` reads become `FOO.get()`.

use core::{
  cell::UnsafeCell,
  sync::atomic::{AtomicBool, AtomicPtr, Ordering},
};
use std::{cell::RefCell, collections::HashMap, ptr::null_mut};

// ---------------------------------------------------------------------------
// Thread-local flag overrides (test isolation)
//
// C++ doctest runs single-threaded, so `ScopedFastFlag`/`ScopedFastInt` (which
// mutate a process-global flag) are safe. Rust's libtest runs tests in PARALLEL
// threads, so a global mutation by one test leaks into another reading the same
// flag — producing nondeterministic failures (and, when a recursion/length
// LIMIT leaks, runaway recursion that overflows a test thread's stack).
//
// Fix: a per-thread override layer. A scoped guard pushes its value onto a
// thread-local stack for that flag; `get()` returns the current thread's
// override when present, else the global. Mutations are thus private to the
// thread (and the scope) that made them — parallel tests no longer interfere,
// and the production path is unchanged.
//
// Cost in production: `get()` does one relaxed atomic load of `OVERRIDES_ACTIVE`
// (set the first time any scoped guard runs — i.e. never, outside tests) before
// the normal read. No cargo feature needed; the flag stays false in real runs.
// ---------------------------------------------------------------------------

/// Set true the first time a scoped override is pushed. Until then `get()` skips
/// the thread-local lookup entirely.
static OVERRIDES_ACTIVE: AtomicBool = AtomicBool::new(false);

/// Has any thread-local override ever been installed in this process?
#[inline]
pub fn overrides_active() -> bool {
  OVERRIDES_ACTIVE.load(Ordering::Relaxed)
}

/// Per-type thread-local override stacks, keyed by the flag's `'static` address.
/// A `Vec` (stack) so nested scopes restore correctly and the entry is removed
/// when the outermost scope ends (no stale leak across tests reusing a thread).
pub trait FValueOverridable: Copy {
  fn with_overrides<R>(f: impl FnOnce(&mut HashMap<usize, Vec<Self>>) -> R) -> R;

  fn override_top(addr: usize) -> Option<Self> {
    Self::with_overrides(|m| m.get(&addr).and_then(|s| s.last().copied()))
  }
  fn override_push(addr: usize, value: Self) {
    OVERRIDES_ACTIVE.store(true, Ordering::Relaxed);
    Self::with_overrides(|m| m.entry(addr).or_default().push(value));
  }
  fn override_pop(addr: usize) {
    Self::with_overrides(|m| {
      if let Some(stack) = m.get_mut(&addr) {
        stack.pop();
        if stack.is_empty() {
          m.remove(&addr);
        }
      }
    });
  }
}

thread_local! {
    static BOOL_OVERRIDES: RefCell<HashMap<usize, Vec<bool>>> =
        RefCell::new(HashMap::new());
    static INT_OVERRIDES: RefCell<HashMap<usize, Vec<i32>>> =
        RefCell::new(HashMap::new());
}

impl FValueOverridable for bool {
  fn with_overrides<R>(f: impl FnOnce(&mut HashMap<usize, Vec<Self>>) -> R) -> R {
    BOOL_OVERRIDES.with(|c| f(&mut c.borrow_mut()))
  }
}

impl FValueOverridable for i32 {
  fn with_overrides<R>(f: impl FnOnce(&mut HashMap<usize, Vec<Self>>) -> R) -> R {
    INT_OVERRIDES.with(|c| f(&mut c.borrow_mut()))
  }
}

impl<T: FValueOverridable> FValue<T> {
  /// Install a thread-local override for this flag (used by the test scope
  /// guard `ScopedFValue`). Visible only to the current thread until popped.
  pub fn push_test_override(&self, value: T) {
    T::override_push(self as *const FValue<T> as usize, value);
  }

  /// Remove the most recent thread-local override for this flag.
  pub fn pop_test_override(&self) {
    T::override_pop(self as *const FValue<T> as usize);
  }
}

pub struct FValue<T> {
  pub(crate) value: UnsafeCell<T>,
  pub(crate) dynamic: bool,
  pub(crate) name: &'static str,
  pub(crate) next: UnsafeCell<*const FValue<T>>,
  pub(crate) version: UnsafeCell<u32>,
}

// See the module deviation note: flags are configured before threads start and
// treated as read-only afterwards.
unsafe impl<T: Sync> Sync for FValue<T> {}

/// C++ `setLuauFlagsDefault(bool)` analog (CLI default-on behavior): walk the
/// bool-flag registry and set every non-Debug flag. Call before threads start.
pub fn set_luau_bool_flags(value: bool) {
  unsafe {
    let mut cur = <bool as FValueList>::head().load(Ordering::Relaxed) as *const FValue<bool>;
    while !cur.is_null() {
      if !(*cur).name.starts_with("Debug") {
        *(*cur).value.get() = value;
      }
      cur = *(*cur).next.get();
    }
  }
}

/// Supplies the per-type intrusive-list head, replacing the inexpressible C++
/// `static FValue<T>* list`. Implemented for exactly the instantiated types.
pub trait FValueList: Sized {
  fn head() -> &'static AtomicPtr<FValue<Self>>;
}

static FVALUE_LIST_BOOL: AtomicPtr<FValue<bool>> = AtomicPtr::new(null_mut());
impl FValueList for bool {
  fn head() -> &'static AtomicPtr<FValue<bool>> {
    &FVALUE_LIST_BOOL
  }
}

static FVALUE_LIST_INT: AtomicPtr<FValue<i32>> = AtomicPtr::new(null_mut());
impl FValueList for i32 {
  fn head() -> &'static AtomicPtr<FValue<i32>> {
    &FVALUE_LIST_INT
  }
}

impl<T: Copy> FValue<T> {
  /// Runtime flag set (the CLI/host path mutates the public `value` field).
  pub fn set(&self, value: T) {
    unsafe { *self.value.get() = value };
  }

  /// Current `version` (0 unless a `LUAU_FLAGVERSION` setter ran).
  pub fn version(&self) -> u32 {
    unsafe { *self.version.get() }
  }

  pub fn dynamic(&self) -> bool {
    self.dynamic
  }
}

impl<T: FValueList> FValue<T> {
  /// The C++ ctor side effect `next = list; list = this;`. Call once, on the
  /// flag's `'static` instance, after construction.
  ///
  /// # Safety
  /// Must be called at most once per flag, before any concurrent list walk
  /// (registration is single-threaded startup work, as in C++).
  pub unsafe fn register(&'static self) {
    unsafe {
      let head = T::head();
      let old = head.load(Ordering::Relaxed);
      *self.next.get() = old as *const FValue<T>;
      head.store(
        self as *const FValue<T> as *mut FValue<T>,
        Ordering::Relaxed,
      );
    }
  }
}

impl<T: FValueList + Copy + 'static> FValue<T> {
  /// Walk the per-type `FValue<T>::list` and set every flag whose `name`
  /// matches to `value` (the C++ test harness `setFastValue<T>(name, value)`
  /// in `tests/main.cpp`). Configured at startup before worker threads — the
  /// same single-threaded contract as flag construction.
  pub fn set_value_by_name(name: &str, value: T) {
    unsafe {
      let mut cur = T::head().load(Ordering::Relaxed) as *const FValue<T>;
      while !cur.is_null() {
        let fvalue = &*cur;
        if fvalue.name == name {
          *fvalue.value.get() = value;
        }
        cur = *fvalue.next.get();
      }
    }
  }

  /// Walk the per-type `FValue<T>::list` and set every flag to `value` unless
  /// the host-supplied `skip` predicate (matched on the flag's UTF-8 name)
  /// returns true. Models the bool `--fflags=true|false` branch of the C++
  /// test harness `setFastFlags`, which sets every non-skipped flag. Startup-
  /// only, single-threaded — the same contract as flag construction.
  pub fn set_all_unless(value: T, skip: impl Fn(&str) -> bool) {
    unsafe {
      let mut cur = T::head().load(Ordering::Relaxed) as *const FValue<T>;
      while !cur.is_null() {
        let fvalue = &*cur;
        if !skip(fvalue.name) {
          *fvalue.value.get() = value;
        }
        cur = *fvalue.next.get();
      }
    }
  }

  /// Walk the flag list, matching either exact name or `<name><version>`.
  /// Returns true if a flag was found and set.
  pub fn set_flag_by_name(name: &str, value: T) -> bool {
    unsafe {
      let mut cur = T::head().load(Ordering::Relaxed) as *const FValue<T>;
      while !cur.is_null() {
        let flag = &*cur;
        if name == flag.name {
          flag.set(value);
          return true;
        }

        let version = flag.version();
        if version != 0 {
          let version_str = version.to_string();
          let prefix_len = flag.name.len();
          if name.len() == prefix_len + version_str.len()
            && name.as_bytes()[..prefix_len] == *flag.name.as_bytes()
            && &name.as_bytes()[prefix_len..] == version_str.as_bytes()
          {
            flag.set(value);
            return true;
          }
        }
        cur = *flag.next.get();
      }
      false
    }
  }
}
