use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{never_type::NeverType, type_checker::TypeChecker},
  type_aliases::{type_id::TypeId, type_id_predicate::TypeIdPredicate},
};

impl TypeChecker {
  pub fn filter_map(
    &mut self,
    r#type: TypeId,
    predicate: TypeIdPredicate,
  ) -> (Option<TypeId>, bool) {
    let ty = self
      .filter_map_impl(r#type, predicate)
      .unwrap_or(self.never_type);
    // C++ 用 `get<NeverType>(ty)` 做类型判断（跟随别名），不能退化为指针比较。
    let ty_is_never = get_type_id::<NeverType>(ty).is_some();
    (Some(ty), !ty_is_never)
  }
}
