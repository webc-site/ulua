use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::relation::Relation,
  functions::{
    begin_type::begin_union_type, flip::flip, follow_type, get_type,
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

  let left = follow_type::follow(left);
  let right = follow_type::follow(right);

  if left == right {
    return Relation::Coincident;
  }

  let type_pair = (left, right);
  if !seen.try_insert(type_pair, true).1 {
    // TODO: is this right at all?
    // The thinking here is that this is a cycle if we get here, and therefore its coincident.
    return Relation::Coincident;
  }

  if get_type::get::<UnknownType>(left).is_some() {
    if get_type::get::<AnyType>(right).is_some() {
      return Relation::Subset;
    }

    if get_type::get::<UnknownType>(right).is_some() {
      return Relation::Coincident;
    }

    if get_type::get::<ErrorType>(right).is_some() {
      return Relation::Disjoint;
    }

    return Relation::Superset;
  }

  if get_type::get::<UnknownType>(right).is_some() {
    return flip(relate(right, left, seen));
  }

  if get_type::get::<AnyType>(left).is_some() {
    if get_type::get::<AnyType>(right).is_some() {
      return Relation::Coincident;
    }

    return Relation::Superset;
  }

  if get_type::get::<AnyType>(right).is_some() {
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
  if get_type::get::<TypeFunctionInstanceType>(left).is_some()
    || get_type::get::<TypeFunctionInstanceType>(right).is_some()
  {
    return Relation::Intersects;
  }

  if get_type::get::<ErrorType>(left).is_some() {
    if get_type::get::<ErrorType>(right).is_some() {
      return Relation::Coincident;
    } else if get_type::get::<AnyType>(right).is_some() {
      return Relation::Subset;
    }

    return Relation::Disjoint;
  } else if get_type::get::<ErrorType>(right).is_some() {
    return flip(relate(right, left, seen));
  }

  if get_type::get::<NeverType>(left).is_some() {
    if get_type::get::<NeverType>(right).is_some() {
      return Relation::Coincident;
    }

    return Relation::Subset;
  } else if get_type::get::<NeverType>(right).is_some() {
    return flip(relate(right, left, seen));
  }

  if get_type::get::<IntersectionType>(left).is_some()
    || get_type::get::<IntersectionType>(right).is_some()
  {
    return Relation::Intersects;
  }

  if let Some(ut) = get_type::get::<UnionType>(left) {
    // cpp for(TypeId part : ut)：TypeIterator 遍历，防环并展平嵌套 union
    for part in begin_union_type(ut) {
      let r = relate(part, right, seen);
      if matches!(r, Relation::Superset | Relation::Coincident) {
        return Relation::Superset;
      }
    }
    return Relation::Intersects;
  } else if let Some(ut) = get_type::get::<UnionType>(right) {
    for part in begin_union_type(ut) {
      let r = relate(left, part, seen);
      if matches!(r, Relation::Subset | Relation::Coincident) {
        return Relation::Subset;
      }
    }
    return Relation::Intersects;
  }

  if let Some(rnt) = get_type::get::<NegationType>(right).as_ref() {
    let a = relate(left, rnt.ty, seen);
    match a {
      Relation::Coincident => {
        // number & ~number
        return Relation::Disjoint;
      }
      Relation::Disjoint => {
        if get_type::get::<NegationType>(left).is_some() {
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
  } else if get_type::get::<NegationType>(left).is_some() {
    return flip(relate(right, left, seen));
  }

  if let Some(lp) = get_type::get::<PrimitiveType>(left).as_ref() {
    if let Some(rp) = get_type::get::<PrimitiveType>(right).as_ref() {
      if lp.r#type == rp.r#type {
        return Relation::Coincident;
      }

      return Relation::Disjoint;
    }

    if let Some(rs) = get_type::get::<SingletonType>(right).as_ref() {
      if lp.r#type == PrimitiveType::STRING && rs.variant.get_if::<StringSingleton>().is_some() {
        return Relation::Superset;
      }

      if lp.r#type == PrimitiveType::BOOLEAN && rs.variant.get_if::<BooleanSingleton>().is_some() {
        return Relation::Superset;
      }

      return Relation::Disjoint;
    }

    if lp.r#type == PrimitiveType::FUNCTION {
      if get_type::get::<FunctionType>(right).is_some() {
        return Relation::Superset;
      }

      return Relation::Disjoint;
    }
    if lp.r#type == PrimitiveType::TABLE {
      if get_type::get::<TableType>(right).is_some() {
        return Relation::Superset;
      }

      return Relation::Disjoint;
    }

    if get_type::get::<FunctionType>(right).is_some()
      || get_type::get::<TableType>(right).is_some()
      || get_type::get::<MetatableType>(right).is_some()
      || get_type::get::<ExternType>(right).is_some()
    {
      return Relation::Disjoint;
    }
  }

  if let Some(ls) = get_type::get::<SingletonType>(left).as_ref() {
    if get_type::get::<FunctionType>(right).is_some()
      || get_type::get::<TableType>(right).is_some()
      || get_type::get::<MetatableType>(right).is_some()
      || get_type::get::<ExternType>(right).is_some()
    {
      return Relation::Disjoint;
    }

    if get_type::get::<PrimitiveType>(right).is_some() {
      return flip(relate(right, left, seen));
    }

    if let Some(rs) = get_type::get::<SingletonType>(right).as_ref() {
      if ls.variant == rs.variant {
        return Relation::Coincident;
      }

      return Relation::Disjoint;
    }
  }

  if get_type::get::<FunctionType>(left).is_some() {
    if let Some(rp) = get_type::get::<PrimitiveType>(right).as_ref() {
      if rp.r#type == PrimitiveType::FUNCTION {
        return Relation::Subset;
      }

      return Relation::Disjoint;
    }

    return Relation::Intersects;
  }

  if let Some(lt) = get_type::get::<TableType>(left).as_ref() {
    if let Some(rp) = get_type::get::<PrimitiveType>(right).as_ref() {
      if rp.r#type == PrimitiveType::TABLE {
        return Relation::Subset;
      }

      return Relation::Disjoint;
    }

    if let Some(rt) = get_type::get::<TableType>(right).as_ref() {
      return relate_tables(lt, rt, seen);
    }

    if let Some(re) = get_type::get::<ExternType>(right).as_ref() {
      return relate_table_to_extern_type(lt, re, seen);
    }

    // TODO metatables

    return Relation::Disjoint;
  }

  if let Some(ct) = get_type::get::<ExternType>(left).as_ref() {
    if get_type::get::<ExternType>(right).as_ref().is_some() {
      if is_subclass_type_id_type_id(left, right) {
        return Relation::Subset;
      }

      if is_subclass_type_id_type_id(right, left) {
        return Relation::Superset;
      }

      return Relation::Disjoint;
    }

    if let Some(tbl) = get_type::get::<TableType>(right).as_ref() {
      return flip(relate_table_to_extern_type(tbl, ct, seen));
    }

    return Relation::Disjoint;
  }

  Relation::Intersects
}

pub fn relate_type_id_type_id(left: TypeId, right: TypeId) -> Relation {
  let mut seen = DenseHashMap::default();
  relate(left, right, &mut seen)
}
