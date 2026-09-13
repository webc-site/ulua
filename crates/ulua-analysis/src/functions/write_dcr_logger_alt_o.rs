use crate::{
  functions::{
    to_pointer_id_dcr_logger::to_pointer_id,
    to_pointer_id_dcr_logger_alt_b::to_pointer_id_not_null_constraint,
  },
  records::{constraint_block::ConstraintBlock, json_emitter::JsonEmitter},
  type_aliases::constraint_block_target::ConstraintBlockTarget,
};

pub fn write_json_emitter_constraint_block(emitter: &mut JsonEmitter, block: &ConstraintBlock) {
  let mut o = emitter.write_object();
  o.write_pair("stringification", &block.stringification);

  let target = &block.target;

  let kind = match target {
    ConstraintBlockTarget::V0(_) => "type",
    ConstraintBlockTarget::V1(_) => "typePack",
    ConstraintBlockTarget::V2(_) => "constraint",
  };

  let ptr_id = match target {
    ConstraintBlockTarget::V0(ty) => to_pointer_id(*ty),
    ConstraintBlockTarget::V1(tp) => to_pointer_id(*tp),
    ConstraintBlockTarget::V2(c) => to_pointer_id_not_null_constraint(*c),
  };

  o.write_pair("id", &ptr_id);
  o.write_pair("kind", kind);

  o.finish();
}
