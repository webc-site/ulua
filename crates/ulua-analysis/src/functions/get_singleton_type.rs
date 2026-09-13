//! Source: `Analysis/include/Luau/Type.h:252-259` (hand-ported)
use crate::{
  records::singleton_type::SingletonType, type_aliases::singleton_variant::SingletonVariantMember,
};

/// C++ `template<typename T> const T* get(const SingletonType* stv)`。
/// Rust 形态：引用进、`Option<&T>` 出。
pub fn get_singleton_type<T: SingletonVariantMember>(stv: &SingletonType) -> Option<&T> {
  T::get_if(&stv.variant)
}
