use alloc::string::String;
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TypeFunctionStringSingleton {
  pub value: String,
}

unsafe impl Send for TypeFunctionStringSingleton {}
unsafe impl Sync for TypeFunctionStringSingleton {}
