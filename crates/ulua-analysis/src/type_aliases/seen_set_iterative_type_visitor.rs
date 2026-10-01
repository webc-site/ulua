use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::records::visit_key::VisitKeyRef;

pub type SeenSet = DenseHashSet<VisitKeyRef>;
