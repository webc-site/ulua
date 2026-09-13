use crate::{
  records::refine_type_scrubber::RefineTypeScrubber, type_aliases::type_pack_id::TypePackId,
};

impl RefineTypeScrubber {
  pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    tp
  }
}
