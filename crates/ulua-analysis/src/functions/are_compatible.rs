// Two tables may be compatible even if their shapes aren't exactly the
// same if the extra property is optional, free (and therefore
// potentially optional), or if the right table has an indexer.  Or if
// the right table is free (and therefore potentially has an indexer or
// a compatible property)
use crate::{
  enums::table_state::TableState,
  functions::{follow_type, get_2::get2, is_optional_or_free::is_optional_or_free},
  records::{property_type::Property, table_type::TableType},
  type_aliases::type_id::TypeId,
};
/// 对应 C++ lambda `missingPropIsCompatible`（Unifier2.cpp:49-68）：右表为 free
/// 或带 indexer 时缺省属性视为兼容；只读/共享属性的 `read_ty` 为 optional 或
/// free 亦兼容。`read_ty` 的 `expect` 由前置 `is_read_only()`/`is_shared()` 判据
/// 蕴含 `read_ty.is_some()`（构造不变式）保证。
fn missing_prop_is_compatible(left_prop: &Property, right_table: &TableType) -> bool {
  if right_table.state == TableState::Free || right_table.indexer.is_some() {
    return true;
  }

  if (left_prop.is_read_only() || left_prop.is_shared())
    && is_optional_or_free(
      left_prop
        .read_ty
        .expect("is_read_only/is_shared 构造不变式蕴含 read_ty 为 Some"),
    )
  {
    return true;
  }

  // FIXME: Could this create an issue for write only / divergent properties?
  false
}

/// 对应 C++ `areCompatible`（Unifier2.cpp:38-93）：两表 shape 不同仍可能兼容——
/// 多余属性为 optional/free、或右表带 indexer/为 free。任一侧 follow 后不是
/// TableType 直接返回 true（cpp `TryPair` 空 `.first` 同语义）。
pub fn are_compatible(left: TypeId, right: TypeId) -> bool {
  // 任一侧不是 TableType 即视为兼容（C++ 同语义：pair.first 为空直接返回 true）。
  let Some((left_table, right_table)) =
    get2::<TableType, TableType, TypeId>(follow_type::follow(left), follow_type::follow(right))
  else {
    return true;
  };

  for (_name, left_prop) in left_table.props.iter() {
    let it = right_table.props.get(_name);
    if it.is_none() && !missing_prop_is_compatible(left_prop, right_table) {
      return false;
    }
  }

  for (_name, right_prop) in right_table.props.iter() {
    let it = left_table.props.get(_name);
    if it.is_none() && !missing_prop_is_compatible(right_prop, left_table) {
      return false;
    }
  }

  true
}
