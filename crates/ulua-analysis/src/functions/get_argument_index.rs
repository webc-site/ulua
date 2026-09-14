//! Source: `Analysis/src/OverloadResolver.cpp:226-287` (hand-ported)
//!
//! Figuring out which argument a particular path points at can be kind of tricky
//! due to generic pack substitutions.
use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::variant::Variant2};

use crate::{
  enums::pack_field::PackField,
  functions::{begin_type_pack::begin, end_type_pack::end, get_type_alt_j::get_type_id},
  records::{function_type::FunctionType, path::Path},
  type_aliases::{
    component::Component, type_id::TypeId, type_or_pack::TypeOrPack, type_pack_id::TypePackId,
  },
};

pub fn get_argument_index(path: &Path, fn_ty: TypeId) -> Option<usize> {
  let mut iter = path.components.iter();

  let first = iter.next()?;

  if let Component::PackField(args) = first {
    if *args != PackField::Arguments {
      return None;
    }
  } else {
    return None;
  }

  let Some(ft) = get_type_id::<FunctionType>(fn_ty) else {
    LUAU_ASSERT!(!fn_ty.is_null());
    return None;
  };

  let mut result: usize = 0;
  let mut ty: TypeOrPack = Variant2::V1(ft.arg_types);

  for component in iter {
    match component {
      Component::Index(index) => return Some(result + index.index),
      Component::GenericPackMapping(subst) => {
        ty = Variant2::V1(subst.mapped_type);
      }
      Component::PackSlice(slice) => {
        result += slice.start_index;
      }
      Component::PackField(pack_field) if *pack_field == PackField::Tail => {
        // If the path component points at the tail of the pack, we need to
        // advance the count by the length of the current pack.
        let tp: Option<&TypePackId> = ty.get_if_1();
        let tp = match tp {
          Some(tp) => *tp,
          None => {
            LUAU_ASSERT!(false);
            return None;
          }
        };

        // Subtyping flattens out chains of concrete packs when it generates
        // these TypePaths, so we need to do the same here.
        let mut pack_iter = begin(tp);
        let pack_end_iter = end(tp);
        while pack_iter.operator_ne(&pack_end_iter) {
          result += 1;
          pack_iter.operator_inc();
        }

        {
          let tail = pack_iter.tail()?;
          ty = Variant2::V1(tail)
        }

        continue;
      }
      _ => return None,
    }
  }

  None
}
