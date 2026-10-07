//! Source: `Analysis/include/Luau/Error.h:712-716` (hand-ported)
use crate::{records::type_error::TypeError, type_aliases::type_error_data::TypeErrorDataMember};

/// C++ `template<typename T> const T* get(const TypeError& e)`.
pub fn get_type_error<T: TypeErrorDataMember>(e: &TypeError) -> Option<&T> {
  T::get_if(&e.data)
}
