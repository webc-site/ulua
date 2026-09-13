use ulua_common::records::small_vector::SmallVector;

use crate::records::bc_block_edge::BcBlockEdge;

pub type BcEdges = SmallVector<BcBlockEdge, 2>;
