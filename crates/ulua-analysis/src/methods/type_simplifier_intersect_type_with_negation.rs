use alloc::collections::BTreeSet;

use ulua_common::records::variant::Variant2;

use crate::{
  enums::relation::Relation,
  functions::{
    follow_type::follow_type_id, get_type_alt_j::get_type_id,
    relate_simplify_alt_b::relate_type_id_type_id,
  },
  records::{
    boolean_singleton::BooleanSingleton, intersection_type::IntersectionType,
    negation_type::NegationType, primitive_type::PrimitiveType, singleton_type::SingletonType,
    type_ids::TypeIds, type_simplifier::TypeSimplifier, union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

impl TypeSimplifier {
  pub fn intersect_type_with_negation(&mut self, left: TypeId, right: TypeId) -> TypeId {
    let builtin_types = unsafe { &*self.builtin_types };

    // C++ Simplify.cpp intersectTypeWithNegation: LUAU_ASSERT(leftNegation)
    let left_negation = get_type_id::<NegationType>(left).expect("left is NegationType");

    let negated_ty = follow_type_id(left_negation.ty);

    if negated_ty == right {
      return builtin_types.never_type;
    }

    if let Some(ut) = get_type_id::<UnionType>(negated_ty) {
      // ~(A | B) & C
      // (~A & C) & (~B & C)
      let mut changed = false;
      let mut new_parts = TypeIds::new();

      for &part in &ut.options {
        let r = relate_type_id_type_id(part, right);
        match r {
          // ~(false?) & nil
          // (~false & nil) & (~nil & nil)
          // nil & never
          //
          // fallthrough
          Relation::Coincident => {
            // ~(boolean | string) & true
            // (~boolean & true) & (~boolean & string)
            // never & string
            return builtin_types.never_type;
          }
          Relation::Superset => {
            return builtin_types.never_type;
          }
          Relation::Disjoint => {
            // ~nil & boolean
            new_parts.insert_type_id(right);
          }
          // ~false & boolean
          // fallthrough
          Relation::Subset | Relation::Intersects => {
            // FIXME: The mkNegation here is pretty unfortunate.
            // Memoizing this will probably be important.
            changed = true;
            new_parts.insert_type_id(right);
            let negation = self.mk_negation(part);
            new_parts.insert_type_id(negation);
          }
        }
      }

      if !changed {
        return right;
      }

      return self.intersect_from_parts(new_parts);
    }

    if let Some(right_union) = get_type_id::<UnionType>(right) {
      // ~A & (B | C)
      let mut changed = false;
      let mut new_parts: BTreeSet<TypeId> = BTreeSet::new();

      for &part in &right_union.options {
        let r = relate_type_id_type_id(negated_ty, part);
        match r {
          Relation::Coincident => {
            changed = true;
            continue;
          }
          Relation::Disjoint => {
            new_parts.insert(part);
          }
          Relation::Superset => {
            changed = true;
            continue;
          }
          // fallthrough
          Relation::Subset | Relation::Intersects => {
            changed = true;
            let arena = unsafe { &mut *self.arena.cast_mut() };
            new_parts.insert(arena.add_type(IntersectionType {
              parts: alloc::vec![left, part],
            }));
          }
        }
      }

      if !changed {
        return right;
      } else if new_parts.is_empty() {
        return builtin_types.never_type;
      } else if new_parts.len() == 1 {
        return *new_parts.iter().next().unwrap();
      } else {
        let arena = unsafe { &mut *self.arena.cast_mut() };
        return arena.add_type(UnionType {
          options: new_parts.into_iter().collect(),
        });
      }
    }

    if let Some(pt) = get_type_id::<PrimitiveType>(right)
      && pt.r#type == PrimitiveType::BOOLEAN
      && let Some(st) = get_type_id::<SingletonType>(negated_ty)
    {
      if st.variant == Variant2::V0(BooleanSingleton::new(true)) {
        return builtin_types.false_type;
      } else if st.variant == Variant2::V0(BooleanSingleton::new(false)) {
        return builtin_types.true_type;
      } else {
        // boolean & ~"hello"
        return builtin_types.boolean_type;
      }
    }

    let r = relate_type_id_type_id(negated_ty, right);

    match r {
      Relation::Disjoint => {
        // ~boolean & string
        right
      }
      // ~string & string
      // fallthrough
      Relation::Coincident => {
        // ~string & "hello"
        builtin_types.never_type
      }
      Relation::Superset => builtin_types.never_type,
      // ~string & unknown
      // ~"hello" & string
      // fallthrough
      // ~("hello" | boolean) & string
      // fallthrough
      // default
      Relation::Subset | Relation::Intersects => {
        let arena = unsafe { &mut *self.arena.cast_mut() };
        arena.add_type(IntersectionType {
          parts: alloc::vec![left, right],
        })
      }
    }
  }
}
