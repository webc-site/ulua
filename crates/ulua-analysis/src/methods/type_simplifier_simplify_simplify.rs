use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::{follow_type, get_type},
  records::{
    any_type::AnyType, negation_type::NegationType, never_type::NeverType,
    recursion_limiter::RecursionLimiter, table_type::TableType, type_simplifier::TypeSimplifier,
    union_type::UnionType, unknown_type::UnknownType,
  },
  type_aliases::type_id::TypeId,
};

impl TypeSimplifier {
  pub fn simplify_type_id(&mut self, ty: TypeId) -> TypeId {
    let mut seen: DenseHashSet<TypeId> = DenseHashSet::default();
    self.simplify_type_id_dense_hash_set_type_id(ty, &mut seen)
  }

  pub fn simplify_type_id_dense_hash_set_type_id(
    &mut self,
    ty: TypeId,
    seen: &mut DenseHashSet<TypeId>,
  ) -> TypeId {
    let _rl = RecursionLimiter::new("TypeSimplifier::simplify", &mut self.recursion_depth, 60);

    let ty = follow_type::follow(ty);
    if seen.contains(&ty) {
      return ty;
    }
    seen.insert(ty);

    if let Some(nt) = get_type::get::<NegationType>(ty) {
      let negated_ty = follow_type::follow(nt.ty);
      // 契约：self.builtin_types 为构造期以 Handle 接线的 C++ NotNull<BuiltinTypes>
      // 会话单例，非空、指向比本 simplifier 长寿的只读内建类型表；借用只读取
      // never/error/unknown 等 Copy 句柄字段。
      let bt = self.builtin_types.get();
      if get_type::get::<AnyType>(negated_ty).is_some() {
        // 契约：self.arena 为 Handle 接线的会话 TypeArena；本分支 add_type 后即
        // return，可变借用不跨越任何可能重入 arena 的调用，bump arena 只追加不
        // 移动，单线程串行下此刻无并存借用。
        let arena = self.arena.get_mut();
        return arena.add_type(UnionType {
          options: alloc::vec![bt.never_type, bt.error_type],
        });
      } else if get_type::get::<UnknownType>(negated_ty).is_some() {
        return bt.never_type;
      } else if get_type::get::<NeverType>(negated_ty).is_some() {
        return bt.unknown_type;
      }
      if let Some(nnt) = get_type::get::<NegationType>(negated_ty) {
        return self.simplify_type_id_dense_hash_set_type_id(nnt.ty, seen);
      }
    }

    if let Some(tt) = get_type::get::<TableType>(ty)
      && tt.props.len() == 1
      // 链上 `props.len() == 1` 蕴含 values().next() 命中 Some。
      && let Some(read_ty) = tt
        .props
        .values()
        .next()
        .expect("上方 tt.props.len() == 1 蕴含唯一属性存在")
        .read_ty
    {
      let prop_ty = self.simplify_type_id_dense_hash_set_type_id(read_ty, seen);
      if get_type::get::<NeverType>(prop_ty).is_some() {
        // 契约：同一 builtin_types 句柄不变量（非空、只读长寿单例），此处
        // 又一次短共享借用，仅读取 never_type 句柄。
        let bt = self.builtin_types.get();
        return bt.never_type;
      }
    }
    ty
  }
}
