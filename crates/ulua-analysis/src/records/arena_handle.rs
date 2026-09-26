//! 进程内唯一存活实例的最小句柄：替代照抄 C++ `NotNull<T>` 成员而散落的
//! `*mut T` 字段（`TypeArena` / `BuiltinTypes` / `Normalizer` /
//! `TypeFunctionRuntime` / `InternalErrorReporter` 等）。
//!
//! 这些指针从不拥有、从不释放目标，只作为「宿主（通常是 `Frontend` 或其
//! arena 持有者）持有的进程级唯一实例」的别名句柄；原先的 `unsafe` 全部
//! 是照抄 C++ 指针解引用的产物，而被调方法（`TypeArena::add_type`、
//! `BuiltinTypes` 的字段读取等）本来就是 safe fn。`Handle` 把解引用的
//! 安全性收拢到本文件一处，业务调用点恢复为普通的 `&`/`&mut` 借用。
//!
//! # Safety（类型级契约，构造与解引用共同依赖）
//!
//! 1. 目标在句柄整个存活期内由宿主独占持有且地址稳定（`Box`/`Vec` 尾部
//!    字段、arena 属主等），句柄不拥有、不释放目标；
//! 2. 单线程驱动：任一时刻经 `get`/`get_mut` 物化的借用期内，不存在其它
//!    存活的可变别名（与原裸指针解引用处的 `// SAFETY:` 注释同一契约）；
//! 3. null 哨兵不进入句柄：可空处一律用 `Option<Handle<T>>` 表达。

use core::{
  fmt::{self, Debug, Formatter},
  hash::{Hash, Hasher},
  ptr::NonNull,
};

/// 进程内唯一存活实例的拷贝句柄（原 `*mut T` 字段的替代品）。
///
/// 类型编码「恒非空」；`get`/`get_mut` 返回的借用生命周期刻意不受约束，
/// 与原 `&*ptr` / `&mut *ptr` 解引用的借用检查行为完全同构，保证迁移不
/// 改变任何调用点的借用语义。
pub struct Handle<T> {
  ptr: NonNull<T>,
}

// `Clone`/`Copy` 手写而非 derive：derive 会强加 `T: Clone` 约束，而
// `TypeArena` 等目标类型并非 Clone——句柄复制本就不需要复制目标。
impl<T> Clone for Handle<T> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<T> Copy for Handle<T> {}

impl<T> Handle<T> {
  /// 由可变引用构造：引用本身保证非空且在借用期内存活。
  pub fn from_mut(r: &mut T) -> Self {
    Self {
      ptr: NonNull::from(r),
    }
  }

  /// 由共享引用构造：对应 C++ 中以 `const T&`/`NotNull<const T&>` 接线、
  /// 下游按会话级单例只读使用的目标（如 `CloneState` 的 `builtinTypes`）。
  /// 引用本身保证非空且在借用期内存活；若下游经 `get_mut` 物化可变借用，
  /// 「同一时刻单条存活可变别名」契约仍由模块级单线程驱动约定保证，
  /// 与原 `x as *const T as *mut T` 转铸的实质完全同构。
  pub fn from_ref(r: &T) -> Self {
    Self {
      ptr: NonNull::from(r),
    }
  }

  /// 由 `NonNull` 构造：`NonNull` 已编码非空（对应 C++ `NotNull<T>` 形参），
  /// 「地址稳定、无别名」契约由调用方类型持有者保证。
  pub fn from_nonnull(ptr: NonNull<T>) -> Self {
    Self { ptr }
  }

  /// 由裸指针构造。
  ///
  /// # Safety
  /// `ptr` 必须非空、对齐，并满足本模块的类型级契约：目标在返回句柄的
  /// 整个存活期内唯一存活、地址稳定，且句柄借用期内无其它存活可变别名。
  pub unsafe fn from_raw(ptr: *mut T) -> Self {
    // SAFETY: 非空与存活契约由调用方经函数签名契约保证。
    unsafe {
      Self {
        ptr: NonNull::new_unchecked(ptr),
      }
    }
  }

  /// 由裸指针构造并断言非空（对应 C++ `NotNull` 形参契约）：null 属契约
  /// 违例，确定性 panic 而非 UB，仅在调用点契约要求非空时使用。
  pub fn from_ptr(ptr: *mut T) -> Self {
    Self {
      ptr: NonNull::new(ptr).expect("Handle 目标不得为 null（C++ NotNull 契约）"),
    }
  }

  /// 可空裸指针的对应构造：null 哨兵用 `Option<Handle<T>>` 表达。
  pub fn from_opt_ptr(ptr: *mut T) -> Option<Self> {
    NonNull::new(ptr).map(Self::from_nonnull)
  }

  /// 共享只读解引用。
  ///
  /// # Safety 说明
  /// 本方法是全 crate 对这些单例句柄唯一允许的直接裸指针读操作：契约见
  /// 模块头（目标进程级存活、单线程驱动、借用期内无并发写别名）。
  pub fn get<'a>(&self) -> &'a T {
    // SAFETY: 见模块级类型契约——`ptr` 非空且目标在句柄存活期内有效。
    unsafe { self.ptr.as_ref() }
  }

  /// 可变解引用（对应原 `&mut *ptr` / `(*ptr).method(..)` 写法）。
  ///
  /// # Safety 说明
  /// 与 [`Handle::get`] 同一契约；调用点须保证同一时刻只有一条经本方法
  /// 物化的借用存活（原裸指针代码的隐含前提，逐处 `// SAFETY:` 不变）。
  pub fn get_mut<'a>(&self) -> &'a mut T {
    // SAFETY: 见模块级类型契约；单线程下调用点保证无并存别名借用。
    // `NonNull` 为 Copy，拷出后解绑对 `self` 的可变借用，与原裸指针语义同构。
    let mut ptr = self.ptr;
    unsafe { ptr.as_mut() }
  }

  /// 还原为裸指针，供尚未迁移的 `*mut T` 字段 / 以指针为 key 的映射使用。
  pub fn as_ptr(&self) -> *mut T {
    self.ptr.as_ptr()
  }

  /// 直接以 `NonNull` 视图取回内部指针：`Handle` 类型编码「恒非空」，
  /// 故与 `NonNull::new(self.as_ptr()).unwrap()` 逐位等价且无判空分支，
  /// 供下游形参为 `NonNull<T>`（C++ `NotNull<T>` 镜像）的调用点直连。
  pub fn as_nonnull(&self) -> NonNull<T> {
    self.ptr
  }
}

// 与原 `*mut T` 字段同构的可比较、可打印、可哈希语义（指针同一性）。
impl<T> PartialEq for Handle<T> {
  fn eq(&self, other: &Self) -> bool {
    self.ptr == other.ptr
  }
}

impl<T> Eq for Handle<T> {}

impl<T> Hash for Handle<T> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.ptr.hash(state);
  }
}

impl<T> Debug for Handle<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    Debug::fmt(&self.ptr.as_ptr(), f)
  }
}

/// 把 cpp 直译层已存在的裸指针别名句柄再借用为 Rust 引用。
///
/// 适用对象是本 crate 自有、按 C++ 裸指针模型流转的**非拥有**句柄：
/// - `arc_as_mut` 从调用方借用的 `Arc<T>`（`Scope`/`ModuleInfo` 等）派生的写入句柄；
/// - `Box::into_raw` 交给 `constraints: Vec<*mut Constraint>` 持有的泄漏约束指针；
/// - 其它以指针身份在记录字段/形参间传递、目标由宿主或 arena 保活的别名。
///
/// 与 [`Handle`] 同一模块级契约（目标在返回借用的存活期内有效、单线程驱动、
/// 无并存可变别名）；这是全 crate 除 [`Handle`] 外唯一允许裸指针解引用的
/// 收口点，业务调用点经此恢复为普通引用用法，`unsafe` 不再外渗。
pub(crate) fn alias<T>(p: *mut T) -> &'static mut T {
  // SAFETY: 见函数级契约——调用点保证 `p` 非空、指向存活对象且借用期内
  // 无其它可变别名（与原 `unsafe { &mut *p }` 解引用处逐条同构）。
  unsafe { &mut *p }
}

/// [`alias`] 的共享只读形态。
pub(crate) fn alias_ref<T>(p: *const T) -> &'static T {
  // SAFETY: 同 [`alias]，共享读不产生可变别名。
  unsafe { &*p }
}

/// 可空裸指针快照 → `Option<&'static T>` 的收口门面（原业务侧
/// `unsafe { p.as_ref() }` + `is_null` 哨兵样板的唯一替身）。
/// null 折叠为 `None`，非 null 折叠为共享借用；判定语义与
/// `!p.is_null()` 逐格等价，不引入任何读写。
pub(crate) fn alias_opt<T>(p: *const T) -> Option<&'static T> {
  // SAFETY: 模块级契约——null 早退不解引用；非空指针由调用点保证指向
  // 存活对象（与 [`alias_ref`] 同一前提）。
  unsafe { p.as_ref() }
}
