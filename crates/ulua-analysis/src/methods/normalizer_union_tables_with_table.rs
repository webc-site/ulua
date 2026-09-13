use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{never_type::NeverType, normalizer::Normalizer, type_ids::TypeIds},
  type_aliases::type_id::TypeId,
};

impl Normalizer {
  pub fn union_tables_with_table(&mut self, heres: &mut TypeIds, there: TypeId) {
    // we can always skip `never`
    let never_ptr = get_type_id::<NeverType>(there);
    if !never_ptr.is_none() {
      return;
    }

    heres.insert_type_id(there);
  }
}
