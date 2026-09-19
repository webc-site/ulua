use ulua_common::records::variant::Variant2;

use crate::{
  enums::{relation::Relation, table_state::TableState},
  functions::{
    follow_type::follow_type_id, get_type_alt_j::get_type_id,
    is_approximately_falsy_type::is_approximately_falsy_type,
    is_approximately_truthy_type::is_approximately_truthy_type,
    relate_simplify_alt_b::relate_type_id_type_id,
  },
  records::{
    any_type::AnyType, boolean_singleton::BooleanSingleton, error_type::ErrorType,
    negation_type::NegationType, never_type::NeverType, primitive_type::PrimitiveType,
    singleton_type::SingletonType, table_type::TableType, type_level::TypeLevel,
    type_simplifier::TypeSimplifier, union_type::UnionType, unknown_type::UnknownType,
  },
  type_aliases::type_id::TypeId,
};

impl TypeSimplifier {
  pub fn basic_intersect(&mut self, left: TypeId, right: TypeId) -> Option<TypeId> {
    let builtin_types = unsafe { &*self.builtin_types };
    let left = follow_type_id(left);
    let right = follow_type_id(right);

    if get_type_id::<AnyType>(left).is_some() && get_type_id::<ErrorType>(right).is_some() {
      return Some(right);
    }
    if get_type_id::<AnyType>(right).is_some() && get_type_id::<ErrorType>(left).is_some() {
      return Some(left);
    }
    if get_type_id::<AnyType>(left).is_some() {
      let arena = unsafe { &mut *self.arena.cast_mut() };
      return Some(arena.add_type(UnionType {
        options: alloc::vec![right, builtin_types.error_type],
      }));
    }
    if get_type_id::<AnyType>(right).is_some() {
      let arena = unsafe { &mut *self.arena.cast_mut() };
      return Some(arena.add_type(UnionType {
        options: alloc::vec![left, builtin_types.error_type],
      }));
    }
    if get_type_id::<UnknownType>(left).is_some() {
      return Some(right);
    }
    if get_type_id::<UnknownType>(right).is_some() {
      return Some(left);
    }
    if get_type_id::<NeverType>(left).is_some() {
      return Some(left);
    }
    if get_type_id::<NeverType>(right).is_some() {
      return Some(right);
    }

    if let Some(pt) = get_type_id::<PrimitiveType>(left) {
      if pt.r#type == PrimitiveType::BOOLEAN {
        if let Some(st) = get_type_id::<SingletonType>(right)
          && st.variant.get_if_0().is_some()
        {
          return Some(right);
        }
        if let Some(nt) = get_type_id::<NegationType>(right)
          && let Some(st) = get_type_id::<SingletonType>(follow_type_id(nt.ty))
          && st.variant.get_if_0().is_some()
        {
          if st.variant == Variant2::V0(BooleanSingleton::new(true)) {
            return Some(builtin_types.false_type);
          } else {
            return Some(builtin_types.true_type);
          }
        }
      }
    } else if let Some(pt) = get_type_id::<PrimitiveType>(right)
      && pt.r#type == PrimitiveType::BOOLEAN
    {
      if let Some(st) = get_type_id::<SingletonType>(left)
        && st.variant.get_if_0().is_some()
      {
        return Some(left);
      }
      if let Some(nt) = get_type_id::<NegationType>(left)
        && let Some(st) = get_type_id::<SingletonType>(follow_type_id(nt.ty))
        && st.variant.get_if_0().is_some()
      {
        if st.variant == Variant2::V0(BooleanSingleton::new(true)) {
          return Some(builtin_types.false_type);
        } else {
          return Some(builtin_types.true_type);
        }
      }
    }

    if let (Some(lt), Some(rt)) = (
      get_type_id::<TableType>(left),
      get_type_id::<TableType>(right),
    ) {
      if lt.props.len() == 1 {
        let (prop_name, left_prop) = lt.props.iter().next().unwrap();
        let left_prop_is_refinable = left_prop.is_shared() || left_prop.is_read_only();

        if let Some(right_prop) = rt.props.get(prop_name)
          && left_prop_is_refinable
          && right_prop.is_shared()
        {
          let relation = relate_type_id_type_id(
            left_prop.read_ty.expect("refinable property has read type"),
            right_prop.read_ty.expect("shared property has read type"),
          );

          match relation {
            Relation::Disjoint => return Some(builtin_types.never_type),
            Relation::Superset | Relation::Coincident => return Some(right),
            Relation::Subset => {
              if rt.props.len() == 1 && left_prop.is_shared() {
                return Some(left);
              }
            }
            Relation::Intersects => {}
          }
        }
      } else if rt.props.len() == 1 {
        return self.basic_intersect(right, left);
      }

      if lt.indexer.is_none()
        && rt.indexer.is_none()
        && lt.state == TableState::Sealed
        && rt.state == TableState::Sealed
      {
        if rt.props.is_empty() {
          return Some(left);
        }

        let are_disjoint = lt.props.keys().all(|name| !rt.props.contains_key(name));

        if are_disjoint {
          let mut merged = TableType::table_type_table_state_type_level_scope(
            TableState::Sealed,
            TypeLevel::default(),
            lt.scope,
          );
          merged.props = lt.props.clone();

          for (name, prop) in &rt.props {
            merged.props.insert(name.clone(), prop.clone());
          }

          let arena = unsafe { &mut *self.arena.cast_mut() };
          return Some(arena.add_type(merged));
        }
      }

      return None;
    }

    if is_approximately_truthy_type(left)
      && let Some(res) = self.basic_intersect_with_truthy(right)
    {
      return Some(res);
    }

    if is_approximately_truthy_type(right)
      && let Some(res) = self.basic_intersect_with_truthy(left)
    {
      return Some(res);
    }

    if is_approximately_falsy_type(left)
      && let Some(res) = self.basic_intersect_with_falsy(right)
    {
      return Some(res);
    }

    if is_approximately_falsy_type(right)
      && let Some(res) = self.basic_intersect_with_falsy(left)
    {
      return Some(res);
    }

    let relation = relate_type_id_type_id(left, right);
    if left == right || Relation::Coincident == relation {
      return Some(left);
    }
    match relation {
      Relation::Disjoint => Some(builtin_types.never_type),
      Relation::Subset => Some(left),
      Relation::Superset => Some(right),
      _ => None,
    }
  }
}
