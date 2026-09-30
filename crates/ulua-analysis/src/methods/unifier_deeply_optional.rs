use core::ptr::null;

use crate::{
  functions::{follow_type, get_mutable_type, get_type, is_optional::is_optional},
  records::{table_type::TableType, unifier::Unifier, union_type::UnionType},
  type_aliases::{collections::HashMap, type_id::TypeId},
};
impl Unifier {
  pub fn unifier_deeply_optional(
    &mut self,
    mut ty: TypeId,
    seen: &mut HashMap<TypeId, TypeId>,
  ) -> TypeId {
    ty = follow_type::follow(ty);

    if is_optional(ty) {
      return ty;
    }

    if let Some(ttv) = get_type::get::<TableType>(ty) {
      if let Some(&result) = seen.get(&ty) {
        return result;
      }

      // Add the table type to the arena first to get a result TypeId
      let result = self.types_mut().add_type(ttv.clone());

      // Store the result in seen map to handle cycles
      seen.insert(ty, result);

      // 刚以 `add_type(ttv.clone())` 把 TableType 变体登记进 arena，result 即
      // 该节点句柄，按 TableType 下转必命中 Some。
      let result_ttv = get_mutable_type::get_mutable::<TableType>(result)
        .expect("result 是刚 add_type 的 TableType 克隆，下转必命中");

      // Recursively make each property deeply optional
      for prop in result_ttv.props.values_mut() {
        let prop_ty = prop.read_ty.or(prop.write_ty).unwrap_or(null());
        let new_prop_ty = self.unifier_deeply_optional(prop_ty, seen);
        prop.read_ty = Some(new_prop_ty);
        prop.write_ty = Some(new_prop_ty);
      }

      // Return nil | result
      let builtin_types = self.builtin_types_ref();
      let union_types = alloc::vec![builtin_types.nil_type, result];
      let union_type = UnionType {
        options: union_types,
      };
      self.types_mut().add_type(union_type)
    } else {
      // Return nil | ty
      let builtin_types = self.builtin_types_ref();
      let union_types = alloc::vec![builtin_types.nil_type, ty];
      let union_type = UnionType {
        options: union_types,
      };
      self.types_mut().add_type(union_type)
    }
  }
}
