//! Node: TableType::Props
//! Source: `Analysis/include/Luau/Type.h:495` (hand-ported)

// C++: using Props = std::map<Name, Property>; — std::map chosen deliberately
// for deterministic iteration order, which BTreeMap preserves.
use alloc::collections::BTreeMap;

use crate::{records::property_type::Property, type_aliases::name_type::Name};
pub type Props = BTreeMap<Name, Property>;
