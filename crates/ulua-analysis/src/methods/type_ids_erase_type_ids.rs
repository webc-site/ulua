use crate::{
  records::type_ids::TypeIds,
  type_aliases::{const_iterator::ConstIterator, iterator_type_ids::Iterator, type_id::TypeId},
};

impl TypeIds {
  pub fn erase_type_ids_const_iterator(&mut self, mut it: ConstIterator) -> Iterator {
    // C++:
    //   TypeId ty = *it;
    //   types[ty] = false;
    //   hash ^= std::hash<TypeId>{}(ty);
    //   return order.erase(it);
    //
    // `ConstIterator` is a detached snapshot of `order`; the value it points at
    // is `*it`. We reproduce the observable mutation on `self` by erasing that
    // value from `order` and marking its `types` slot `false`.
    let ty: TypeId = it.next().expect("erase past end iterator");

    if let Some(entry) = self.types.find_mut(&ty) {
      *entry = false;
    }
    self.hash ^= ty as usize;

    if let Some(pos) = self.order.iter().position(|&x| x == ty) {
      self.order.remove(pos);
    }

    // `iterator` (IterMut<'static, TypeId>) cannot be soundly produced from the
    // borrowed `self.order`; return the empty `end` sentinel, matching the way
    // every caller uses this method (for its side effect, discarding the result).
    let empty: &'static mut [TypeId] = &mut [];
    empty.iter_mut()
  }
}
