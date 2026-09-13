//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:1391:type_stringifier_stringify`
//! Source: `Analysis/src/ToString.cpp:1391-1395` (hand-ported)

use crate::{
  records::{
    function_argument::FunctionArgument, type_pack_stringifier::TypePackStringifier,
    type_stringifier::TypeStringifier,
  },
  type_aliases::type_pack_id::TypePackId,
};

impl TypeStringifier {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  /// C++ `void TypeStringifier::stringify(TypePackId tpid, const std::vector<std::optional<FunctionArgument>>& names)`.
  pub unsafe fn stringify_type_pack_id_vector_optional_function_argument(
    &mut self,
    tpid: TypePackId,
    names: &[Option<FunctionArgument>],
  ) {
    let mut tps = TypePackStringifier::type_pack_stringifier_stringifier_state_vector_optional_function_argument(
            self.state, names,
        );
    tps.stringify_type_pack_id(tpid);
  }
}
