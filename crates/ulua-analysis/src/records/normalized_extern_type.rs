use alloc::{collections::BTreeMap, vec::Vec};

use crate::{records::type_ids::TypeIds, type_aliases::type_id::TypeId};

#[derive(Debug, Clone)]
pub struct NormalizedExternType {
  pub(crate) extern_types: BTreeMap<TypeId, TypeIds>,
  pub(crate) shape_extensions: TypeIds,
  pub(crate) ordering: Vec<TypeId>,
}
