//! Luau FastFlag value `FValue<T>` 的安全移植. Reference:
//! `luau/Common/include/Luau/Common.h`（the `FValue<T>` template, the
//! `LUAU_FASTFLAG*` macros, `FValueVersionSetter`）and the list walkers in
//! `luau/CLI/src/Flags.cpp`.
//!
//! Flags read as their `value`（the C++ `operator T()`，see [`FValue::get`]）；
//! 一个 per-type 全局注册表让 host 按名枚举并设置 flag 及其 version。
//!
//! Deviations from C++（documented, behavior-faithful）：
//! - The per-`T` `static FValue<T>* list` is inexpressible in Rust（no generic
//!   statics），so the registry is supplied by the [`FValueList`] trait,
//!   implemented for exactly the instantiated types（`bool`, `i32`）。C++ 的
//!   侵入式链表改为 `RwLock<Vec<&'static FValue<T>>>`：注册仅发生在
//!   ensure_flags_registered 的 `OnceLock` 串行启动窗口，遍历只拿读锁。
//! - The C++ ctor self-registers（`list = this`）。Rust statics are
//!   const-initialized with no ctor side effects, so registration runs once on
//!   first list traversal via ensure_flags_registered（a generated checklist
//!   per flag module, serialized by `OnceLock` — the C++ magic-static contract）。
//!   Reading a flag does NOT require registration — only enumeration /
//!   set-by-name does.
//! - Public mutable fields become *atomics*（`AtomicBool` / `AtomicI32` /
//!   `AtomicU32`, all `Relaxed`）：与 C++ 相同的"启动期配置、此后只读"使用
//!   方式下无任何同步开销（relaxed 原子即普通读写），同时把"运行期改 flag"
//!   从数据竞争（UB）收敛为良性竞态（读旧值或新值）。`unsafe impl Sync` 与
//!   全部 `UnsafeCell` 随之删除。
//! - cpp ctor 的 `bool dynamic` 位不落地：cpp 侧它唯一的读者是测试可执行文件
//!   `tests/main.cpp` 的 `--list-fflags` 打印（`flag->dynamic ? "D" : ""`），本端口
//!   无该入口；旗标的动态性在 Rust 由所在模块（`dfflag` / `dfint`）表达，写入即
//!   恒真/恒假的字面量，故整位连同 `FValue::new` 的第三参数一并删除。
//!
//! Downstream `LUAU_FASTFLAGVARIABLE(Foo)` becomes a
//! `static FOO: FValue<bool> = FValue::new("Foo", false);` registered at
//! startup; `FFlag::Foo` reads become `FOO.get()`.

use core::{
  ptr::from_ref,
  sync::atomic::{AtomicBool, AtomicI32, AtomicU32, Ordering},
};
use std::{
  cell::RefCell,
  collections::HashMap,
  sync::{Once, OnceLock},
};

use foldhash::fast::FixedState;
use parking_lot::RwLock;

use crate::functions::is_default_enabled_flag::is_default_enabled_flag;

/// 覆盖栈表：固定种子 foldhash，取代 std `HashMap` 默认的 SipHash
/// `RandomState`。理由与工作区先例 `ulua-analysis/src/type_aliases/collections.rs`
/// 一致 —— 键虽是 flag 的 `'static` 地址，但 `--fflags=` 的名字/顺序会经链接期
/// 影响地址分布，而 SipHash 的每进程随机种子又让桶序跨进程不可复现；
/// `FixedState` 既快又确定。
///
/// 选型与 `ulua-common/src/collections.rs` 的 `DefaultBuildHasher` 同源：
/// foldhash `FixedState` 快且迭代序跨进程可复现，gxhash 经 b31 实测在
/// 本仓键型上为负收益且种子随机，不取。
type OverrideMap<T> = HashMap<usize, Vec<T>, FixedState>;

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

/// 是否安装过任何线程本地覆盖（`get()` 快路径的开关）。仅本 crate 的
/// `FValue::get` 消费。
#[inline]
pub(crate) fn overrides_active() -> bool {
  OVERRIDES_ACTIVE.load(Ordering::Relaxed)
}

/// Per-type thread-local override stacks, keyed by the flag's `'static` address.
/// A `Vec` (stack) so nested scopes restore correctly and the entry is removed
/// when the outermost scope ends (no stale leak across tests reusing a thread).
///
/// `FValue::get`（读线程本地覆盖）挂在 `impl<T: FValueList>` 上，覆盖栈挂在
/// 本 trait 上；两者实现集相同（bool / i32），故以超 trait 关联，使
/// `T: FValueOverridable` 即可读值。
pub trait FValueOverridable: Copy + FValueList {
  fn with_overrides<R>(f: impl FnOnce(&mut OverrideMap<Self>) -> R) -> R;

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
    static BOOL_OVERRIDES: RefCell<OverrideMap<bool>> =
        RefCell::new(OverrideMap::default());
    static INT_OVERRIDES: RefCell<OverrideMap<i32>> =
        RefCell::new(OverrideMap::default());
}

impl FValueOverridable for bool {
  fn with_overrides<R>(f: impl FnOnce(&mut OverrideMap<Self>) -> R) -> R {
    BOOL_OVERRIDES.with(|c| f(&mut c.borrow_mut()))
  }
}

impl FValueOverridable for i32 {
  fn with_overrides<R>(f: impl FnOnce(&mut OverrideMap<Self>) -> R) -> R {
    INT_OVERRIDES.with(|c| f(&mut c.borrow_mut()))
  }
}

/// flag 值的线程安全存储：关联到具体原子类型（`AtomicBool` / `AtomicI32`）。
/// 全部操作为 `Relaxed` —— C++ 本就无任何同步（纯读写），relaxed 原子与其
/// 同成本，并把并发写从 UB 收敛为良性竞态。
pub trait FValueCell: Copy {
  type Storage: Send + Sync;
  fn load(s: &Self::Storage) -> Self;
  fn store(s: &Self::Storage, v: Self);
}

impl FValueCell for bool {
  type Storage = AtomicBool;
  fn load(s: &AtomicBool) -> bool {
    s.load(Ordering::Relaxed)
  }
  fn store(s: &AtomicBool, v: bool) {
    s.store(v, Ordering::Relaxed);
  }
}

impl FValueCell for i32 {
  type Storage = AtomicI32;
  fn load(s: &AtomicI32) -> i32 {
    s.load(Ordering::Relaxed)
  }
  fn store(s: &AtomicI32, v: i32) {
    s.store(v, Ordering::Relaxed);
  }
}

pub struct FValue<T: FValueCell> {
  pub(crate) value: T::Storage,
  pub(crate) name: &'static str,
  pub(crate) version: AtomicU32,
}

// cpp `FValue<T>::FValue(const char* name, T def, bool dynamic)`
// （`Common/include/Luau/Common.h`）：字段初始化，提为 `const fn` 以便旗标写成
// `static`。cpp 构造的 `list = this` 副作用改由 [`FValue::register`] 承担（见模块
// 头偏差清单）；`dynamic` 实参本端口无消费者，整位删除。
//
// 泛型构造无法写成 `const fn`（原子存储在 trait 里构造需要 `~const`），故按当前仅有的
// 两种旗标类型（bool / i32）各给一个具体构造，宏展开点不受影响。
impl FValue<bool> {
  pub const fn new(name: &'static str, def: bool) -> Self {
    FValue {
      value: AtomicBool::new(def),
      name,
      version: AtomicU32::new(0),
    }
  }
}

impl FValue<i32> {
  pub const fn new(name: &'static str, def: i32) -> Self {
    FValue {
      value: AtomicI32::new(def),
      name,
      version: AtomicU32::new(0),
    }
  }
}

/// 覆盖栈的键：flag 的 `'static` 地址。同一实例在全程序内地址唯一，故可寻址。
#[inline]
fn addr<T: FValueCell>(flag: &FValue<T>) -> usize {
  from_ref(flag) as usize
}

impl<T: FValueOverridable> FValue<T> {
  /// cpp `FValue<T>::operator T() const`（`Common.h` 的 `LUAU_FORCEINLINE operator
  /// T()`）：读旗标当前值。建模为 `get()` 而非 `From`/`Deref` 转换 —— 值住在原子里
  /// （`static` 旗标因此仍可变），按拷贝返回而非引用。
  pub fn get(&self) -> T {
    // 快路径：从未有测试装过覆盖 -> 直读全局。
    // 装了 `ScopedFastFlag`/`ScopedFastInt` 的测试会翻起 `OVERRIDES_ACTIVE`，其
    // 线程本地覆盖随后只遮蔽本线程的全局值（并行测试互不干扰）。
    if overrides_active()
      && let Some(v) = T::override_top(addr(self))
    {
      return v;
    }
    // Relaxed 读取与 C++ 普通读同成本;启动期之外的竞争按良性数据竞争处理,不再触发 UB。
    T::load(&self.value)
  }

  /// Install a thread-local override for this flag (used by the test scope
  /// guard `ScopedFValue`). Visible only to the current thread until popped.
  pub fn push_test_override(&self, value: T) {
    T::override_push(addr(self), value);
  }

  /// Remove the most recent thread-local override for this flag.
  pub fn pop_test_override(&self) {
    T::override_pop(addr(self));
  }
}

/// C++ `setLuauFlagsDefault` analog (CLI default-on behavior): enable the
/// `Luau*` non-experimental bool flags (see `is_default_enabled_flag`).
/// Call before threads start.
///
/// 进程内幂等：批量写只在**首次**调用发生。`FValue::set_all_unless` 写的是
/// 数百个全局原子，若每次 `eval` / `execute_script` / wasm `run` 都重跑一遍，
/// 就会与另一线程正在跑的 VM 的 `get()` 构成良性但无谓的竞态。收口成 `Once`
/// 后，重复调用至多串行等待首个调用完成，之后的调用是 no-op——旗标自始即按
/// 「启动期配置、此后只读」对待。
///
/// 因此首次调用之后传入相反值不会生效；需要显式改旗标走 [`FValue::set`]
/// （测试期用 [`FValue::push_test_override`] 的线程本地覆盖），或 CLI 的
/// `--fflags=`（直调 [`FValue::set_all_unless`]，不经本入口）。
pub fn set_luau_bool_flags(value: bool) {
  static APPLIED: Once = Once::new();
  APPLIED.call_once(|| {
    FValue::<bool>::set_all_unless(value, |name| !is_default_enabled_flag(name));
  });
}

/// Supplies the per-type flag registry, replacing the inexpressible C++
/// `static FValue<T>* list`. Implemented for exactly the instantiated types.
pub trait FValueList: FValueCell + Sized {
  /// 本类型的全局注册表。注册（启动期写锁追加）仅发生在
  /// ensure_flags_registered 的 `OnceLock` 串行窗口；此后内容不再变化，
  /// 遍历只拿读锁。C++ 头插链表的遍历序（注册逆序）对行为无影响：
  /// `set_all_unless` 全量写、`set_flag_by_name` 名字唯一首命中。
  fn registry() -> &'static RwLock<Vec<&'static FValue<Self>>>;
}

static BOOL_REGISTRY: RwLock<Vec<&'static FValue<bool>>> = RwLock::new(Vec::new());
static INT_REGISTRY: RwLock<Vec<&'static FValue<i32>>> = RwLock::new(Vec::new());

impl FValueList for bool {
  fn registry() -> &'static RwLock<Vec<&'static FValue<bool>>> {
    &BOOL_REGISTRY
  }
}

impl FValueList for i32 {
  fn registry() -> &'static RwLock<Vec<&'static FValue<i32>>> {
    &INT_REGISTRY
  }
}

/// C++ `FValue` ctor 的 `list = this` 自注册在 Rust 中无法静态执行，注册表
/// 曾保持为空 —— 导致 `set_flag_by_name`/`set_all_unless`（CLI `--fflags=`、
/// web `set_luau_bool_flags`）静默无效。此入口按 [`crate::fflag`]/[`crate::fint`]/
/// [`crate::dfint`] 生成的清单一次性补挂注册表，首次链表操作时触发；
/// `get_or_init` 串行化保证与 C++ 静态初始化同样的只跑一次语义。
pub(crate) fn ensure_flags_registered() {
  use crate::{dfflag, dfint, fflag, fint};
  static DONE: OnceLock<()> = OnceLock::new();
  DONE.get_or_init(|| {
    fflag::register_flags();
    fint::register_flags();
    dfint::register_flags();
    dfflag::register_flags();
  });
}

impl<T: FValueCell> FValue<T> {
  pub fn set(&self, value: T) {
    T::store(&self.value, value);
  }

  /// Current `version` (0 unless a `LUAU_FLAGVERSION` setter ran).
  pub fn version(&self) -> u32 {
    self.version.load(Ordering::Relaxed)
  }

  /// `LUAU_FLAGVERSION(name, v)` 对应：为 flag 标记版本号。C++ 由
  /// `FValueVersionSetter` 在静态初始化时遍历链表完成；Rust 的 flag 是
  /// const 构造，故在注册清单中直接写入同一槽位。
  pub fn set_version(&self, version: u32) {
    self.version.store(version, Ordering::Relaxed);
  }
}

impl<T: FValueList> FValue<T> {
  /// The C++ ctor side effect `next = list; list = this`. Call once, on the
  /// flag's `'static` instance, after construction. 仅允许在
  /// ensure_flags_registered 的 `OnceLock` 串行启动窗口内调用（与 C++ 静态
  /// 初始化的单线程语义一致）；每个 flag 至多注册一次。
  pub fn register(&'static self) {
    let mut registry = T::registry().write();
    registry.push(self);
  }
}

/// 在 per-type 注册表的读锁内跑 `body`：先补注册（cpp 静态初始化窗口），再把
/// 注册项切片交给调用方，遍历形态（`for` / `find`）由调用方自选。
fn with_flags<T: FValueList + 'static, R>(body: impl FnOnce(&[&'static FValue<T>]) -> R) -> R {
  ensure_flags_registered();
  let registry = T::registry().read();
  body(&registry)
}

/// `--fflags=<name>[<version>]` 的命中判定：精确名，或 `<name><version>` 形式的
/// 后缀是"无前导零的十进制数"（与 `version.to_string()` 逐字节等价）且数值相等。
fn name_matches<T: FValueList>(flag: &FValue<T>, name: &str) -> bool {
  if name == flag.name {
    return true;
  }
  let version = flag.version();
  version != 0
    && name.strip_prefix(flag.name).is_some_and(|suffix| {
      !suffix.is_empty()
        && !suffix.starts_with('0')
        && suffix.bytes().all(|b| b.is_ascii_digit())
        && suffix.parse::<u32>() == Ok(version)
    })
}

impl<T: FValueList + 'static> FValue<T> {
  /// Walk the per-type registry and set every flag to `value` unless
  /// the host-supplied `skip` predicate (matched on the flag's UTF-8 name)
  /// returns true. Models the bool `--fflags=true|false` branch of the C++
  /// test harness `setFastFlags`, which sets every non-skipped flag.
  ///
  /// 行为契约（无内存安全后果）：存储已原子化，并发调用不产生 UB（读到的
  /// 至多是旧值或新值）；但仍应与 C++ 一致，按"启动期配置、此后只读"对待，
  /// 运行期改 flag 会让依赖默认值的代码路径观察到非预期值。
  pub fn set_all_unless(value: T, skip: impl Fn(&str) -> bool) {
    with_flags(|flags| {
      for flag in flags.iter().filter(|f| !skip(f.name)) {
        flag.set(value);
      }
    });
  }

  /// Walk the flag registry, matching either exact name or `<name><version>`.
  /// Returns true if a flag was found and set.
  ///
  /// 行为契约同 [`FValue::set_all_unless`]：实现已原子化，调用无内存安全后果。
  pub fn set_flag_by_name(name: &str, value: T) -> bool {
    with_flags(|flags| {
      let Some(flag) = flags.iter().copied().find(|f| name_matches(f, name)) else {
        return false;
      };
      flag.set(value);
      true
    })
  }
}
