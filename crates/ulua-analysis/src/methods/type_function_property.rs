//! `type_function_property` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use crate::{
  records::type_function_property::TypeFunctionProperty,
  type_aliases::type_function_type_id::TypeFunctionTypeId,
};

impl TypeFunctionProperty {
  #[inline]
  pub fn is_read_only(&self) -> bool {
    self.read_ty.is_some() && self.write_ty.is_none()
  }
}

impl TypeFunctionProperty {
  #[inline]
  pub fn is_shared(&self) -> bool {
    if let (Some(read_ty), Some(write_ty)) = (self.read_ty, self.write_ty) {
      read_ty == write_ty
    } else {
      false
    }
  }
}

impl TypeFunctionProperty {
  #[inline]
  pub fn is_write_only(&self) -> bool {
    self.write_ty.is_some() && self.read_ty.is_none()
  }
}

impl TypeFunctionProperty {
  #[inline]
  pub fn readonly(ty: TypeFunctionTypeId) -> Self {
    TypeFunctionProperty {
      read_ty: Some(ty),
      write_ty: None,
    }
  }
}

impl TypeFunctionProperty {
  #[inline]
  pub fn writeonly(ty: TypeFunctionTypeId) -> Self {
    TypeFunctionProperty {
      read_ty: None,
      write_ty: Some(ty),
    }
  }
}
