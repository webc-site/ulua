use core::fmt::{Debug, Formatter, Result};

use crate::enums::ir_const_kind::IrConstKind;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct IrConst {
  pub kind: IrConstKind,
  pub value: IrConstValue,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union IrConstValue {
  pub value_int: i32,
  pub value_int64: i64,
  pub value_uint: u32,
  pub value_double: f64,
  pub value_tag: u8,
}

impl Default for IrConst {
  fn default() -> Self {
    Self {
      kind: IrConstKind::Int,
      value: IrConstValue { value_int: 0 },
    }
  }
}

impl Debug for IrConst {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    let mut s = f.debug_struct("IrConst");
    s.field("kind", &self.kind);
    unsafe {
      match self.kind {
        IrConstKind::Int => s.field("value_int", &self.value.value_int),
        IrConstKind::Int64 => s.field("value_int64", &self.value.value_int64),
        IrConstKind::Uint | IrConstKind::Import => s.field("value_uint", &self.value.value_uint),
        IrConstKind::Double => s.field("value_double", &self.value.value_double),
        IrConstKind::Tag => s.field("value_tag", &self.value.value_tag),
      };
    }
    s.finish()
  }
}

impl Debug for IrConstValue {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("IrConstValue")
      .field("bits", unsafe { &self.value_int64 })
      .finish()
  }
}
