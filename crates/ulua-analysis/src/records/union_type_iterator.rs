//! Node: `cxx:TypeAlias:Luau.Analysis:Analysis/include/Luau/Type.h:1118:union_type_iterator`
//! Source: `Analysis/include/Luau/Type.h:1118-1120` (hand-ported)

use crate::records::type_iterator::TypeIterator;
use crate::records::union_type::UnionType;

pub type UnionTypeIterator = TypeIterator<UnionType>;
