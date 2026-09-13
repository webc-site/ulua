use alloc::boxed::Box;

use ulua_analysis::records::type_function::TypeFunction;

use crate::records::fixture::Fixture;

#[derive(Debug)]
#[repr(C)]
pub struct TypeFunctionFixture {
  pub base: Fixture,
  pub swap_function: Box<TypeFunction>,
}
