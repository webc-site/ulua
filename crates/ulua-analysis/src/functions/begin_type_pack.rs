use crate::{
  records::type_pack_iterator::TypePackIterator, type_aliases::type_pack_id::TypePackId,
};

pub fn begin(tp: TypePackId) -> TypePackIterator {
  let mut it = TypePackIterator::new();
  it.type_pack_iterator_type_pack_id(tp);
  it
}

pub use begin as begin_type_pack_id;
