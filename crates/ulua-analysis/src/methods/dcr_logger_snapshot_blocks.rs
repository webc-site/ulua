use alloc::vec::Vec;

use crate::{
  functions::{
    to_string_to_string_alt_m::to_string_type_id_to_string_options,
    to_string_to_string_alt_n::to_string_type_pack_id_to_string_options,
    to_string_to_string_alt_q::to_string_constraint_to_string_options,
  },
  records::{constraint::Constraint, constraint_block::ConstraintBlock, dcr_logger::DcrLogger},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl DcrLogger {
  /// `std::vector<ConstraintBlock> DcrLogger::snapshotBlocks(NotNull<const Constraint> c)`
  /// (`Analysis/src/DcrLogger.cpp:510-550`).
  pub fn snapshot_blocks(&self, c: *const Constraint) -> Vec<ConstraintBlock> {
    // The hash from `c` is independent of `opts`, so a `&self` shared borrow
    // suffices for the lookup; stringification clones `opts` internally.
    let mut opts = self.opts.clone();

    let it = match self.constraint_blocks.find(&c) {
      Some(list) => list,
      None => return Vec::new(),
    };

    let mut snapshot: Vec<ConstraintBlock> = Vec::new();

    for target in it.iter() {
      if let Some(ty) = target.get_if::<TypeId>() {
        snapshot.push(ConstraintBlock {
          target: target.clone(),
          stringification: to_string_type_id_to_string_options(*ty, &mut opts),
        });
      } else if let Some(tp) = target.get_if::<TypePackId>() {
        snapshot.push(ConstraintBlock {
          target: target.clone(),
          stringification: to_string_type_pack_id_to_string_options(*tp, &mut opts),
        });
      } else if let Some(c) = target.get_if::<*const Constraint>() {
        snapshot.push(ConstraintBlock {
          target: target.clone(),
          stringification: to_string_constraint_to_string_options(unsafe { &**c }, &mut opts),
        });
      } else {
        debug_assert!(false);
      }
    }

    snapshot
  }
}
