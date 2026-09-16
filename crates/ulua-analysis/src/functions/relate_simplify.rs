use crate::{
  enums::relation::Relation,
  functions::{
    flip::flip, follow_type::follow_type_id, get_type_alt_j::get_type_id,
    is_subclass_normalize::is_subclass_type_id_type_id, is_type_variable::is_type_variable,
    relate_table_to_extern_type::relate_table_to_extern_type, relate_tables::relate_tables,
  },
  records::{
    any_type::AnyType, boolean_singleton::BooleanSingleton, extern_type::ExternType,
    function_type::FunctionType, intersection_type::IntersectionType,
    metatable_type::MetatableType, negation_type::NegationType, never_type::NeverType,
    primitive_type::PrimitiveType, singleton_type::SingletonType,
    string_singleton::StringSingleton, table_type::TableType,
    type_function_instance_type::TypeFunctionInstanceType, union_type::UnionType,
    unknown_type::UnknownType,
  },
  type_aliases::{error_type::ErrorType, simplifier_seen_set::SimplifierSeenSet, type_id::TypeId},
};

// A cheap and approximate subtype test
pub fn relate(left: TypeId, right: TypeId, seen: &mut SimplifierSeenSet) -> Relation {
  // TODO nice to have: Relate functions of equal argument and return arity

  let left = follow_type_id(left);
  let right = follow_type_id(right);

  if left == right {
    return Relation::Coincident;
  }

  let type_pair = (left, right);
  if !seen.try_insert(type_pair, true).1 {
    // TODO: is this right at all?
    // The thinking here is that this is a cycle if we get here, and therefore its coincident.
    return Relation::Coincident;
  }

  if !get_type_id::<UnknownType>(left).is_none() {
    if !get_type_id::<AnyType>(right).is_none() {
      return Relation::Subset;
    }

    if !get_type_id::<UnknownType>(right).is_none() {
      return Relation::Coincident;
    }

    if !get_type_id::<ErrorType>(right).is_none() {
      return Relation::Disjoint;
    }

    return Relation::Superset;
  }

  if !get_type_id::<UnknownType>(right).is_none() {
    return flip(relate(right, left, seen));
  }

  if !get_type_id::<AnyType>(left).is_none() {
    if !get_type_id::<AnyType>(right).is_none() {
      return Relation::Coincident;
    }

    return Relation::Superset;
  }

  if !get_type_id::<AnyType>(right).is_none() {
    return flip(relate(right, left, seen));
  }

  // Type variables
  // * FreeType
  // * GenericType
  // * BlockedType
  // * PendingExpansionType

  // Tops and bottoms
  // * ErrorType
  // * AnyType
  // * NeverType
  // * UnknownType

  // Concrete
  // * PrimitiveType
  // * SingletonType
  // * FunctionType
  // * TableType
  // * MetatableType
  // * ExternType
  // * UnionType
  // * IntersectionType
  // * NegationType

  if is_type_variable(left) || is_type_variable(right) {
    return Relation::Intersects;
  }

  // if either type is a type function, we cannot know if they'll be related.
  if !get_type_id::<TypeFunctionInstanceType>(left).is_none()
    || !get_type_id::<TypeFunctionInstanceType>(right).is_none()
  {
    return Relation::Intersects;
  }

  if !get_type_id::<ErrorType>(left).is_none() {
    if !get_type_id::<ErrorType>(right).is_none() {
      return Relation::Coincident;
    } else if !get_type_id::<AnyType>(right).is_none() {
      return Relation::Subset;
    }

    return Relation::Disjoint;
  } else if !get_type_id::<ErrorType>(right).is_none() {
    return flip(relate(right, left, seen));
  }

  if !get_type_id::<NeverType>(left).is_none() {
    if !get_type_id::<NeverType>(right).is_none() {
      return Relation::Coincident;
    }

    return Relation::Subset;
  } else if !get_type_id::<NeverType>(right).is_none() {
    return flip(relate(right, left, seen));
  }

  if !get_type_id::<IntersectionType>(left).is_none()
    || !get_type_id::<IntersectionType>(right).is_none()
  {
    return Relation::Intersects;
  }

  if let Some(ut) = get_type_id::<UnionType>(left).as_ref() {
    for &part in &ut.options {
      let r = relate(part, right, seen);
      if r == Relation::Superset || r == Relation::Coincident {
        return Relation::Superset;
      }
    }
    return Relation::Intersects;
  } else if let Some(ut) = get_type_id::<UnionType>(right).as_ref() {
    for &part in &ut.options {
      let r = relate(left, part, seen);
      if r == Relation::Subset || r == Relation::Coincident {
        return Relation::Subset;
      }
    }
    return Relation::Intersects;
  }

  if let Some(rnt) = get_type_id::<NegationType>(right).as_ref() {
    let a = relate(left, rnt.ty, seen);
    match a {
      Relation::Coincident => {
        // number & ~number
        return Relation::Disjoint;
      }
      Relation::Disjoint => {
        if !get_type_id::<NegationType>(left).is_none() {
          // ~number & ~string
          return Relation::Intersects;
        } else {
          // number & ~string
          return Relation::Subset;
        }
      }
      Relation::Intersects => {
        // ~(false?) & ~boolean
        return Relation::Intersects;
      }
      Relation::Subset => {
        // "hello" & ~string
        return Relation::Disjoint;
      }
      Relation::Superset => {
        // ~function & ~(false?)  -> ~function
        // boolean & ~(false?)    -> true
        // string & ~"hello"      -> string & ~"hello"
        return Relation::Intersects;
      }
    }
  } else if !get_type_id::<NegationType>(left).is_none() {
    return flip(relate(right, left, seen));
  }

  if let Some(lp) = get_type_id::<PrimitiveType>(left).as_ref() {
    if let Some(rp) = get_type_id::<PrimitiveType>(right).as_ref() {
      if lp.r#type == rp.r#type {
        return Relation::Coincident;
      }

      return Relation::Disjoint;
    }

    if let Some(rs) = get_type_id::<SingletonType>(right).as_ref() {
      if lp.r#type == PrimitiveType::STRING && rs.variant.get_if::<StringSingleton>().is_some() {
        return Relation::Superset;
      }

      if lp.r#type == PrimitiveType::BOOLEAN && rs.variant.get_if::<BooleanSingleton>().is_some() {
        return Relation::Superset;
      }

      return Relation::Disjoint;
    }

    if lp.r#type == PrimitiveType::FUNCTION {
      if !get_type_id::<FunctionType>(right).is_none() {
        return Relation::Superset;
      }

      return Relation::Disjoint;
    }
    if lp.r#type == PrimitiveType::TABLE {
      if !get_type_id::<TableType>(right).is_none() {
        return Relation::Superset;
      }

      return Relation::Disjoint;
    }

    if !get_type_id::<FunctionType>(right).is_none()
      || !get_type_id::<TableType>(right).is_none()
      || !get_type_id::<MetatableType>(right).is_none()
      || !get_type_id::<ExternType>(right).is_none()
    {
      return Relation::Disjoint;
    }
  }

  if let Some(ls) = get_type_id::<SingletonType>(left).as_ref() {
    if !get_type_id::<FunctionType>(right).is_none()
      || !get_type_id::<TableType>(right).is_none()
      || !get_type_id::<MetatableType>(right).is_none()
      || !get_type_id::<ExternType>(right).is_none()
    {
      return Relation::Disjoint;
    }

    if !get_type_id::<PrimitiveType>(right).is_none() {
      return flip(relate(right, left, seen));
    }

    if let Some(rs) = get_type_id::<SingletonType>(right).as_ref() {
      if ls.variant == rs.variant {
        return Relation::Coincident;
      }

      return Relation::Disjoint;
    }
  }

  if !get_type_id::<FunctionType>(left).is_none() {
    if let Some(rp) = get_type_id::<PrimitiveType>(right).as_ref() {
      if rp.r#type == PrimitiveType::FUNCTION {
        return Relation::Subset;
      }

      return Relation::Disjoint;
    }

    return Relation::Intersects;
  }

  if let Some(lt) = get_type_id::<TableType>(left).as_ref() {
    if let Some(rp) = get_type_id::<PrimitiveType>(right).as_ref() {
      if rp.r#type == PrimitiveType::TABLE {
        return Relation::Subset;
      }

      return Relation::Disjoint;
    }

    if let Some(rt) = get_type_id::<TableType>(right).as_ref() {
      return relate_tables(lt, rt, seen);
    }

    if let Some(re) = get_type_id::<ExternType>(right).as_ref() {
      return relate_table_to_extern_type(lt, re, seen);
    }

    // TODO metatables

    return Relation::Disjoint;
  }

  if let Some(ct) = get_type_id::<ExternType>(left).as_ref() {
    if get_type_id::<ExternType>(right).as_ref().is_some() {
      if is_subclass_type_id_type_id(left, right) {
        return Relation::Subset;
      }

      if is_subclass_type_id_type_id(right, left) {
        return Relation::Superset;
      }

      return Relation::Disjoint;
    }

    if let Some(tbl) = get_type_id::<TableType>(right).as_ref() {
      return flip(relate_table_to_extern_type(tbl, ct, seen));
    }

    return Relation::Disjoint;
  }

  Relation::Intersects
}
