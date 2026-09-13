use alloc::vec::Vec;

use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{type_checker::TypeChecker, union_type::UnionType},
  type_aliases::type_id::TypeId,
};
impl TypeChecker {
  pub fn try_strip_union_from_nil(&mut self, ty: TypeId) -> Option<TypeId> {
    let utv = get_type_id::<UnionType>(ty)?;

    let mut has_nil = false;
    let mut result: Vec<TypeId> = Vec::new();

    for &option in utv.options.iter() {
      if option == self.nil_type {
        has_nil = true;
        continue;
      }
      result.push(option);
    }

    if !has_nil {
      return Some(ty);
    }

    if result.is_empty() {
      return None;
    }

    if result.len() == 1 {
      return Some(result[0]);
    }

    Some(self.add_type(&UnionType { options: result }))
  }
}
