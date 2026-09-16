use core::ptr::null;

use ulua_ast::records::location::Location;

use crate::{
  functions::get_def::get_def_id,
  records::{
    blocked_type::BlockedType, cell::Cell, constraint_generator::ConstraintGenerator, phi::Phi,
    scope::Scope, type_ids::TypeIds,
  },
  type_aliases::{def_id_def::DefId, scope_ptr_type::ScopePtr, type_id::TypeId},
};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn lookup(
    &mut self,
    scope: &ScopePtr,
    location: Location,
    def: DefId,
    prototype: bool,
  ) -> Option<TypeId> {
    if !unsafe { get_def_id::<Cell>(def) }.is_null() {
      return scope.lookup_def_id(def);
    }

    if let Some(phi) = unsafe { get_def_id::<Phi>(def).as_ref() } {
      if let Some(found) = scope.lookup_def_id(def) {
        return Some(found);
      } else if !prototype && phi.operands.len() == 1 {
        return self.lookup(scope, location, phi.operands[0], prototype);
      } else if !prototype {
        return None;
      }

      let mut res = unsafe { (*self.builtin_types).never_type };

      for operand in &phi.operands {
        let mut ty = self.lookup(scope, location, *operand, /*prototype*/ false);
        if ty.is_none() {
          let blocked_ty = unsafe {
            (*self.arena).add_type(BlockedType {
              index: 0,
              owner: null(),
            })
          };
          self.local_types.try_insert(blocked_ty, TypeIds::new());
          unsafe {
            *(*self.root_scope).lvalue_types.get_or_insert(*operand) = blocked_ty;
          }
          ty = Some(blocked_ty);
        }

        res = self.make_union_scope_ptr_location_type_id_type_id(
          self.root_scope,
          location,
          res,
          ty.unwrap(),
        );
      }

      unsafe {
        let scope_ptr = scope.as_ref() as *const Scope as *mut Scope;
        *(*scope_ptr).lvalue_types.get_or_insert(def) = res;
      }
      return Some(res);
    }

    unsafe { (*self.ice).ice_string("ConstraintGenerator::lookup is inexhaustive?") };
    None
  }
}
