use core::ptr::null_mut;

use ulua_ast::records::allocator::Allocator;

use crate::{
  records::{module::Module, type_attacher::TypeAttacher},
  type_aliases::synthetic_names::SyntheticNames,
};
impl TypeAttacher {
  pub fn type_attacher_type_attacher(checker: *mut Module, alloc: *mut Allocator) -> Self {
    Self {
      module: checker,
      allocator: alloc,
      // C++ default-constructs `SyntheticNames synthetic_names;`; the Rust
      // DenseHashMap uses a null-pointer empty-key sentinel.
      synthetic_names: SyntheticNames::new(null_mut()),
    }
  }
}
