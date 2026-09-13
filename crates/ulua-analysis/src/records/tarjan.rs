use core::{ffi::c_void, ptr::null_mut};

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
/// `*mut c_void`; each installed thunk was generated for one concrete subclass
/// and casts `owner` straight back to that type.
#[derive(Debug, Clone, Copy)]
pub struct SubstitutionVtable {
  pub owner: *mut c_void,
  pub is_dirty_ty: Option<fn(*mut c_void, TypeId) -> bool>,
  pub is_dirty_tp: Option<fn(*mut c_void, TypePackId) -> bool>,
  pub clean_ty: Option<fn(*mut c_void, TypeId) -> TypeId>,
  pub clean_tp: Option<fn(*mut c_void, TypePackId) -> TypePackId>,
  pub found_dirty_ty: Option<fn(*mut c_void, TypeId)>,
  pub found_dirty_tp: Option<fn(*mut c_void, TypePackId)>,
  pub ignore_children_ty: Option<fn(*mut c_void, TypeId) -> bool>,
  pub ignore_children_tp: Option<fn(*mut c_void, TypePackId) -> bool>,
  pub ignore_children_visit_ty: Option<fn(*mut c_void, TypeId) -> bool>,
  pub ignore_children_visit_tp: Option<fn(*mut c_void, TypePackId) -> bool>,
}

impl SubstitutionVtable {
  /// The "no subclass installed yet" state: every override unset. `Tarjan` is
  /// abstract in C++ and is never traversed without a concrete subclass, so an
  /// uninstalled table only ever yields the base-class defaults (`false` for
  /// `ignoreChildren*`); reaching an unset `isDirty`/`clean`/`foundDirty`
  /// is a wiring bug and panics, mirroring a C++ pure-virtual call.
  pub const fn null() -> Self {
    SubstitutionVtable {
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
