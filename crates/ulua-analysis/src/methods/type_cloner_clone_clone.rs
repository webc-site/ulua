use crate::{
  records::type_cloner::TypeCloner,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl TypeCloner {
  pub fn clone_type_id(&mut self, ty: TypeId) -> TypeId {
    self.shallow_clone_type_id(ty);
    self.run();
    if self.has_exceeded_iteration_limit() {
      // Safety: builtin_types 是构造期按 C++ 引用成员接线的非空 BuiltinTypes
      // 会话单例，比本 cloner 长寿；error_type 按值读出。
      let error = self.builtin_types.get_mut().error_type;
      // Safety: self.types 指向调用方持有的存活 SeenTypes 表（引用成员语义，
      // 比 cloner 长寿）；ty/error 均为存活句柄值，&mut 再借用止于本次 insert
      // 返回，单线程窗口内无并存别名。
      unsafe { (*self.types).insert(ty, error) };
      return error;
    }
    self
      .find_type_id(ty)
      // Safety: 同上——非空 BuiltinTypes 单例的只读字段拷贝。
      .unwrap_or(self.builtin_types.get_mut().error_type)
  }

  pub fn clone_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    self.shallow_clone_type_pack_id(tp);
    self.run();
    if self.has_exceeded_iteration_limit() {
      // Safety: builtin_types 为构造期接线的非空 BuiltinTypes 单例（引用成员
      // 语义长寿于 cloner），error_type_pack 按值读出。
      let error = self.builtin_types.get_mut().error_type_pack;
      // Safety: self.packs 指向调用方持有的存活 SeenTypePacks 表；tp/error 为
      // 存活句柄值，&mut 再借用止于本次 insert，无并存别名。
      unsafe { (*self.packs).insert(tp, error) };
      return error;
    }
    self
      .find_type_pack_id(tp)
      // Safety: 同 clone_type_id 头——只读拷贝单例字段。
      .unwrap_or(self.builtin_types.get_mut().error_type_pack)
  }
}
