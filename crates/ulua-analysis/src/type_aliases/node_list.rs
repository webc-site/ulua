use alloc::collections::VecDeque;

use crate::records::node::NodeId;

/// Deferred-node queue `Q` in `toposort`: arena indices, not node pointers.
pub type NodeList = VecDeque<NodeId>;
