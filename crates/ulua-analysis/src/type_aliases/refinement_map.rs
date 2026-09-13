//! Node: `cxx:TypeAlias:Luau.Analysis:Analysis/include/Luau/LValue.h:46:refinement_map`
//! Source: `Analysis/include/Luau/LValue.h` (LValue.h:46, hand-ported)

// C++: using RefinementMap = std::unordered_map<LValue, TypeId, LValueHasher>;
use std::collections::HashMap;

use crate::type_aliases::{l_value::LValue, type_id::TypeId};
pub type RefinementMap = HashMap<LValue, TypeId>;
