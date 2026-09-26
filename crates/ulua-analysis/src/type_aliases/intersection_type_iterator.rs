//! Source: `Analysis/include/Luau/Type.h`
use crate::records::{intersection_type::IntersectionType, type_iterator::TypeIterator};

pub type IntersectionTypeIterator = TypeIterator<IntersectionType>;
