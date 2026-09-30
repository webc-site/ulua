use core::ptr::null_mut;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::{
    tarjan_node::TarjanNode, tarjan_worklist_vertex::TarjanWorklistVertex, txn_log::TxnLog,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
#[derive(Debug, Clone)]
pub struct Tarjan {
  pub(crate) type_to_index: DenseHashMap<TypeId, i32>,
  pub(crate) pack_to_index: DenseHashMap<TypePackId, i32>,
  pub(crate) nodes: Vec<TarjanNode>,
  pub(crate) stack: Vec<i32>,
  pub(crate) child_count: i32,
  pub(crate) child_limit: i32,
  pub(crate) log: *const TxnLog,
  pub(crate) edges_ty: Vec<TypeId>,
  pub(crate) edges_tp: Vec<TypePackId>,
  pub(crate) worklist: Vec<TarjanWorklistVertex>,
  /// Subclass virtual-override dispatch table (see [`SubstitutionVtable`]).
  pub(crate) vtable: SubstitutionVtable,
}

/// Virtual-dispatch table restoring the C++ `Tarjan`/`Substitution` override
/// semantics that plain Rust embedding loses.
///
/// In C++ `Tarjan` is an abstract base whose `isDirty` / `foundDirty` /
/// `ignoreChildren` / `ignoreChildrenVisit` (and `Substitution::clean`) are
/// (pure-)virtual and dispatched at runtime to the concrete subclass
/// (`Instantiation`, `Anyification`, `ApplyMappedGenerics`, ...). The Rust port
/// embeds `Tarjan` as the `base` of `Substitution`, which is in turn the `base`
/// of each subclass. The shared traversal (`loop`/`visitSCC`/`visitChildren`/
/// `substitute`) only ever holds a `&mut Tarjan` / `&mut Substitution`, so it
/// cannot reach the subclass overrides on its own. Each subclass installs its
/// override thunks (plus an `owner` pointer back to itself) into this table
/// immediately before invoking `substitute`; the traversal then dispatches
/// through the fn pointers.
///
/// The fn pointers are real, fully-typed `fn(...)` values — no transmute and no
/// fn-pointer erasure. Only the `owner` data pointer is type-erased to
/// `*mut ()`; each installed thunk was generated for one concrete subclass
/// and casts `owner` straight back to that type.
///
/// §4 处置：本表为「基类 → 宿主覆写」方向的伪 vtable，泛型单态化（11 宿主 ×
/// 共享遍历 ≈ 11 份代码爆炸，且回调与遍历的可变借用重叠不可安全表达）、
/// `dyn Trait`（同开销且丢失 `Copy`）、`enum_dispatch`（分派方向不适用）均已
/// 评估不可行，保留；完整论证见 [`crate::macros::substitution_vtable`] 模块文档。
#[derive(Debug, Clone, Copy)]
pub struct SubstitutionVtable {
  pub owner: *mut (),
  pub is_dirty_ty: Option<fn(*mut (), TypeId) -> bool>,
  pub is_dirty_tp: Option<fn(*mut (), TypePackId) -> bool>,
  pub clean_ty: Option<fn(*mut (), TypeId) -> TypeId>,
  pub clean_tp: Option<fn(*mut (), TypePackId) -> TypePackId>,
  pub found_dirty_ty: Option<fn(*mut (), TypeId)>,
  pub found_dirty_tp: Option<fn(*mut (), TypePackId)>,
  pub ignore_children_ty: Option<fn(*mut (), TypeId) -> bool>,
  pub ignore_children_tp: Option<fn(*mut (), TypePackId) -> bool>,
  pub ignore_children_visit_ty: Option<fn(*mut (), TypeId) -> bool>,
  pub ignore_children_visit_tp: Option<fn(*mut (), TypePackId) -> bool>,
}

impl SubstitutionVtable {
  /// The "no subclass installed yet" state: every override unset. `Tarjan` is
  /// abstract in C++ and is never traversed without a concrete subclass, so an
  /// uninstalled table only ever yields the base-class defaults (`false` for
  /// `ignoreChildren*`); reaching an unset `isDirty`/`clean`/`foundDirty`
  /// is a wiring bug and panics, mirroring a C++ pure-virtual call.
  pub const fn null() -> Self {
    SubstitutionVtable {
      // `owner` 是伪 vtable 的宿主基址（`self as *mut Host as *mut ()` 的类型擦除回指），
      // 不是「可选依赖」：所有 thunk 都以 `owner` 为第一参把宿主转铸回具体类型后解引用
      // （消费点在 `methods/substitution.rs`、`methods/tarjan.rs` 的
      // `let owner = self.vtable.owner;` 及其后的 `*mut Host` 回转）。`null()` 里的
      // `null_mut()` 对应 C++ 抽象基类未安装具体子类覆写时的空 vtable 基址——这是
      // 「尚未接线」的身份哨兵，`Option<NonNull<()>>` 无法表达其擦除语义且会强迫全部
      // 转铸调用点先判空再解引用（热路径 loop/visitSCC 每节点都读 owner），故按 §2(b)
      // 保留裸指针；真正的「未安装即取 isDirty/clean」属接线 bug，由 `Option<fn>` 臂
      // 的 panic 拦截，`owner` 本身从不被独立判空消费。
      owner: null_mut(),
      is_dirty_ty: None,
      is_dirty_tp: None,
      clean_ty: None,
      clean_tp: None,
      found_dirty_ty: None,
      found_dirty_tp: None,
      ignore_children_ty: None,
      ignore_children_tp: None,
      ignore_children_visit_ty: None,
      ignore_children_visit_tp: None,
    }
  }
}
