//! Source: `Analysis/src/TypePath.cpp:1107-1141` (hand-ported)
use alloc::vec::Vec;

use crate::{
  enums::pack_field::PackField,
  functions::{end_type_pack::end, get_type_pack::get_type_pack_id},
  records::{
    generic_type_pack::GenericTypePack, path::Path, type_pack::TypePack,
    type_pack_iterator::TypePackIterator,
  },
  type_aliases::{component::Component, type_id::TypeId, type_pack_id::TypePackId},
};
pub fn flatten_pack_with_path(root: TypePackId, path: &Path) -> TypePack {
  let mut flattened: Vec<TypeId> = Vec::new();

  let mut curr: Option<TypePackId> = Some(root);
  let mut path_iter: usize = 0;
  let path_end = path.components.len();

  while let Some(curr_pack) = curr {
    let mut it = TypePackIterator::new();
    it.type_pack_iterator_type_pack_id(curr_pack);

    // Push back curr's head
    while it.operator_ne(&end(curr_pack)) {
      flattened.push(*it.operator_deref());
      it.operator_inc();
    }

    // Check if curr has a tail, and if the next bit of path is Tail +
    // GenericPackMapping
    curr = it.tail();
    let has_generic_tail = match curr {
      Some(tail) => !get_type_pack_id::<GenericTypePack>(tail).is_none(),
      None => false,
    };
    if !has_generic_tail || path_iter == path_end {
      break;
    }

    // const TypePath::PackField* pf = get_if<PackField>(&*pathIter);
    // if (!pf || *pf != Tail) break;
    match path.components.get(path_iter) {
      Some(Component::PackField(pf)) if *pf == PackField::Tail => {}
      _ => break,
    }

    path_iter += 1;

    // const GenericPackMapping* gpm = get_if<GenericPackMapping>(&*pathIter);
    // if (!gpm) break;
    let gpm = match path.components.get(path_iter) {
      Some(Component::GenericPackMapping(gpm)) => *gpm,
      _ => break,
    };

    path_iter += 1;
    curr = Some(gpm.mapped_type);
  }

  TypePack {
    head: flattened,
    tail: curr,
  }
}
