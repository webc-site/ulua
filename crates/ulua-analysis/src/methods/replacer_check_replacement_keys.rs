use crate::{
  functions::{follow_type, follow_type_pack},
  records::replacer::Replacer,
};

impl Replacer {
  pub fn check_replacement_keys(&self) -> bool {
    // Safety: replacements 由 Replacer::new 收下驱动本次 replace 的调用方本地
    // DenseHashMap 地址（如 instantiate 的 `&mut replacements` 实参），非空且
    // 严格比 self 长寿；此处只重建共享借用做只读遍历，方法持 &self，单线程
    // 串行下无在册可变别名。
    let replacements = unsafe { &*self.replacements };
    for (k, _) in replacements.iter() {
      let followed = follow_type::follow(*k);
      if *k != followed {
        return false;
      }
    }

    // Safety: 同上——replacement_packs 与 replacements 同为构造期接线的调用方
    // 本地映射地址，存活覆盖本方法；只读共享借用无别名冲突。
    let replacement_packs = unsafe { &*self.replacement_packs };
    for (k, _) in replacement_packs.iter() {
      let followed = follow_type_pack::follow(*k);
      if *k != followed {
        return false;
      }
    }

    true
  }
}
