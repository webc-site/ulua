use crate::{
  functions::{follow_type, get_type},
  records::{negation_type::NegationType, type_simplifier::TypeSimplifier},
  type_aliases::type_id::TypeId,
};

impl TypeSimplifier {
  pub fn mk_negation(&self, ty: TypeId) -> TypeId {
    // 契约：self.builtin_types 在构造期接线为 `Handle<BuiltinTypes>`，指向比
    // self 长寿的内置类型表；仅重建共享借用读取其字段，不产生可变别名。
    let builtin_types = self.builtin_types.get();
    // 契约：self.arena 为构造期接线的 `Handle<TypeArena>`，指向比 self 长寿且本线程独占
    // 的类型 arena（bump arena 地址稳定）；&self 仅借用 simplifier 字段、不覆盖 arena，
    // 故重建 &mut 与上面的 builtin_types 借用指向不同对象，无别名冲突。
    let arena = self.arena.get_mut();
    if ty == builtin_types.truthy_type {
      builtin_types.falsy_type
    } else if ty == builtin_types.falsy_type {
      builtin_types.truthy_type
    } else if let Some(ntv) = get_type::get::<NegationType>(ty) {
      follow_type::follow(ntv.ty)
    } else {
      arena.add_type(NegationType { ty })
    }
  }
}
