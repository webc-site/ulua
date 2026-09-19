use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{def::Def, symbol::Symbol};
pub type Bindings = DenseHashMap<Symbol, *const Def>;
