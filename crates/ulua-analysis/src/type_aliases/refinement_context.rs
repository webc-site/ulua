use ulua_common::records::insertion_ordered_map::InsertionOrderedMap;

use crate::{records::refinement_partition::RefinementPartition, type_aliases::def_id_def::DefId};

pub type RefinementContext = InsertionOrderedMap<DefId, RefinementPartition>;
