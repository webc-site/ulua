use crate::{
  functions::follow_type_pack::follow_type_pack_id,
  records::type_function_serializer::TypeFunctionSerializer,
  type_aliases::{type_function_type_pack_id::TypeFunctionTypePackId, type_pack_id::TypePackId},
};

impl TypeFunctionSerializer {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn find_type_pack_id(&self, tp: TypePackId) -> Option<TypeFunctionTypePackId> {
    let tp = unsafe { follow_type_pack_id(tp) };
    self.packs.get(&tp).copied()
  }
}
