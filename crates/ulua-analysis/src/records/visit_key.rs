//! `VisitKey` / `VisitKeyRef` —— 分析层 visitor 已访问集合（seen-set）的身份键。
//!
//! 背景：C++ `GenericTypeVisitor<S>` 的 `seen` 集合以类型/类型包节点的
//! 指针地址为去重键（`hasSeen`/`unsee` 只比较地址，从不解引用），移植时
//! 落成散落的 `HashSet<*mut ()>` / `DenseHashSet<*mut ()>` /
//! `DenseHashSet<*const ()>` 与各处 `x as *mut ()`、`x as *const ()`
//! 裸指针转换。本 newtype 把「地址即身份」这一既有契约收敛到单一受审计点，
//! 调用点退化为 `DenseHashSet<VisitKey>` 等，不再手写指针转换。
//!
//! §2（arena/自引用图的索引句柄化）：键的**载荷是地址数值（`usize`）而不是
//! 裸指针类型**。本键永不解引用，只参与 `Hash`/`Eq`/`Ord`，故以数值承载与
//! 以指针承载完全等价，而数值形态带来三点收益：
//!
//! 1. 本模块与全仓 seen-set 不再出现 `*mut ()`/`*const ()` 这类「假引用」类型，
//!    指针解引用面收窄到 arena 侧真正的节点句柄；
//! 2. 键不再被误当成可解引用的指针传来传去（旧形态 `key.0 as *const Type` 一
//!    旦出现即 UB 隐患）；
//! 3. 后续波次把 `TypeId` 换成 `u32` 索引句柄时，只需替换 [`VisitKey::from_ptr`]
//!    的实现（转发到索引号），键的对外形态、`DenseDefault` 占位与全部 48 处
//!    调用点均不变。
//!
//! 逐字节等价的论证：`Hash for *const T/*mut T` 对瘦指针即 `write_usize(地址)`，
//! 与 `usize` 的 `Hash` 同一条 `write_usize`；`Ord` 转发到地址即 `usize` 的地址序；
//! `DenseDefault` 空槽占位为地址 0（= 原 null 指针）。故遍历顺序、环检测与报错
//! 消息均与迁移前一致（见 `tests/visit_key.rs` 三条契约钉）。
//!
//! 姊妹类型（分型而非泛型化）：上一波迁移的 28 处键为 `*mut ()` 极性，
//! 且约束求解器内既有 `DenseHashSet<VisitKey>` 用法已按 `*mut ()` 逐字节
//! 对拍定案；本波新收敛的集合原键为 `*const ()` 极性。载荷改成地址数值后，
//! 两极性在 `Hash`/`Eq`/`Ord` 上完全同构，`VisitKeyRef` 因此退化为
//! [`VisitKey`] 的具名别名——保留名字只为让调用点继续自述其来源极性，
//! 且 `BTreeMap` 所需的 `Ord` 由同一实现提供（单字段数值 newtype 的派生
//! `Ord` 即地址序，与原 `BTreeMap<*mut (), _>` 的排序键逐位一致）。

use ulua_common::records::dense_hash_table::DenseDefault;

/// 空槽占位地址：对应迁移前 null 指针的位模式。
const NULL_ADDR: usize = 0;

/// 以节点地址为身份的 seen-set 键（§2 索引句柄层的最小身份原语）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VisitKey(usize);

impl VisitKey {
  /// 由任意（只读）节点指针建立身份键。语义等同原 `ptr as *mut ()`：仅保留
  /// 地址身份，调用方持有的 arena 生命周期即本键的有效性前提（C++ 同契约）。
  ///
  /// 本函数是「指针身份 → 数值身份」的唯一折算点：`TypeId` 索引化后改这里即可。
  #[inline]
  pub fn from_ptr<T>(ptr: *const T) -> Self {
    Self(ptr as *const () as usize)
  }

  /// 空槽占位键（原 null 指针键），仅供 [`DenseDefault`] 与判等比较使用。
  #[inline]
  pub const fn null() -> Self {
    Self(NULL_ADDR)
  }
}

impl DenseDefault for VisitKey {
  /// 空槽占位与 `*mut T` 的 [`DenseDefault`] 逐位一致（null ⇒ 地址 0）；位图判定
  /// 下该占位不参与命中比较，故 null 键同样可正常 insert/find/erase。
  fn dense_default() -> Self {
    Self::null()
  }
}

/// `*const ()` 极性的 seen-set 键别名：载荷地址化后与 [`VisitKey`] 完全同型，
/// 保留具名是为了不改动既有调用点的可读性（详见模块头）。
pub type VisitKeyRef = VisitKey;
