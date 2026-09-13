use crate::{
  records::subtyping_environment::SubtypingEnvironment,
  type_aliases::{lookup_result::LookupResult, type_pack_id::TypePackId},
};

impl SubtypingEnvironment {
  pub fn lookup_generic_pack(&self, tp: TypePackId) -> LookupResult {
    let result = self.mapped_generic_packs.lookup_generic_pack(tp);
    if result.get_if::<TypePackId>().is_some() {
      result
    } else if !self.parent.is_null() {
      unsafe { (*self.parent).lookup_generic_pack(tp) }
    } else {
      result
    }
  }
}
