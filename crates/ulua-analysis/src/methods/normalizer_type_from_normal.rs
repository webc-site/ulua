use ulua_common::FFlag;

use crate::{
  functions::{
    add_intersection::add_intersection, assert_invariant::assert_invariant,
    get_type_alt_j::get_type_id, is_top::is_top,
  },
  records::{
    intersection_type::IntersectionType, negation_type::NegationType, never_type::NeverType,
    normalized_type::NormalizedType, normalizer::Normalizer, union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

impl Normalizer {
  pub fn type_from_normal(&mut self, norm: &NormalizedType) -> TypeId {
    assert_invariant(norm);

    if get_type_id::<NeverType>(norm.tops).is_none() {
      return norm.tops;
    }

    let mut result: Vec<TypeId> = Vec::new();

    if get_type_id::<NeverType>(norm.booleans).is_none() {
      result.push(norm.booleans);
    }

    if is_top(unsafe { &*self.builtin_types }, &norm.extern_types) {
      let builtin_types = unsafe { &*self.builtin_types };
      result.push(builtin_types.extern_type);
    } else if !norm.extern_types.is_never() {
      let mut parts: Vec<TypeId> = Vec::with_capacity(norm.extern_types.extern_types.len());

      for &norm_ty in norm.extern_types.ordering.iter() {
        let norm_negations = norm.extern_types.extern_types.get(&norm_ty).unwrap();

        if norm_negations.empty()
          && (!FFlag::LuauExternTypesNormalizeWithShapes.get()
            || norm.extern_types.shape_extensions.empty())
        {
          parts.push(norm_ty);
        } else {
          let mut intersection: Vec<TypeId> = Vec::with_capacity(norm_negations.size() + 1);

          intersection.push(norm_ty);
          for &negation in norm_negations.order.iter() {
            let negation_type = NegationType { ty: negation };
            intersection.push(unsafe { (*self.arena).add_type(negation_type) });
          }

          if FFlag::LuauExternTypesNormalizeWithShapes.get() {
            for &shape in norm.extern_types.shape_extensions.order.iter() {
              intersection.push(shape);
            }
          }

          parts.push(unsafe {
            (*self.arena).add_type(IntersectionType {
              parts: intersection,
            })
          });
        }
      }

      if parts.len() == 1 {
        result.push(parts[0]);
      } else if parts.len() > 1 {
        result.push(unsafe { (*self.arena).add_type(UnionType { options: parts }) });
      }
    }

    if get_type_id::<NeverType>(norm.errors).is_none() {
      result.push(norm.errors);
    }

    if norm.functions.is_top {
      let builtin_types = unsafe { &*self.builtin_types };
      result.push(builtin_types.function_type);
    } else if !norm.functions.parts.is_never() {
      if norm.functions.parts.order.len() == 1 {
        result.push(norm.functions.parts.order[0]);
      } else {
        let mut parts: Vec<TypeId> = Vec::new();
        parts.extend(norm.functions.parts.order.iter().copied());
        result.push(unsafe { (*self.arena).add_type(IntersectionType { parts }) });
      }
    }

    if get_type_id::<NeverType>(norm.nils).is_none() {
      result.push(norm.nils);
    }

    if get_type_id::<NeverType>(norm.numbers).is_none() {
      result.push(norm.numbers);
    }

    if FFlag::LuauIntegerType2.get() && get_type_id::<NeverType>(norm.integers).is_none() {
      result.push(norm.integers);
    }

    if norm.strings.is_string() {
      let builtin_types = unsafe { &*self.builtin_types };
      result.push(builtin_types.string_type);
    } else if norm.strings.is_union() {
      for ty in norm.strings.singletons.values() {
        result.push(*ty);
      }
    } else if norm.strings.is_intersection() {
      let mut parts: Vec<TypeId> = Vec::new();
      let builtin_types = unsafe { &*self.builtin_types };
      parts.push(builtin_types.string_type);

      for ty in norm.strings.singletons.values() {
        let negation_type = NegationType { ty: *ty };
        parts.push(unsafe { (*self.arena).add_type(negation_type) });
      }

      result.push(unsafe { (*self.arena).add_type(IntersectionType { parts }) });
    }

    if get_type_id::<NeverType>(norm.threads).is_none() {
      let builtin_types = unsafe { &*self.builtin_types };
      result.push(builtin_types.thread_type);
    }

    if get_type_id::<NeverType>(norm.buffers).is_none() {
      let builtin_types = unsafe { &*self.builtin_types };
      result.push(builtin_types.buffer_type);
    }

    if self.use_new_luau_solver() {
      result.reserve(norm.tables.size());
      for &table in norm.tables.order.iter() {
        result.push(table);
      }
    } else {
      result.extend(norm.tables.order.iter().copied());
    }

    for (&tyvar, intersect) in norm.tyvars.iter() {
      if !get_type_id::<NeverType>(intersect.tops).is_none() {
        let ty = self.type_from_normal(intersect);
        result.push(add_intersection(
          self.arena,
          self.builtin_types,
          &[tyvar, ty],
        ));
      } else {
        result.push(tyvar);
      }
    }

    if result.is_empty() {
      let builtin_types = unsafe { &*self.builtin_types };
      builtin_types.never_type
    } else if result.len() == 1 {
      result[0]
    } else {
      unsafe { (*self.arena).add_type(UnionType { options: result }) }
    }
  }
}
