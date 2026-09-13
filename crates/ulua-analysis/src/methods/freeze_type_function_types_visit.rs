//! Node: `cxx:Method:Luau.Analysis:Analysis/src/UserDefinedTypeFunction.cpp:92:FreezeTypeFunctionTypes::visit`
//! Source: `Analysis/src/UserDefinedTypeFunction.cpp:92-96`
//!
//! C++:
//! ```cpp
//! bool visit(TypeFunctionTypeId ty) override
//! {
//!     const_cast<TypeFunctionType*>(ty)->frozen = true;
//!     return true;
//! }
//! ```
//!
//! `FreezeTypeFunctionTypes` overrides the bare `visit(TypeFunctionTypeId)` of
//! `IterativeTypeFunctionTypeVisitor`, which every per-kind `visit(ty, Kind&)`
//! default delegates to (see the base visitor). The traversal-side override is
//! wired onto the base visitor in
//! `crate::records::freeze_type_function_types` (the port models the single
//! subclass by overriding the base's `visit_type_function_type_id`). This
//! method mirrors the C++ override 1:1 so a direct `freezer.visit(ty)` call has
//! the same effect: it const-casts away the `const` of the `TypeFunctionTypeId`
//! (`*const TypeFunctionType`) and sets `frozen = true`.
use crate::{
  records::{
    freeze_type_function_types::FreezeTypeFunctionTypes, type_function_type::TypeFunctionType,
  },
  type_aliases::type_function_type_id::TypeFunctionTypeId,
};

impl FreezeTypeFunctionTypes {
  pub fn visit(&mut self, ty: TypeFunctionTypeId) -> bool {
    // const_cast<TypeFunctionType*>(ty)->frozen = true;
    unsafe {
      (*(ty as *mut TypeFunctionType)).frozen = true;
    }
    true
  }
}
