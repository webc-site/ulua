use crate::{
  functions::{follow_type::follow_type_id, follow_type_pack::follow_type_pack_id},
  records::replacer::Replacer,
};

impl Replacer {
  pub fn check_replacement_keys(&self) -> bool {
    let replacements = unsafe { &*self.replacements };
    for (k, _) in replacements.iter() {
      let followed = follow_type_id(*k);
      if *k != followed {
        return false;
      }
    }

    let replacement_packs = unsafe { &*self.replacement_packs };
    for (k, _) in replacement_packs.iter() {
      let followed = unsafe { follow_type_pack_id(*k) };
      if *k != followed {
        return false;
      }
    }

    true
  }
}
