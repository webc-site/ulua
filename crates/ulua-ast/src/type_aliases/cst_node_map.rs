use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{ast_node::AstNode, cst_node::CstNode};

pub type CstNodeMap = DenseHashMap<*mut AstNode, *mut CstNode>;
