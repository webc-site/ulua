//! Node of the toposort dependency graph, addressed by index into the arena
//! (`Vec<Node>`) rather than by raw pointer. See [`NodeId`].

use alloc::collections::BTreeSet;

use ulua_ast::records::{ast_stat::AstStat, node_handle::Node as StatHandle};

use crate::records::identifier::Identifier;

/// A node handle: index into the `Vec<Node>` arena owned by [`toposort`].
///
/// Replaces the C++ `Node*` used as graph identity. `usize` gives a stable,
/// ordered (so `BTreeSet`-compatible) handle without leaking raw pointers into
/// the graph logic.
///
/// [`toposort`]: crate::functions::toposort::toposort
pub type NodeId = usize;

#[derive(Debug, Clone)]
pub struct Node {
  // Adjacency, by arena index. C++ used `std::set<Node*>`; ordering by index is
  // the deterministic analogue of ordering by address.
  pub(crate) provides: BTreeSet<NodeId>,
  pub(crate) depends: BTreeSet<NodeId>,

  pub(crate) name: Option<Identifier>,
  // 语句本体在 AST arena 中,图逻辑只把它当作不透明身份在节点间搬运——载荷由
  // 早先的 `*mut AstStat` 收敛为 arena 句柄 `node_handle::Node<AstStat>`:
  // 句柄 `#[repr(transparent)]` 编码恒非空,`Eq`/`Hash` 按指针地址判等,与原
  // 裸指针的 cpp `Node* == Node*` 身份语义逐位同构,排序与去重行为不变;读经
  // `get`/`get_mut` 安全借用,解引用 unsafe 收口在句柄模块一处,不外渗到本图。
  pub(crate) element: StatHandle<AstStat>,
}

impl Node {
  pub fn new(name: Option<Identifier>, el: StatHandle<AstStat>) -> Self {
    Self {
      provides: BTreeSet::new(),
      depends: BTreeSet::new(),
      name,
      element: el,
    }
  }
}
