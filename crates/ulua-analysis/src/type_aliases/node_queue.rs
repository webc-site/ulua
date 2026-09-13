use ulua_common::records::vec_deque::VecDeque;

use crate::records::node::Node;

pub type NodeQueue = VecDeque<*mut Node>;
