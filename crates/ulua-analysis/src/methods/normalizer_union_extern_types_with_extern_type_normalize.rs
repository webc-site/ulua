use crate::{
  functions::{get_type_alt_j::get_type_id, is_subclass_type::is_subclass_extern_type_extern_type},
  records::{extern_type::ExternType, normalizer::Normalizer, type_ids::TypeIds},
  type_aliases::type_id::TypeId,
};

impl Normalizer {
  pub fn union_extern_types_with_extern_type_type_ids_type_id(
    &mut self,
    heres: &mut TypeIds,
    there: TypeId,
  ) {
    self.consume_fuel();

    if heres.count(there) > 0 {
      return;
    }

    let tctv = get_type_id::<ExternType>(there).unwrap();

    let mut i = 0;
    while i < heres.order.len() {
      let here = heres.order[i];
      let hctv = get_type_id::<ExternType>(here).unwrap();

      if is_subclass_extern_type_extern_type(tctv, hctv) {
        return;
      } else if is_subclass_extern_type_extern_type(hctv, tctv) {
        heres.erase_type_id(here);
        // Do not advance `i` because elements shifted left.
      } else {
        i += 1;
      }
    }

    heres.insert_type_id(there);
  }
}
