use core::ptr::null_mut;

use crate::{
  enums::{relation::Relation, table_state::TableState},
  functions::{begin_type::begin_union_type, get_type, relate_simplify::relate_type_id_type_id},
  records::{
    never_type::NeverType, property_type::Property, recursion_limiter::RecursionLimiter,
    singleton_type::SingletonType, table_type::TableType, type_level::TypeLevel,
    type_simplifier::TypeSimplifier, union_builder::UnionBuilder, union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};
impl TypeSimplifier {
  pub fn union_(&mut self, left: TypeId, right: TypeId) -> TypeId {
    let _rl = RecursionLimiter::new("TypeSimplifier::union", &mut self.recursion_depth, 15);

    let left = self.simplify_type_id(left);
    let right = self.simplify_type_id(right);
    if get_type::get::<NeverType>(left).is_some() {
      return right;
    }
    if get_type::get::<NeverType>(right).is_some() {
      return left;
    }
    if let Some(left_union) = get_type::get::<UnionType>(left) {
      let mut changed = false;
      let mut ub = UnionBuilder::new(self.arena, self.builtin_types);
      ub.reserve(left_union.options.len());

      // C++ 的 `for (TypeId part : leftUnion)` 走防环且展平嵌套 union 的
      // `TypeIterator`，裸遍历 options 会在环状 union 上挂死。
      for part in begin_union_type(left_union) {
        if get_type::get::<NeverType>(part).is_some() {
          changed = true;
          continue;
        }

        match relate_type_id_type_id(part, right) {
          Relation::Coincident | Relation::Superset => return left,
          Relation::Subset => {
            ub.add(right);
            changed = true;
          }
          _ => {
            ub.add(part);
            ub.add(right);
            changed = true;
          }
        }
      }

      if !changed {
        return left;
      }

      // If the left-side is changed but has no parts, then the left-side union is uninhabited.
      if ub.size() == 0 {
        return right;
      }

      return ub.build();
    }
    if get_type::get::<UnionType>(right).is_some() {
      return self.union_(right, left);
    }

    let relation = relate_type_id_type_id(left, right);
    if left == right || matches!(relation, Relation::Coincident | Relation::Superset) {
      return left;
    }

    if relation == Relation::Subset {
      return right;
    }

    // 契约：`self.builtin_types` 对应 C++ `const BuiltinTypes&`，构造期由
    // `simplify_union` 以 Handle 接线（NotNull 语义），非空、不可变且比持有者长寿；
    // 此处取共享借用仅读 `boolean_type` 一个 TypeId 值。
    let builtin_types = self.builtin_types.get();
    if let Some(left_singleton) = get_type::get::<SingletonType>(left)
      && let Some(left_bool) = left_singleton.variant.get_if_0()
      && let Some(right_singleton) = get_type::get::<SingletonType>(right)
      && let Some(right_bool) = right_singleton.variant.get_if_0()
      && left_bool.value != right_bool.value
    {
      return builtin_types.boolean_type;
    }

    if let (Some(left_table), Some(right_table)) = (
      get_type::get::<TableType>(left),
      get_type::get::<TableType>(right),
    ) && left_table.props.len() == 1
      && right_table.props.len() == 1
    {
      // 上方 `props.len() == 1` 守卫蕴含 next() 命中 Some。
      let (prop_name, left_prop) = left_table
        .props
        .iter()
        .next()
        .expect("上方 left_table.props.len() == 1 蕴含首元素存在");
      let (right_prop_name, right_prop) = right_table
        .props
        .iter()
        .next()
        .expect("上方 right_table.props.len() == 1 蕴含首元素存在");

      // 契约：`self.arena` 为 Handle 接线的会话 TypeArena（C++ `NotNull<TypeArena>`），
      // bump arena 块地址在遍历期不移动；本次 simplify 调用单线程独占 arena，
      // get_mut 物化的可变借用无并发别名，仅用于 `add_type` 追加新 union 节点。
      let arena = self.arena.get_mut();

      if right_prop_name != prop_name {
        return arena.add_type(UnionType {
          options: alloc::vec![left, right],
        });
      }

      // Consider:
      //
      //  { prop: number? } | { prop: string? }
      //
      // Even though these two tables share a property, we cannot
      // simplify this type any further, otherwise we can, say,
      // launder a `{ prop: number? }` into a `{ prop: string? }`
      // and then write a string to it.
      //
      // We also elect to not simplify unsealed tables.
      if !left_prop.is_read_only()
        || !right_prop.is_read_only()
        || left_table.state != TableState::Sealed
        || right_table.state != TableState::Sealed
      {
        return arena.add_type(UnionType {
          options: alloc::vec![left, right],
        });
      }

      let left_read_ty = left_prop.read_ty.expect("read-only property has read type");
      let right_read_ty = right_prop
        .read_ty
        .expect("read-only property has read type");
      match relate_type_id_type_id(left_read_ty, right_read_ty) {
        Relation::Coincident | Relation::Superset => return left,
        Relation::Subset => return right,
        Relation::Disjoint | Relation::Intersects => {
          let prop_ty = self.union_(left_read_ty, right_read_ty);
          let mut result = TableType::table_type_table_state_type_level_scope(
            TableState::Sealed,
            TypeLevel::default(),
            null_mut(),
          );
          result
            .props
            .insert(prop_name.clone(), Property::readonly(prop_ty));
          return arena.add_type(result);
        }
      }
    }

    // 契约：同前——Handle 契约保证目标非空、块地址稳定、单线程独占，get_mut
    // 物化的可变借用与任何只读借用不重叠，仅向 arena 追加新 UnionType 节点。
    let arena = self.arena.get_mut();
    arena.add_type(UnionType {
      options: alloc::vec![left, right],
    })
  }
}
