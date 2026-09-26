//! Source: `Analysis/src/OverloadResolver.cpp:604-643` (hand-ported)
//!
//! A utility function for ::resolveOverload. If a particular overload is a table
//! with a __call metamethod, unwrap that and test it.
//!
//! Note: The __call metamethod can itself be overloaded, but it cannot be a
//! table that overloads __call.  It must be an actual function.
use alloc::vec::Vec;

use ulua_ast::records::location::Location;
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::{
    begin_type::begin_intersection_type, find_metatable_entry::find_metatable_entry, follow_type,
    get_type,
  },
  records::{
    arena_handle::Handle, function_type::FunctionType, intersection_type::IntersectionType,
    overload_resolution::OverloadResolution, overload_resolver::OverloadResolver,
  },
  type_aliases::{error_vec::ErrorVec, type_id::TypeId, type_pack_id::TypePackId},
};
impl OverloadResolver<'_> {
  pub fn test_function_or_call_metamethod(
    &mut self,
    result: &mut OverloadResolution,
    fn_ty: TypeId,
    args_pack: TypePackId,
    fn_location: Location,
    unique_types: *mut DenseHashSet<TypeId>,
  ) {
    let mut fn_ty = follow_type::follow(fn_ty);
    let mut args_pack = args_pack;

    let mut dummy_errors: ErrorVec = Vec::new();
    if let Some(call_metamethod) = find_metatable_entry(
      Handle::from_ptr(self.builtin_types.as_ptr()),
      &mut dummy_errors,
      fn_ty,
      "__call",
      self.call_loc,
    ) {
      // Calling a metamethod forwards `fnTy` as self.
      args_pack = {
        // Safety: self.arena.as_ptr() 由构造期布线为非空（C++ NotNull<AstTypeArena>），指向的 arena
        // 存活于整个类型检查会话；add_type_pack 只在 arena 尾部追加，不移动既有节点。
        self
          .arena
          .get_mut()
          .add_type_pack_vector_type_id_optional_type_pack_id(alloc::vec![fn_ty], Some(args_pack))
      };
      fn_ty = follow_type::follow(call_metamethod);

      // Handle an overloaded __call metamethod.
      if let Some(it) = get_type::get::<IntersectionType>(fn_ty) {
        // C++ `for (TypeId component : it)`——IntersectionTypeIterator 展平
        // 嵌套 intersection 并 follow,裸遍历 parts 会漏掉嵌套成员。
        for component in begin_intersection_type(it) {
          let component = follow_type::follow(component);
          result.metamethods.insert(component);

          if let Some(fn_ref) = get_type::get::<FunctionType>(component)
            && !self.is_arity_compatible(args_pack, fn_ref.arg_types, self.builtin_types)
          {
            result.arity_mismatches.push(component);
          } else {
            self.test_function(result, component, args_pack, fn_location, unique_types);
          }
        }
        return;
      }

      result.metamethods.insert(fn_ty);
    }

    // Handle non-metamethods and metamethods which aren't overloaded.
    self.test_function(result, fn_ty, args_pack, fn_location, unique_types);
  }
}
