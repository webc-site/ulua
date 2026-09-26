use crate::{
  records::subtyping_environment::SubtypingEnvironment,
  type_aliases::{lookup_result::LookupResult, type_pack_id::TypePackId},
};

impl SubtypingEnvironment {
  pub fn contains_mapped_pack(&self, tp: TypePackId) -> bool {
    let lookup_result: LookupResult = self.lookup_generic_pack(tp);
    match lookup_result {
      LookupResult::V0(_) => true,
      _ => {
        if !self.parent.is_null() {
          unsafe { (*self.parent).contains_mapped_pack(tp) }
        } else {
          false
        }
      }
    }
  }
}
