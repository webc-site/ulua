use crate::{
  records::type_pack_iterator::TypePackIterator, type_aliases::type_pack_id::TypePackId,
};

pub fn end_type_pack_id(_tp: TypePackId) -> TypePackIterator {
  TypePackIterator::new()
}

pub fn end(tp: TypePackId) -> TypePackIterator {
  end_type_pack_id(tp)
}
