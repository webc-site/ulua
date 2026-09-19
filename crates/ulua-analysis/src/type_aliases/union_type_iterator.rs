//! Source: `Analysis/include/Luau/Type.h`

use crate::records::{type_iterator::TypeIterator, union_type::UnionType};

pub type UnionTypeIterator = TypeIterator<UnionType>;
