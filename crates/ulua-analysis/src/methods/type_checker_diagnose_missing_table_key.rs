use alloc::collections::BTreeSet;

use ulua_common::functions::equals_lower::equalsLower;

use crate::{
  functions::{
    follow_type::follow_type_id, get_table_type::get_table_type, get_type_alt_j::get_type_id,
  },
  records::{
    extern_type::ExternType, type_checker::TypeChecker,
    unknown_prop_but_found_like_prop::UnknownPropButFoundLikeProp,
    unknown_property::UnknownProperty,
  },
  type_aliases::{props_type::Props, type_error_data::TypeErrorData},
};
impl TypeChecker {
  pub fn diagnose_missing_table_key(&mut self, utk: &UnknownProperty, data: &mut TypeErrorData) {
    let sv = utk.key();
    let mut candidates = BTreeSet::new();

    let mut accumulate = |props: &Props| {
      for name in props.keys() {
        if sv != name.as_str() && equalsLower(sv.as_bytes(), name.as_str().as_bytes()) {
          candidates.insert(name.clone());
        }
      }
    };

    if let Some(ttv) = get_table_type(utk.table()) {
      accumulate(&ttv.props);
    } else if let Some(first) = get_type_id::<ExternType>(follow_type_id(utk.table())) {
      // 沿 extern 父链收集候选名（C++ TypeChecker.cpp diagnoseMissingTableKey）
      let mut current = Some(first);
      while let Some(et) = current {
        accumulate(&et.props);
        current = match et.parent {
          Some(parent) => get_type_id::<ExternType>(follow_type_id(parent)),
          None => break,
        };
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
