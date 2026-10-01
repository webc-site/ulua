use core::ptr::null;

use ulua_ast::records::location::Location;

use crate::{
  functions::shared_mut::shared_mut,
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
      }
      if !prototype {
        // 单操作数 phi 直接内联其唯一来源；否则本层无类型可返回。
        return match phi.operands.as_slice() {
          [operand] => self.lookup(scope, location, *operand, prototype),
          _ => None,
        };
      }

      // `self.builtin_types` 为构造期接线的非空 `Handle<BuiltinTypes>`（C++
      // `NotNull<BuiltinTypes>`），比本 ConstraintGenerator 长寿，只读 `Copy` 的 never_type 句柄。
      let mut res = self.builtin_types.get().never_type;

      for operand in &phi.operands {
        let ty = match self.lookup(scope, location, *operand, /*prototype*/ false) {
          Some(ty) => ty,
          None => {
            // `self.arena` 为非空长寿 `Handle<TypeArena>`，`get_mut` 是独占短借用，
            // bump arena 块地址稳定；BlockedType 的 `owner` 裸句柄置空即 cpp
            // `BlockedType{}` 的记录初值，本路径不被解引用。
            let blocked_ty = self.arena.get_mut().add_type(BlockedType {
              index: 0,
              owner: null(),
            });
            self.local_types.try_insert(blocked_ty, TypeIds::new());
            // `root()` 取出与会话同寿的根 scope（C++ `rootScope`），`alias` 收口
            // 单线程串行下对 lvalueTypes 的一次独占短写，`operand` 是存活 Def 句柄。
            *alias(shared_mut(self.root()))
              .lvalue_types
              .get_or_insert(*operand) = blocked_ty;
            blocked_ty
          }
        };

        // 先克隆根句柄再调用（root() 只读借用不可与 &mut self 并存）。
        let root = self.root().clone();
        res = self.make_union_scope_ptr_location_type_id_type_id(&root, location, res, ty);
      }

      // `scope` 为入参 `&ScopePtr`，其 Arc 主体在本次调用期间存活；`shared_mut`
      // +`alias` 收口单线程串行下的 `&mut` 写 lvalueTypes，与上条 root 写入
      // 时序衔接、不并存（对应 C++ `scope->lvalueTypes[def] = res`）。
      *alias(shared_mut(scope)).lvalue_types.get_or_insert(def) = res;
      return Some(res);
    }

    // `self.ice` 为构造期接线的非空 `Handle<InternalErrorReporter>`（C++
    // `NotNull<InternalErrorReporter>`），非空且长寿；此处仅调用其 ice_string
    // 上报不可达分支（C++ 同句 ice->ice(...)）。
    self
      .ice
      .get()
      .ice_string("ConstraintGenerator::lookup is inexhaustive?");
    None
  }
}
