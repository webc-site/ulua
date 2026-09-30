use alloc::vec::Vec;
use core::ptr::write;

use crate::{
  records::type_pack_function::TypePackFunction,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
#[derive(Debug, Clone)]
pub struct TypeFunctionInstanceTypePack {
  pub(crate) function: *const TypePackFunction,
  pub(crate) type_arguments: Vec<TypeId>,
  pub(crate) pack_arguments: Vec<TypePackId>,
}

impl TypeFunctionInstanceTypePack {
  /// `function` 字段的安全读取：指针是实例构造时登记的类型 pack 函数定义句柄
  /// （builtin pack 函数表条目，或既有实例同目标的逐字转发副本——substitution
  /// 克隆是当前唯一构造点，逐字复制指针），先于实例节点存活、覆盖整个分析
  /// 会话且期内只读——与 [`crate::records::type_function_instance_type::
  /// TypeFunctionInstanceType::function`] 同一契约。
  /// 把散布在 reducer/stringifier/rehydration 调用方的逐点 unsafe 收口至此一处。
  pub fn function(&self) -> &TypePackFunction {
    // Safety: 见方法文档；指针由构造从存活的 &TypePackFunction 登记或转发，恒非空对齐。
    unsafe { &*self.function }
  }
}

impl Drop for TypeFunctionInstanceTypePack {
  fn drop(&mut self) {
    // Safety: drop(&mut self) 提供对两字段的独占可写访问且它们均为完好初始化的
    // Vec；ptr::write 直接覆盖存储、不读也不 drop 旧值——元素是 Copy 的 arena
    // 句柄（*const Type/*const TypePackVar），无自有堆资源，不产生第二所有者或
    // 双 drop，旧 Vec 缓冲被刻意弃置；后续 drop 只会见到新写入的空 Vec。
    unsafe {
      write(&mut self.type_arguments, Vec::new());
      write(&mut self.pack_arguments, Vec::new());
    }
  }
}
