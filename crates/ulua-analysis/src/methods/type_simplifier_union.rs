use core::{mem::zeroed, ptr::null_mut};

use crate::{
  enums::{relation::Relation, table_state::TableState},
  functions::{
    begin_type::begin_union_type, end_type::end_union_type, get_type_alt_j::get_type_id,
    relate_simplify_alt_b::relate_type_id_type_id,
  },
  records::{
    never_type::NeverType, property_type::Property, recursion_limiter::RecursionLimiter,
    singleton_type::SingletonType, table_type::TableType, type_level::TypeLevel,
    type_simplifier::TypeSimplifier, union_builder::UnionBuilder, union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};
impl TypeSimplifier {
  pub fn union_(&mut self, left: TypeId, right: TypeId) -> TypeId {
    let mut rl = RecursionLimiter {
      base: unsafe { zeroed() },
      native_stack_guard: unsafe { zeroed() },
    };
    rl.recursion_limiter_recursion_limiter("TypeSimplifier::union", &mut self.recursion_depth, 15);

    let left = self.simplify_type_id(left);
    let right = self.simplify_type_id(right);
    if get_type_id::<NeverType>(left).is_some() {
      return right;
    }
    if get_type_id::<NeverType>(right).is_some() {
      return left;
    }
    if let Some(left_union) = get_type_id::<UnionType>(left) {
      let mut changed = false;
      let mut ub = UnionBuilder::new(self.arena.cast_mut(), self.builtin_types as *mut _);
      ub.reserve(left_union.options.len());

      let mut iter = begin_union_type(left_union);
      let end = end_union_type(left_union);
      while iter.operator_ne(&end) {
        let part = iter.operator_deref();

        if get_type_id::<NeverType>(part).is_some() {
          changed = true;
          iter.operator_inc();
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

        iter.operator_inc();
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
    if get_type_id::<UnionType>(right).is_some() {
      return self.union_(right, left);
    }

    let relation = relate_type_id_type_id(left, right);
    if left == right || relation == Relation::Coincident || relation == Relation::Superset {
      return left;
    }

    if relation == Relation::Subset {
      return right;
    }

    let builtin_types = unsafe { &*self.builtin_types };
    if let Some(left_singleton) = get_type_id::<SingletonType>(left)
      && let Some(left_bool) = left_singleton.variant.get_if_0()
      && let Some(right_singleton) = get_type_id::<SingletonType>(right)
      && let Some(right_bool) = right_singleton.variant.get_if_0()
      && left_bool.value != right_bool.value
    {
      return builtin_types.boolean_type;
    }

    if let (Some(left_table), Some(right_table)) = (
      get_type_id::<TableType>(left),
      get_type_id::<TableType>(right),
    ) && left_table.props.len() == 1
      && right_table.props.len() == 1
    {
      let (prop_name, left_prop) = left_table.props.iter().next().unwrap();
      let (right_prop_name, right_prop) = right_table.props.iter().next().unwrap();

      let arena = unsafe { &mut *self.arena.cast_mut() };

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

    let arena = unsafe { &mut *self.arena.cast_mut() };
    arena.add_type(UnionType {
      options: alloc::vec![left, right],
    })
  }
}
