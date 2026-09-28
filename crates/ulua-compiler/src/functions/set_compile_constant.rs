use ulua_ast::records::location::Location;

use crate::{
  records::{compile_error::CompileError, constant::Constant},
  type_aliases::compile_constant::CompileConstant,
};

#[inline]
pub fn set_compile_constant_nil(constant: CompileConstant) {
  // Safety: `constant` 由调用方从活的 `&mut Constant`/arena 常量槽上转为 `CompileConstant`，非空且按
  // `Constant` 对齐，回 cast 合法；`Constant::Nil` 是 `Copy` 单元变体，整体覆盖写不读旧值、无 `Drop`。
  unsafe { *constant.cast::<Constant>() = Constant::Nil };
}

#[inline]
pub fn set_compile_constant_boolean(constant: CompileConstant, b: bool) {
  // Safety: 同上，整体覆盖写为 `Constant::Boolean(b)`。
  unsafe { *constant.cast::<Constant>() = Constant::Boolean(b) };
}

#[inline]
pub fn set_compile_constant_number(constant: CompileConstant, n: f64) {
  // Safety: 同上，整体覆盖写为 `Constant::Number(n)`。
  unsafe { *constant.cast::<Constant>() = Constant::Number(n) };
}

#[inline]
pub fn set_compile_constant_vector(constant: CompileConstant, x: f32, y: f32, z: f32, w: f32) {
  // Safety: 同上，整体覆盖写为 `Constant::Vector([x, y, z, w])`。
  unsafe { *constant.cast::<Constant>() = Constant::Vector([x, y, z, w]) };
}

pub fn set_compile_constant_string(constant: CompileConstant, s: *const u8, l: usize) {
  if l > u32::MAX as usize {
    CompileError::raise(
      &Location::default(),
      format_args!("Exceeded custom string constant length limit"),
    );
  }
  // Safety: 同上，整体覆盖写为 `Constant::string(s, l as u32)`。
  unsafe { *constant.cast::<Constant>() = Constant::string(s, l as u32) };
}

/// 以 Rust 原生字节切片设置字符串常量
#[inline]
pub fn set_compile_constant_slice(constant: CompileConstant, s: &[u8]) {
  if s.len() > u32::MAX as usize {
    CompileError::raise(
      &Location::default(),
      format_args!("Exceeded custom string constant length limit"),
    );
  }
  // Safety: 同上，整体覆盖写为 `Constant::string(s.as_ptr(), s.len() as u32)`。
  unsafe { *constant.cast::<Constant>() = Constant::string(s.as_ptr(), s.len() as u32) };
}

/// 以 Rust 原生字符串切片设置字符串常量
#[inline]
pub fn set_compile_constant_str(constant: CompileConstant, s: &str) {
  set_compile_constant_slice(constant, s.as_bytes());
}
