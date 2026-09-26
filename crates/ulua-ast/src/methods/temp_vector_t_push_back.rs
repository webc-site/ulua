use crate::records::temp_vector::TempVector;

impl<'a, T> TempVector<'a, T> {
  pub fn push_back(&mut self, item: T) {
    let storage = unsafe {
      // Safety: storage 由 TempVector::new 从 &mut Vec<T> 取得，非空、对齐且 'a 内存活（PhantomData<&'a mut T> 记录独占借用）；此视图是该区间的唯一持有者（cpp TempVector 同款单写者模型），&mut 重借用归还独占权后 push 并维持不变式 size_==len-offset，单线程串行无别名冲突。
      &mut *self.storage
    };
    ulua_common::LUAU_ASSERT!(storage.len() == self.offset + self.size_);
    storage.push(item);
    self.size_ += 1;
  }
}
