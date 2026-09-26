use crate::{
  functions::{follow_type, get_type},
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

    let ty_followed = follow_type::follow(ty);
    // 唯一调用方 `intersect_normal_with_ty` 先以 `get_type::get::<PrimitiveType>(follow(t)).is_some()`
    // 甄别分派才进入本函数（cpp 按类型 tag 分派后直接 get 的同前提），下转必命中。
    let ptv = get_type::get::<PrimitiveType>(ty_followed)
      .expect("调用方已按 PrimitiveType 甄别后分派，下转必命中");

    let builtin_types = here.builtin_types;
    // Safety: here.builtin_types 是 Normalizer/NormalizedType 构造期接线的非空
    // BuiltinTypes 会话单例（C++ NotNull<BuiltinTypes>），比本次归一化长寿且此
    // 后只读不变；借用提到函数头一次取得，替代分支内 6 处重复 `&*` 解引用。
    let builtin = builtin_types.get();
    match ptv.r#type {
      Type::NilType => {
        here.nils = builtin.never_type;
      }
      Type::Boolean => {
        here.booleans = builtin.never_type;
      }
      Type::Number => {
        here.numbers = builtin.never_type;
      }
      Type::Integer => {
        here.integers = builtin.never_type;
      }
      Type::String => {
        here.strings.reset_to_never();
      }
      Type::Thread => {
        here.threads = builtin.never_type;
      }
      Type::Buffer => {
        here.buffers = builtin.never_type;
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
