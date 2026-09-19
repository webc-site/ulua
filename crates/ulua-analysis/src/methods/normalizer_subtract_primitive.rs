use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    normalized_type::NormalizedType,
    normalizer::Normalizer,
    primitive_type::{PrimitiveType, Type},
  },
  type_aliases::type_id::TypeId,
};
impl Normalizer {
  pub fn subtract_primitive(&mut self, here: &mut NormalizedType, ty: TypeId) {
    self.consume_fuel();

    let ty_followed = follow_type_id(ty);
    let ptv = get_type_id::<PrimitiveType>(ty_followed).unwrap();

    let builtin_types = here.builtin_types;
    match ptv.r#type {
      Type::NilType => {
        here.nils = unsafe { (*builtin_types).never_type };
      }
      Type::Boolean => {
        here.booleans = unsafe { (*builtin_types).never_type };
      }
      Type::Number => {
        here.numbers = unsafe { (*builtin_types).never_type };
      }
      Type::Integer => {
        here.integers = unsafe { (*builtin_types).never_type };
      }
      Type::String => {
        here.strings.reset_to_never();
      }
      Type::Thread => {
        here.threads = unsafe { (*builtin_types).never_type };
      }
      Type::Buffer => {
        here.buffers = unsafe { (*builtin_types).never_type };
      }
      Type::Function => {
        here.functions.reset_to_never();
      }
      Type::Table => {
        here.tables.clear();
      }
    }
  }
}
