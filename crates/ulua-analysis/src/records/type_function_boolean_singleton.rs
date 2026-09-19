#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct TypeFunctionBooleanSingleton {
  pub value: bool,
}

unsafe impl Send for TypeFunctionBooleanSingleton {}
unsafe impl Sync for TypeFunctionBooleanSingleton {}
