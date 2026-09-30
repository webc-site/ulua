use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::{
    follow_type, get_type, is_approximately_falsy_type::is_approximately_falsy_type,
    is_approximately_truthy_type::is_approximately_truthy_type,
  },
  records::{
    extern_type::ExternType, negation_type::NegationType, primitive_type::PrimitiveType,
    singleton_type::SingletonType, table_type::TableType,
  },
  type_aliases::type_id::TypeId,
};

pub fn is_simple_discriminant(ty: TypeId, seen: &mut DenseHashSet<TypeId>) -> bool {
  let ty = follow_type::follow(ty);
  if seen.contains(&ty) {
    return false;
  }
  seen.insert(ty);

  if let Some(ttv) = get_type::get::<TableType>(ty)
    && ttv.props.len() == 1
    && ttv.indexer.is_none()
  {
    // 链上 `props.len() == 1` 蕴含 values().next() 命中 Some。
    let prop = ttv
      .props
      .values()
      .next()
      .expect("上方 ttv.props.len() == 1 蕴含唯一属性存在");
    let read_ok = prop
      .read_ty
      .is_none_or(|rt| is_simple_discriminant(rt, seen));
    let write_ok = prop
      .write_ty
      .is_none_or(|wt| is_simple_discriminant(wt, seen));
    return read_ok && write_ok;
  }

  if let Some(nt) = get_type::get::<NegationType>(ty) {
    return is_simple_discriminant(nt.ty, seen);
  }

  get_type::get::<PrimitiveType>(ty).is_some()
    || get_type::get::<SingletonType>(ty).is_some()
    || get_type::get::<ExternType>(ty).is_some()
    || is_approximately_truthy_type(ty)
    || is_approximately_falsy_type(ty)
}

pub fn is_simple_discriminant_type_id(ty: TypeId) -> bool {
  let mut seen_set = DenseHashSet::default();
  is_simple_discriminant(ty, &mut seen_set)
}
