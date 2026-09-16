use crate::{
  enums::table_state::TableState,
  functions::follow_type::follow_type_id,
  methods::type_cacher_visit_generalization_alt_i::cacher_traverse_type_id,
  records::{table_type::TableType, type_cacher::TypeCacher},
  type_aliases::type_id::TypeId,
};

impl TypeCacher {
  pub fn visit_type_id_table_type(&mut self, ty: TypeId, tt: &TableType) -> bool {
    if self.is_cached(ty) || self.is_uncacheable_type_id(ty) {
      return false;
    }

    if let Some(bound) = tt.bound_to {
      let followed = follow_type_id(bound);
      cacher_traverse_type_id(self, followed);
      if self.is_uncacheable_type_id(followed) {
        self.mark_uncacheable_type_id(ty);
        return false;
      }
    }

    let mut uncacheable = tt.state == TableState::Free || tt.state == TableState::Unsealed;

    for prop in tt.props.values() {
      if let Some(read) = prop.read_ty {
        let followed = follow_type_id(read);
        cacher_traverse_type_id(self, followed);
        if self.is_uncacheable_type_id(followed) {
          uncacheable = true;
        }
      }
      if let Some(write) = prop.write_ty
        && Some(write) != prop.read_ty
      {
        let followed = follow_type_id(write);
        cacher_traverse_type_id(self, followed);
        if self.is_uncacheable_type_id(followed) {
          uncacheable = true;
        }
      }
    }

    if let Some(indexer) = &tt.indexer {
      let idx = follow_type_id(indexer.index_type);
      let res = follow_type_id(indexer.index_result_type);
      cacher_traverse_type_id(self, idx);
      cacher_traverse_type_id(self, res);
      if self.is_uncacheable_type_id(idx) || self.is_uncacheable_type_id(res) {
        uncacheable = true;
      }
    }

    if uncacheable {
      self.mark_uncacheable_type_id(ty);
    } else {
      self.cache(ty);
    }

    false
  }
}
