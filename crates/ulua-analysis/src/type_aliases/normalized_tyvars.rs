use alloc::{boxed::Box, collections::BTreeMap};

use crate::{records::normalized_type::NormalizedType, type_aliases::type_id::TypeId};

pub type NormalizedTyvars = BTreeMap<TypeId, Box<NormalizedType>>;
