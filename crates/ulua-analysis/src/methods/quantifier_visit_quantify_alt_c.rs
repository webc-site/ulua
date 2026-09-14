use core::ptr::null_mut;

use crate::{
  enums::polarity::Polarity,
  functions::as_mutable_type_pack::as_mutable_type_pack_id,
  records::{
    free_type_pack::FreeTypePack, generic_type_pack::GenericTypePack, quantifier::Quantifier,
    type_pack_var::TypePackVar,
  },
  type_aliases::type_pack_id::TypePackId,
};
impl Quantifier {
  pub fn visit_type_pack_id_free_type_pack(&mut self, tp: TypePackId, ftp: &FreeTypePack) -> bool {
    self.seen_mutable_type = true;

    if !self.level.subsumes(&ftp.level) {
      return false;
    }

    // *asMutable(tp) = GenericTypePack{level};
    let mut gtp = GenericTypePack {
      index: 0,
      level: Default::default(),
      scope: null_mut(),
      name: Default::default(),
      explicit_name: false,
      polarity: Polarity::None,
    };
    gtp.generic_type_pack_type_level(self.level);

    unsafe {
      *as_mutable_type_pack_id(tp) = TypePackVar::from(gtp);
    }

    self.generic_packs.push(tp);
    true
  }
}
