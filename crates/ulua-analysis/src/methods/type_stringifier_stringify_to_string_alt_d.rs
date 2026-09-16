//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:1385:type_stringifier_stringify`
//! Source: `Analysis/src/ToString.cpp:1385-1389` (hand-ported)

use crate::{
  records::{type_pack_stringifier::TypePackStringifier, type_stringifier::TypeStringifier},
  type_aliases::type_pack_id::TypePackId,
};

impl TypeStringifier {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  /// C++ `void TypeStringifier::stringify(TypePackId tp)`.
  pub(crate) fn stringify_type_pack_id(&mut self, tp: TypePackId) {
    let mut tps = TypePackStringifier::type_pack_stringifier_stringifier_state(self.state);
    tps.stringify_type_pack_id(tp);
  }
}
