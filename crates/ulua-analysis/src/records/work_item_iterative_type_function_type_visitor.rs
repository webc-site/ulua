use crate::type_aliases::{
  type_function_type_id::TypeFunctionTypeId, type_function_type_pack_id::TypeFunctionTypePackId,
};
#[derive(Debug, Clone)]
pub struct WorkItem {
  pub(crate) t: *const (),
  pub(crate) is_type: bool,
  pub(crate) parent: i32,
}

impl WorkItem {
  pub fn work_item_type_function_type_id_i32(ty: TypeFunctionTypeId, parent: i32) -> Self {
    Self {
      t: ty as *const (),
      is_type: true,
      parent,
    }
  }
  pub fn work_item_type_function_type_pack_id_i32(tp: TypeFunctionTypePackId, parent: i32) -> Self {
    Self {
      t: tp as *const (),
      is_type: false,
      parent,
    }
  }

  /// 按 `is_type` 判别取出 `TypeFunctionTypeId`，替代返回自引用指针的
  /// `as_type`（调用侧不再需要 unsafe 解引用）。
  pub fn type_function_type_id(&self) -> Option<TypeFunctionTypeId> {
    if self.is_type {
      Some(self.t as TypeFunctionTypeId)
    } else {
      None
    }
  }

  pub fn type_function_type_pack_id(&self) -> Option<TypeFunctionTypePackId> {
    if self.is_type {
      None
    } else {
      Some(self.t as TypeFunctionTypePackId)
    }
  }
}
