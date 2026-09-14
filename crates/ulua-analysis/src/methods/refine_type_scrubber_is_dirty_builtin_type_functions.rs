use crate::{
  records::refine_type_scrubber::RefineTypeScrubber, type_aliases::type_pack_id::TypePackId,
};

impl RefineTypeScrubber {
  pub fn is_dirty_type_pack_id(&mut self, _tp: TypePackId) -> bool {
    false
  }
}
