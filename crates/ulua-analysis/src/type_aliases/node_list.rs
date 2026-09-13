use alloc::collections::VecDeque;

use crate::records::node::Node;

/// `NodeList` is a type alias for a `VecDeque` of `Node` pointers.
/// In C++ it was `std::list<std::unique_ptr<Node>>`; we model it as
/// `VecDeque<*mut Node>` since `Node` is arena-allocated and owned
/// elsewhere. The `VecDeque` provides efficient push/pop from both ends.
pub type NodeList = VecDeque<*mut Node>;
