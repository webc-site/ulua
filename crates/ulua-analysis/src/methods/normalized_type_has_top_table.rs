use crate::{
  functions::get_type,
  records::{normalized_type::NormalizedType, primitive_type::PrimitiveType},
};
impl NormalizedType {
  pub fn has_top_table(&self) -> bool {
    if !self.has_tables() {
      return false;
    }

    for &ty in &self.tables.order {
      if let Some(prim_ref) = get_type::get::<PrimitiveType>(ty)
        && prim_ref.r#type == PrimitiveType::TABLE
      {
        return true;
      }
    }

    false
  }
}
