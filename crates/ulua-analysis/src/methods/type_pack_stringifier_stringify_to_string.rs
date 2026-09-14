//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:1187:type_pack_stringifier_stringify`
//! Source: `Analysis/src/ToString.cpp:1187-1191` (hand-ported)

use crate::{
  records::{type_pack_stringifier::TypePackStringifier, type_stringifier::TypeStringifier},
  type_aliases::type_id::TypeId,
};

impl TypePackStringifier {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  /// C++ `void stringify(TypeId tv)`.
  pub unsafe fn stringify_type_id(&mut self, tv: TypeId) {
    let mut tvs = TypeStringifier { state: self.state };
    unsafe { tvs.stringify_type_id(tv) };
  }
}
