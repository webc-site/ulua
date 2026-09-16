use alloc::{collections::BTreeSet, string::String};

use ulua_common::functions::equals_lower::equalsLower;

use crate::{
  functions::{
    follow_type::follow_type_id, get_table_type::get_table_type, get_type_alt_j::get_type_id,
  },
  records::{
    extern_type::ExternType, type_checker_2::TypeChecker2,
    unknown_prop_but_found_like_prop::UnknownPropButFoundLikeProp,
    unknown_property::UnknownProperty,
  },
  type_aliases::{props_type::Props, type_error_data::TypeErrorData},
};
impl TypeChecker2 {
  pub fn diagnose_missing_table_key(&self, utk: &UnknownProperty, data: &mut TypeErrorData) {
    let sv = utk.key();
    let mut candidates: BTreeSet<String> = BTreeSet::new();

    let mut accumulate = |props: &Props| {
      for name in props.keys() {
        if sv != name.as_str() && equalsLower(sv.as_bytes(), name.as_str().as_bytes()) {
          candidates.insert(name.clone());
        }
      }
    };

    if let Some(ttv) = get_table_type(utk.table()) {
      accumulate(&ttv.props);
    } else if let Some(etv) = get_type_id::<ExternType>(follow_type_id(utk.table())) {
      let mut current = Some(etv);
      while let Some(et) = current {
        accumulate(&et.props);

        let Some(parent) = et.parent else {
          break;
        };

        current = get_type_id::<ExternType>(follow_type_id(parent));
      }
    }

    if !candidates.is_empty() {
      *data = TypeErrorData::UnknownPropButFoundLikeProp(UnknownPropButFoundLikeProp {
        table: utk.table(),
        key: utk.key().to_string(),
        candidates,
      });
    }
  }
}
