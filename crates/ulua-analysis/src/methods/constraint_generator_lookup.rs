use core::ptr::null;

use ulua_ast::records::location::Location;

use crate::{
  functions::arc_as_mut::arc_as_mut,
  records::{
    arena_handle::alias, blocked_type::BlockedType, cell::Cell,
    constraint_generator::ConstraintGenerator, def_registry::def_as, phi::Phi, type_ids::TypeIds,
  },
  type_aliases::{def_id_def::DefId, scope_ptr_type::ScopePtr, type_id::TypeId},
};
impl ConstraintGenerator {
  pub(crate) fn lookup(
    &mut self,
    scope: &ScopePtr,
    location: Location,
    def: DefId,
    prototype: bool,
  ) -> Option<TypeId> {
    // `def` 是 def arena 的注册句柄，节点探测经安全的 `def_as` 变体下转。
    if def_as::<Cell>(def).is_some() {
      return scope.lookup_def_id(def);
    }

    if let Some(phi) = def_as::<Phi>(def) {
      if let Some(found) = scope.lookup_def_id(def) {
        return Some(found);
      } else if !prototype && phi.operands.len() == 1 {
        return self.lookup(scope, location, phi.operands[0], prototype);
      } else if !prototype {
        return None;
      }

      // Safety: `self.builtin_types.as_ptr()` 对应 C++ `NotNull<BuiltinTypes>`——构造期
      // 接线、非空且比本 ConstraintGenerator 长寿，只读 `Copy` 的 never_type 句柄。
      let mut res = self.builtin_types.get().never_type;

      for operand in &phi.operands {
        let mut ty = self.lookup(scope, location, *operand, /*prototype*/ false);
        if ty.is_none() {
          // Safety: `self.arena.as_ptr()` 为非空长寿 TypeArena，`add_type` 是独占短借用，
          // bump arena 块地址稳定；BlockedType 的 `owner: null()` 本就不被解引用。
          let blocked_ty = {
            self.arena.get_mut().add_type(BlockedType {
              index: 0,
              owner: null(),
            })
          };
          self.local_types.try_insert(blocked_ty, TypeIds::new());
          // `root()` 取出与会话同寿的根 scope（C++ `rootScope`），`alias` 收口
          // 单线程串行下对 lvalueTypes 的一次独占短写，`operand` 是存活 Def 句柄。
          *alias(arc_as_mut(self.root()))
            .lvalue_types
            .get_or_insert(*operand) = blocked_ty;
          ty = Some(blocked_ty);
        }

        res = self.make_union_scope_ptr_location_type_id_type_id(
          arc_as_mut(self.root()),
          location,
          res,
          // Safety: 上方 is_none 分支已补 Some(blocked_ty)，至此必为 Some。
          ty.expect("is_none 分支已补建 Some，至此必为 Some"),
        );
      }

      // `scope` 为入参 `&ScopePtr`，其 Arc 主体在本次调用期间存活；`arc_as_mut`
      // +`alias` 收口单线程串行下的 `&mut` 写 lvalueTypes，与上条 root 写入
      // 时序衔接、不并存（对应 C++ `scope->lvalueTypes[def] = res`）。
      *alias(arc_as_mut(scope)).lvalue_types.get_or_insert(def) = res;
      return Some(res);
    }

    // Safety: `self.ice.as_ptr()` 对应 C++ `NotNull<InternalErrorReporter>`——构造期接线、
    // 非空且长寿；此处仅调用其 ice_string 上报不可达分支（C++ 同句 ice->ice(...)）。
    self
      .ice
      .get()
      .ice_string("ConstraintGenerator::lookup is inexhaustive?");
    None
  }
}
