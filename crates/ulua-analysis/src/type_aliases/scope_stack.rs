use alloc::vec::Vec;

use crate::records::dfg_scope::DfgScope;
pub type ScopeStack = Vec<*mut DfgScope>;
