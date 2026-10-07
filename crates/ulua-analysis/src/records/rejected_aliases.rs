//! `Scope::invalid_type_aliases` 的共享可变封装（review §2「确需内部可变时用
//! `&UnsafeCell` 局部封装，把 unsafe 关进有契约的最小边界」）。
//!
//! cpp 侧 `Scope::invalidTypeAliases` 经 `Scope*` 直写（`scope->invalidTypeAliases
//! .try_insert(name, location)`），而 Rust 侧 `Scope` 由 `ScopePtr = Arc<Scope>`
//! 共享持有，可达的可变形态只有 `&Scope`。旧写法在 `&self` 方法里
//! `from_ref(self).cast_mut()` 再解写：经共享引用派生的指针解写即 UB，且令
//! `unsafe impl Sync for Scope` 的「并发持有 `&Scope` 为纯读」契约落空
//! （先例同 `LuaTable.tmcache: Cell<u8>`）。本类型把该写入收进内部可变的两个
//! 方法，`Scope` 侧调用点全部转 safe。

use core::{
  cell::UnsafeCell,
  fmt::{Debug, Formatter, Result},
  panic::RefUnwindSafe,
};

use ulua_ast::records::location::Location;
use ulua_common::records::dense_hash_map::DenseHashMap;

/// 键为别名名、值为声明处 `Location` 的登记表；语义等同 cpp
/// `DenseHashMap<std::string, Location> invalidTypeAliases`。
#[derive(Default)]
pub struct RejectedAliases(UnsafeCell<DenseHashMap<String, Location>>);

/// 共享可变容器的公共读口契约：单线程分析驱动（cpp 同款前提），全部写入只经
/// [`RejectedAliases::record`] 发生，且求解器为递归下降单线程执行，故任一时刻
/// 至多一个持有者，读写不并存于跨线程。
impl RejectedAliases {
  /// cpp `invalidTypeAliases.try_insert(name, location)`：键已存在则保留原值。
  ///
  /// # Safety 前提（由本类型全部使用者共同成立的单线程契约）
  /// 调用方至多持有 `&Self`，且同线程内不存在对同一表的其他可变访问；本方法是
  /// 全 crate 对该表的唯一写入点。
  pub fn record(&self, name: String, location: Location) {
    // SAFETY: 见本方法 `# Safety 前提`；写入经内部可变收口，不再经共享引用的
    // 派生指针，故 provenance 合法。
    unsafe { (*self.0.get()).try_insert(name, location) };
  }

  /// 借用查询：按 `&str` 命中已登记的 `Location`（零分配，cpp `find` 同形）。
  ///
  /// # Safety 前提
  /// 同 [`Self::record`]：读取期间本线程不会并发写入同一表（单线程驱动下
  /// `find` 与 `record` 必然串行）。
  pub fn find(&self, name: &str) -> Option<Location> {
    // SAFETY: 见本方法 `# Safety 前提`；返回值按值拷贝，不把内部引用外渗给调用方。
    unsafe { &*self.0.get() }.find_str(name).copied()
  }
}

impl Debug for RejectedAliases {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    // SAFETY: 纯读诊断输出；单线程契约下同 [`Self::find`]，无并发写入。
    unsafe { &*self.0.get() }.fmt(f)
  }
}

/// 内部可变不外溢为「可观察的可变性」：本表只增不改（cpp `try_insert` 语义），
/// panic 展开后最多多出一条拒绝记录，读侧只会因此命中而非破坏不变量，故跨
/// `catch_unwind` 边界持有 `&Self` 视为安全（`Mutex`/`RwLock` 同类先例）。
/// 缺此 impl 时 `Scope` 不再是 `RefUnwindSafe`，并经 `Type` 的 arena 指针臂
/// 把 `ulua-unit-test/tests/visit_type.rs` 的 `catch_unwind` 用例在编译期打断。
///
/// # Safety
/// 前提同 [`RejectedAliases::record`] 的单线程契约。
impl RefUnwindSafe for RejectedAliases {}
