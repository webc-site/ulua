//! cpp `diagnoseMissingTableKey` 的单一实现：`TypeChecker.cpp` 与
//! `TypeChecker2.cpp` 各有一份同体逻辑（只读 `UnknownProperty`、只写
//! `TypeErrorData`，不触碰 solver 自身状态），故两个 solver 共用本函数。

use alloc::{collections::BTreeSet, string::String};

use crate::{
  functions::{
    follow_type::follow_type_id, get_table_type::get_table_type, get_type_alt_j::get_type_id,
  },
  records::{
    extern_type::ExternType, unknown_prop_but_found_like_prop::UnknownPropButFoundLikeProp,
    unknown_property::UnknownProperty,
  },
  type_aliases::{props_type::Props, type_error_data::TypeErrorData},
};

/// 收集与缺失属性名「仅大小写不同」的候选属性名（cpp `equalsLower`）；
/// 有候选时把 `data` 改写为 `UnknownPropButFoundLikeProp`。
pub(crate) fn diagnose_missing_table_key(utk: &UnknownProperty, data: &mut TypeErrorData) {
  let key = utk.key();
  let mut candidates: BTreeSet<String> = BTreeSet::new();

  let mut accumulate = |props: &Props| {
    candidates.extend(
      props
        .keys()
        .filter(|name| name.as_str() != key && name.as_bytes().eq_ignore_ascii_case(key.as_bytes()))
        .cloned(),
    );
  };

  if let Some(ttv) = get_table_type(utk.table()) {
    accumulate(&ttv.props);
  } else {
    // extern 类型：沿父链逐级收集
    let mut current = get_type_id::<ExternType>(follow_type_id(utk.table()));
    while let Some(et) = current {
      accumulate(&et.props);
      current = et
        .parent
        .and_then(|parent| get_type_id::<ExternType>(follow_type_id(parent)));
    }
  }

  if candidates.is_empty() {
    return;
  }

  *data = TypeErrorData::UnknownPropButFoundLikeProp(UnknownPropButFoundLikeProp {
    table: utk.table(),
    key: key.to_string(),
    candidates,
  });
}
